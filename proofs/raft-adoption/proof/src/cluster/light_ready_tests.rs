#[test]
#[ignore = "Q2 real-disk LightReady tier; execute explicitly"]
fn q2_real_light_ready_commit_is_durable_before_apply_and_failure_returns_no_messages() {
    use glade_raft_adoption_api::{Action, Command, RequestId};
    use glade_raft_disk::DiskStore;
    use glade_raft_durability_api::{Binding, DurableImage, DurableStore, StoreError, StoredState};
    use std::cell::{Cell, RefCell};
    use std::collections::BTreeMap;
    use std::rc::Rc;

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
        let mut cluster = super::Cluster::recover(&[1, 2, 3], stores).unwrap();
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
        cluster.propose(1, mutation);
        let leader = cluster.voters.get_mut(&1).unwrap();
        let ready = leader.raft.ready();
        assert!(!ready.entries().is_empty());
        assert!(ready.hs().is_none_or(|state| state.commit == cut));
        // Legal alternate host schedule: persist, release appends, deliver data
        // quorum responses, then advance append. This produces real LightReady.
        leader.persist_ready(ready.entries(), ready.hs()).unwrap();
        let messages = ready
            .messages()
            .iter()
            .chain(ready.persisted_messages())
            .cloned()
            .collect::<Vec<_>>();
        let before = leader.persistence.as_ref().unwrap().1.image.clone();
        assert_eq!(before.commit, cut);
        assert_eq!(before.entries.last().unwrap().index, cut + 1);
        for message in messages {
            let follower = cluster.voters.get_mut(&message.to).unwrap();
            follower.raft.step(message).unwrap();
            let responses = follower.ready().unwrap();
            for response in responses {
                assert_eq!(response.to, 1);
                cluster
                    .voters
                    .get_mut(&1)
                    .unwrap()
                    .raft
                    .step(response)
                    .unwrap();
            }
        }
        armed.set(fail);
        commits.borrow_mut().clear();
        let leader = cluster.voters.get_mut(&1).unwrap();
        let result = leader.finish_ready(ready);
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
        if fail {
            assert_eq!(result, Err(StoreError::Io));
            assert_eq!(leader.application.applied(), cut);
            assert!(leader.application.reply(mutation).is_none());
            assert_eq!(leader.persistence.as_ref().unwrap().1.image.commit, cut);
        } else {
            assert!(result.is_ok());
            assert_eq!(leader.application.applied(), cut + 1);
            assert!(leader.application.reply(mutation).is_some());
            assert_eq!(leader.persistence.as_ref().unwrap().1.image.commit, cut + 1);
        }
        drop(images);
        drop(cluster);
        std::fs::remove_dir_all(directory).unwrap();
    }
}
