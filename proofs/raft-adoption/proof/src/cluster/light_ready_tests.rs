#[test]
#[ignore = "Q2 real-disk LightReady tier; execute explicitly"]
fn q2_real_light_ready_commit_is_durable_before_apply_and_failure_returns_no_messages() {
    use glade_raft_adoption_api::{Action, Command, Outcome, Receipt, RequestId, Resource};
    use glade_raft_disk::DiskStore;
    use glade_raft_durability_api::{Binding, DurableImage, DurableStore, StoreError, StoredState};
    use raft::eraftpb::MessageType;
    use std::cell::{Cell, RefCell};
    use std::collections::BTreeMap;
    use std::rc::Rc;

    // Test-only lifecycle guard: raft-rs ready() forbids state changes until the
    // Ready is returned through an advance family. Each controlled leader mutation
    // below must pass this guard, independently of Raft's internal behavior.
    #[derive(Default)]
    struct ReadyDiscipline {
        outstanding: bool,
    }

    impl ReadyDiscipline {
        fn take_ready(&mut self) {
            assert!(!self.outstanding, "previous Ready must be advanced");
            self.outstanding = true;
        }

        fn advanced(&mut self) {
            assert!(self.outstanding, "advance requires a collected Ready");
            self.outstanding = false;
        }

        fn allow_mutation(&self, _operation: &str) -> Result<(), &'static str> {
            if self.outstanding {
                return Err("state mutation while Ready outstanding");
            }
            Ok(())
        }
    }

    let mut guard = ReadyDiscipline::default();
    guard.take_ready();
    for operation in ["step", "propose", "campaign"] {
        assert_eq!(
            guard.allow_mutation(operation),
            Err("state mutation while Ready outstanding")
        );
    }
    guard.advanced();
    for operation in ["step", "propose", "campaign"] {
        assert_eq!(guard.allow_mutation(operation), Ok(()));
    }

    struct Gate {
        inner: DiskStore,
        armed: Rc<Cell<bool>>,
        commits: Rc<RefCell<Vec<DurableImage>>>,
    }
    impl DurableStore for Gate {
        fn load(&mut self) -> Result<StoredState, StoreError> {
            self.inner.load()
        }
        fn persist(
            &mut self,
            revision: u64,
            image: DurableImage,
        ) -> Result<StoredState, StoreError> {
            let prior = self.inner.load()?;
            if image.commit > prior.image.commit && image.entries == prior.image.entries {
                self.commits.borrow_mut().push(image.clone());
                if self.armed.get() {
                    return Err(StoreError::Io);
                }
            }
            self.inner.persist(revision, image)
        }
    }
    for fail in [false, true] {
        let directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!("q2-light-{}-{fail}", std::process::id()));
        std::fs::create_dir(&directory).unwrap();
        let armed = Rc::new(Cell::new(false));
        let commits = Rc::new(RefCell::new(Vec::new()));
        let mut stores: BTreeMap<u64, Box<dyn DurableStore>> = BTreeMap::new();
        for node in 1..=3 {
            let inner = DiskStore::create_new(
                &directory.join(format!("node-{node}")),
                Binding {
                    scope: 7,
                    node,
                    voters: vec![1, 2, 3],
                    application_profile: 1,
                },
            )
            .unwrap();
            if node == 1 {
                stores.insert(
                    node,
                    Box::new(Gate {
                        inner,
                        armed: armed.clone(),
                        commits: commits.clone(),
                    }),
                );
            } else {
                stores.insert(node, Box::new(inner));
            }
        }
        let mut discipline = ReadyDiscipline::default();
        let mut cluster = super::Cluster::recover(&[1, 2, 3], stores).unwrap();
        discipline.allow_mutation("campaign").unwrap();
        cluster.campaign(1);
        cluster.drain();
        let create = Command {
            request: RequestId {
                scope: 7,
                resource: 100,
                incarnation: 1,
                principal: 1,
                sequence: 1,
            },
            generation: 0,
            home: 0,
            policy_frontier: 0,
            action: Action::Create {
                name: 40,
                home: 1,
                payload: 11,
            },
        };
        discipline.allow_mutation("propose").unwrap();
        cluster.propose(1, create);
        cluster.drain();
        let cut = cluster.applied(1);
        let mutation = Command {
            request: RequestId {
                sequence: 2,
                ..create.request
            },
            generation: 1,
            home: 1,
            action: Action::Mutate { payload: 23 },
            ..create
        };
        discipline.allow_mutation("propose").unwrap();
        cluster.propose(1, mutation);
        let leader = cluster.voters.get_mut(&1).unwrap();
        assert!(leader.raft.has_ready());
        discipline.take_ready();
        let ready = leader.raft.ready();
        assert!(!ready.entries().is_empty());
        assert!(ready.hs().is_none_or(|state| state.commit == cut));
        // RawNode::ready contract: no step/propose/campaign until this Ready
        // is returned. Persist the full image/memory mirror first, then return
        // through advance_append_async while delaying the persistence notice.
        // raw_node.rs:471-475, 689-698 (raft 0.7.0).
        assert!(ready.committed_entries().is_empty());
        leader.persist_ready(ready.entries(), ready.hs()).unwrap();
        let messages = ready
            .messages()
            .iter()
            .chain(ready.persisted_messages())
            .filter(|message| message.to == 2 && message.get_msg_type() == MessageType::MsgAppend)
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(
            messages.len(),
            1,
            "one persisted follower data ack is enough once local persistence is notified"
        );
        let before = leader.persistence.as_ref().unwrap().1.image.clone();
        assert_eq!(before.commit, cut);
        assert_eq!(before.entries.last().unwrap().index, cut + 1);
        leader.raft.advance_append_async(ready);
        discipline.advanced();
        for message in messages {
            let follower = cluster.voters.get_mut(&message.to).unwrap();
            discipline.allow_mutation("follower step").unwrap();
            follower.raft.step(message).unwrap();
            let responses = follower.ready().unwrap();
            for response in responses {
                assert_eq!(response.to, 1);
                assert_eq!(response.get_msg_type(), MessageType::MsgAppendResponse);
                discipline.allow_mutation("step").unwrap();
                cluster
                    .voters
                    .get_mut(&1)
                    .unwrap()
                    .raft
                    .step(response)
                    .unwrap();
            }
        }
        let leader = cluster.voters.get_mut(&1).unwrap();
        // The delayed local persistence notification keeps local match at cut;
        // only follower 2 has acknowledged cut+1, so no quorum commit yet.
        assert_eq!(leader.raft.raft.hard_state().commit, cut);
        discipline.allow_mutation("ping").unwrap();
        leader.raft.ping();
        assert!(leader.raft.has_ready());
        discipline.take_ready();
        let ready = leader.raft.ready();
        assert!(ready.entries().is_empty());
        assert!(ready.hs().is_none_or(|state| state.commit == cut));
        // Production finish_ready -> advance_append -> on_persist_ready drains
        // both Ready records. Local match catches up, yielding a genuinely new
        // LightReady commit. raw_node.rs:617-645,669-684; raft.rs:1033-1058.
        armed.set(fail);
        commits.borrow_mut().clear();
        let leader = cluster.voters.get_mut(&1).unwrap();
        let result = leader.finish_ready(ready);
        discipline.advanced();
        let images = commits.borrow();
        assert_eq!(
            images.len(),
            1,
            "actual LightReady commit-only port must run"
        );
        assert_eq!(
            (images[0].term, images[0].vote, images[0].commit),
            (before.term, before.vote, cut + 1)
        );
        let expected = Receipt {
            request: mutation.request,
            index: cut + 1,
            outcome: Outcome::Accepted(Resource {
                id: 100,
                name: 40,
                incarnation: 1,
                generation: 1,
                home: 1,
                payload: 23,
                retired: false,
            }),
        };
        if fail {
            assert_eq!(result, Err(StoreError::Io));
            assert_eq!(leader.application.applied(), cut);
            assert!(leader.application.reply(mutation).is_none());
            assert_eq!(leader.persistence.as_ref().unwrap().1.image.commit, cut);
        } else {
            assert!(result.is_ok());
            assert_eq!(leader.application.applied(), cut + 1);
            assert_eq!(leader.application.reply(mutation), Some(expected));
            assert_eq!(leader.persistence.as_ref().unwrap().1.image.commit, cut + 1);
        }
        drop(images);
        drop(cluster);
        if !fail {
            let stores = (1..=3)
                .map(|node| {
                    let store = DiskStore::open(
                        &directory.join(format!("node-{node}")),
                        Binding {
                            scope: 7,
                            node,
                            voters: vec![1, 2, 3],
                            application_profile: 1,
                        },
                        None,
                    )
                    .unwrap();
                    (node, Box::new(store) as Box<dyn DurableStore>)
                })
                .collect();
            let recovered = super::Cluster::recover(&[1, 2, 3], stores).unwrap();
            assert_eq!(recovered.outcome(1, mutation.request), Some(expected));
            assert_eq!(
                recovered.resource(1, 100),
                match expected.outcome {
                    Outcome::Accepted(resource) => {
                        Some(resource)
                    }
                    Outcome::Rejected(_) => {
                        unreachable!("fixture expects complete accepted Resource")
                    }
                }
            );
            drop(recovered);
        }
        std::fs::remove_dir_all(directory).unwrap();
    }
}
