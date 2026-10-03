#[test]
fn actual_light_ready_commit_all_five_publication_faults_gate_apply_and_messages() {
    use super::*;
    use glade_raft_disk::{
        FaultPoint,
        v2::{V2DiskStore, V2StoreFactory},
    };
    use glade_raft_q3_api::{CheckpointStore, Image, StoreLifecycle, conformance};
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    struct Gate {
        inner: V2DiskStore,
        armed: Rc<Cell<Option<FaultPoint>>>,
        commits: Rc<RefCell<Vec<Image>>>,
    }
    impl CheckpointStore for Gate {
        fn load(&mut self) -> Result<State, Error> {
            self.inner.load()
        }
        fn publish(&mut self, revision: u64, image: Image) -> Result<State, Error> {
            let prior = self.inner.load()?;
            if image.commit > prior.image.commit && image.suffix == prior.image.suffix {
                self.commits.borrow_mut().push(image.clone());
                if let Some(fault) = self.armed.take() {
                    self.inner.inject_once(fault);
                }
            }
            self.inner.publish(revision, image)
        }
    }
    for (case, fault) in [
        None,
        Some(FaultPoint::BeforeWrite),
        Some(FaultPoint::PartialWrite),
        Some(FaultPoint::AfterWrite),
        Some(FaultPoint::BeforeSync),
        Some(FaultPoint::AfterSync),
    ]
    .into_iter()
    .enumerate()
    {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!("q3-light-{}-{case}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let armed = Rc::new(Cell::new(None));
        let commits = Rc::new(RefCell::new(Vec::new()));
        let mut factory = V2StoreFactory::new(root.clone());
        let mut stores: BTreeMap<u64, Box<dyn CheckpointStore>> = BTreeMap::new();
        for id in [1, 2, 3] {
            if id == 1 {
                let inner =
                    V2DiskStore::create_new(&root.join("node-1"), instance(1), recovery::initial())
                        .unwrap();
                stores.insert(
                    id,
                    Box::new(Gate {
                        inner,
                        armed: armed.clone(),
                        commits: commits.clone(),
                    }),
                );
            } else {
                stores.insert(
                    id,
                    factory.create(instance(id), recovery::initial()).unwrap(),
                );
            }
        }
        let mut session = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
        conformance::accepted_create(&mut session);
        let cut = session.nodes[&1].machine.applied();
        let create = conformance::create();
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
        let expected = Receipt {
            request: mutation.request,
            index: cut + 1,
            outcome: glade_raft_q3_api::Outcome::Accepted(glade_raft_q3_api::Resource {
                payload: 23,
                ..conformance::created_resource(1)
            }),
        };
        session.propose(mutation).unwrap();
        let leader = session.nodes.get_mut(&1).unwrap();
        let ready = leader.raft.ready();
        assert!(!ready.entries().is_empty());
        assert!(ready.committed_entries().is_empty());
        leader
            .persist_ready(ready.entries(), ready.hs(), ready.snapshot())
            .unwrap();
        let original = leader.state.image.suffix.last().unwrap().clone();
        let messages: Vec<_> = ready
            .messages()
            .iter()
            .chain(ready.persisted_messages())
            .filter(|m| m.to == 2 && m.get_msg_type() == MessageType::MsgAppend)
            .cloned()
            .collect();
        assert_eq!(messages.len(), 1);
        let before = leader.state.clone();
        leader.raft.advance_append_async(ready);
        for message in messages {
            let follower = session.nodes.get_mut(&2).unwrap();
            follower.raft.step(message).unwrap();
            for response in follower.ready().unwrap() {
                assert_eq!(response.get_msg_type(), MessageType::MsgAppendResponse);
                session
                    .nodes
                    .get_mut(&1)
                    .unwrap()
                    .raft
                    .step(response)
                    .unwrap();
            }
        }
        let leader = session.nodes.get_mut(&1).unwrap();
        assert_eq!(leader.raft.raft.hard_state().commit, cut);
        leader.raft.ping();
        let ready = leader.raft.ready();
        assert!(ready.entries().is_empty());
        assert!(ready.hs().is_none_or(|hs| hs.commit == cut));
        commits.borrow_mut().clear();
        armed.set(fault);
        let result = leader.finish(ready);
        assert_eq!(
            commits.borrow().len(),
            1,
            "genuine commit-only LightReady publication"
        );
        assert_eq!(commits.borrow()[0].commit, cut + 1);
        if fault.is_some() {
            assert_eq!(result, Err(Error::IoUnknown));
            assert_eq!(leader.machine.applied(), cut);
            assert!(leader.machine.application.reply(mutation).is_none());
            assert_eq!(leader.state, before);
        } else {
            assert!(result.is_ok());
            assert_eq!(leader.machine.application.reply(mutation), Some(expected));
            assert_eq!(leader.machine.history[&(cut + 1)].0, original);
        }
        drop(session);
        match V2DiskStore::open(&root.join("node-1"), instance(1), None) {
            Ok(mut reopened) => {
                let state = reopened.load().unwrap();
                let restored = recovery::restore(&state, &instance(1)).unwrap();
                assert_eq!(
                    state.image.commit,
                    if fault == Some(FaultPoint::BeforeWrite) {
                        cut
                    } else {
                        cut + 1
                    }
                );
                if state.image.commit > cut {
                    assert_eq!(restored.application.reply(mutation), Some(expected));
                    assert_eq!(restored.history[&(cut + 1)].0, original);
                }
            }
            Err(error) => {
                assert_eq!(fault, Some(FaultPoint::PartialWrite));
                assert_eq!(error, Error::Quarantined);
            }
        }
        std::fs::remove_dir_all(root).unwrap();
    }
}
