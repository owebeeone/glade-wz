#[test]
fn direct_install_and_catchup_all_five_faults_stop_service_until_physical_reopen() {
    use super::*;
    use glade_raft_disk::{
        FaultPoint,
        v2::{V2DiskStore, V2StoreFactory},
    };
    use glade_raft_q3_api::{ConfigOutcome, QualificationSession, StoreLifecycle, conformance};
    use raft::eraftpb::Message;
    use std::cell::Cell;
    use std::rc::Rc;
    struct Gate {
        inner: V2DiskStore,
        armed: Rc<Cell<Option<FaultPoint>>>,
    }
    impl CheckpointStore for Gate {
        fn load(&mut self) -> Result<State, Error> {
            self.inner.load()
        }
        fn publish(
            &mut self,
            revision: u64,
            image: glade_raft_q3_api::Image,
        ) -> Result<State, Error> {
            if let Some(fault) = self.armed.take() {
                self.inner.inject_once(fault);
            }
            self.inner.publish(revision, image)
        }
    }
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    for catchup in [false, true] {
        for (case, fault) in [
            FaultPoint::PartialWrite,
            FaultPoint::BeforeWrite,
            FaultPoint::AfterWrite,
            FaultPoint::BeforeSync,
            FaultPoint::AfterSync,
        ]
        .into_iter()
        .enumerate()
        {
            let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../target")
                .join(format!(
                    "q3-rem-install-{}-{catchup}-{case}",
                    std::process::id()
                ));
            std::fs::create_dir(&root).unwrap();
            let directory = Directory(root.clone());
            let armed = Rc::new(Cell::new(None));
            let mut factory = V2StoreFactory::new(root.clone());
            let mut stores: BTreeMap<u64, Box<dyn CheckpointStore>> = BTreeMap::new();
            for id in [1, 2, 3] {
                if id == 1 {
                    let inner = V2DiskStore::create_new(
                        &root.join("node-1"),
                        instance(1),
                        recovery::initial(),
                    )
                    .unwrap();
                    stores.insert(
                        id,
                        Box::new(Gate {
                            inner,
                            armed: armed.clone(),
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
            let original = conformance::accepted_create(&mut session);
            if catchup {
                let cp = session.checkpoint().unwrap();
                session.install(cp).unwrap();
            }
            let joined = session
                .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
                .unwrap()
                .unwrap();
            assert_eq!(joined.outcome, ConfigOutcome::Accepted);
            let original_entry = session.applied_entry(original.index).unwrap();
            let config_entry = session.applied_entry(joined.index).unwrap();
            let prior = session.nodes[&1].state.clone();
            let prior_file = std::fs::read(root.join("node-1")).unwrap();
            let checkpoint = session.checkpoint().unwrap();
            armed.set(Some(fault));
            let result = if catchup {
                session.control(Control::CatchUp { node: 4 })
            } else {
                session.install(checkpoint.clone())
            };
            assert_eq!(result, Err(Error::IoUnknown));
            assert_eq!(
                session.outcome(original.request),
                Ok(None),
                "failed leader must not release even a retained original receipt"
            );
            assert_eq!(session.submit(conformance::create()), Ok(None));
            assert_eq!(
                session.configuration_outcome(joined.intent.key),
                Err(Error::NoQuorum)
            );
            assert_eq!(session.configure(joined.intent.clone()), Ok(None));
            assert_eq!(session.resource(100), Err(Error::NoQuorum));
            assert_eq!(session.nodes[&1].failure, Some(Error::IoUnknown));
            assert_eq!(session.nodes[&1].state, prior);
            if catchup {
                assert_eq!(session.nodes[&4].machine.applied(), 0);
            }
            // A previously queued outgoing envelope from the stopped voter must
            // not make another voter change term or generate a response.
            let follower_before = session.nodes[&2].state.clone();
            let mut message = Message::default();
            message.set_msg_type(MessageType::MsgHeartbeat);
            message.from = 1;
            message.to = 2;
            message.term = prior.image.term + 1;
            session.messages.push_back(message);
            session.drain().unwrap();
            assert_eq!(session.nodes[&2].state, follower_before);
            assert!(session.messages.is_empty());
            drop(session);
            let ids = if catchup {
                vec![1, 2, 3, 4]
            } else {
                vec![1, 2, 3]
            };
            let mut factory = V2StoreFactory::new(root.clone());
            let mut stores = BTreeMap::new();
            let mut poisoned = false;
            for id in ids {
                match factory.open(instance(id), None) {
                    Ok(mut store) => {
                        let state = store.load().unwrap();
                        if id == 1 {
                            if fault == FaultPoint::BeforeWrite {
                                assert_eq!(state, prior);
                                assert_eq!(std::fs::read(root.join("node-1")).unwrap(), prior_file);
                            } else {
                                assert_eq!(state.image.checkpoint, Some(checkpoint.clone()));
                                assert_eq!(state.image.applied, prior.image.applied);
                                assert_eq!(state.image.configuration, prior.image.configuration);
                            }
                            let machine = recovery::restore(&state, &instance(id)).unwrap();
                            assert_eq!(
                                machine.history[&original.index],
                                (original_entry.clone(), ReplayResult::Application(original))
                            );
                            assert_eq!(
                                machine.history[&joined.index],
                                (
                                    config_entry.clone(),
                                    ReplayResult::Configuration(joined.clone())
                                )
                            );
                        }
                        stores.insert(id, store);
                    }
                    Err(error) => {
                        assert_eq!(id, 1);
                        assert_eq!(fault, FaultPoint::PartialWrite);
                        assert_eq!(error, Error::Quarantined);
                        poisoned = true;
                    }
                }
            }
            if !poisoned {
                let mut recovered =
                    Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
                assert_eq!(recovered.outcome(original.request), Ok(Some(original)));
                assert_eq!(recovered.submit(conformance::create()), Ok(Some(original)));
                assert_eq!(
                    recovered.configure(joined.intent.clone()),
                    Ok(Some(joined.clone()))
                );
                assert_eq!(
                    recovered.replay(original_entry),
                    Ok(ReplayResult::Application(original))
                );
                assert_eq!(
                    recovered.replay(config_entry),
                    Ok(ReplayResult::Configuration(joined.clone()))
                );
                if catchup {
                    recovered.control(Control::CatchUp { node: 4 }).unwrap();
                    let view = recovered.view().unwrap();
                    let target = view.nodes.iter().find(|node| node.node == 4).unwrap();
                    assert!(target.snapshot_index.is_some_and(|cut| cut >= joined.index));
                    assert_eq!(target.applied, view.committed);
                }
                drop(recovered);
            } else {
                drop(stores);
            }
            drop(directory);
        }
    }
}

#[test]
fn actual_publication_with_malformed_success_stops_until_physical_reopen() {
    use super::*;
    use glade_raft_disk::v2::{V2DiskStore, V2StoreFactory};
    use glade_raft_q3_api::{QualificationSession, StoreLifecycle, conformance};
    use std::cell::Cell;
    use std::rc::Rc;
    struct Gate {
        inner: V2DiskStore,
        armed: Rc<Cell<Option<usize>>>,
    }
    impl CheckpointStore for Gate {
        fn load(&mut self) -> Result<State, Error> {
            self.inner.load()
        }
        fn publish(
            &mut self,
            revision: u64,
            image: glade_raft_q3_api::Image,
        ) -> Result<State, Error> {
            let case = self.armed.take();
            // The fourth adversary isolates checked host-revision overflow
            // AFTER genuine publication. The actual V2 revision never wraps.
            let revision = if case == Some(3) {
                self.inner.load()?.revision
            } else {
                revision
            };
            let mut state = self.inner.publish(revision, image)?;
            match case {
                Some(0) => {
                    state.instance.group = 71;
                }
                Some(1) => {
                    state.revision += 1;
                }
                Some(2) => {
                    state.image.checkpoint = None;
                }
                _ => {}
            }
            Ok(state)
        }
    }
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    for case in 0..4 {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!(
                "q3-rem-malformed-success-{}-{case}",
                std::process::id()
            ));
        std::fs::create_dir(&root).unwrap();
        let directory = Directory(root.clone());
        let armed = Rc::new(Cell::new(None));
        let mut factory = V2StoreFactory::new(root.clone());
        let mut stores: BTreeMap<u64, Box<dyn CheckpointStore>> = BTreeMap::new();
        for id in [1, 2, 3] {
            if id == 1 {
                stores.insert(
                    id,
                    Box::new(Gate {
                        inner: V2DiskStore::create_new(
                            &root.join("node-1"),
                            instance(1),
                            recovery::initial(),
                        )
                        .unwrap(),
                        armed: armed.clone(),
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
        let original = conformance::accepted_create(&mut session);
        let original_entry = session.applied_entry(original.index).unwrap();
        let checkpoint = session.checkpoint().unwrap();
        let actual_prior_revision = session.nodes[&1].state.revision;
        if case == 3 {
            session.nodes.get_mut(&1).unwrap().state.revision = u64::MAX;
        }
        let cached_prior = session.nodes[&1].state.clone();
        armed.set(Some(case));
        let error = if case == 3 {
            Error::CapacityExhausted
        } else {
            Error::Quarantined
        };
        assert_eq!(session.install(checkpoint.clone()), Err(error.clone()));
        assert_eq!(
            session.outcome(original.request),
            Ok(None),
            "malformed successful publication must stop retained memory replies"
        );
        assert_eq!(session.submit(conformance::create()), Ok(None));
        assert_eq!(session.resource(100), Err(Error::NoQuorum));
        assert_eq!(session.nodes[&1].failure, Some(error));
        assert_eq!(session.nodes[&1].state, cached_prior);
        let follower_before = session.nodes[&2].state.clone();
        let mut message = raft::eraftpb::Message::default();
        message.set_msg_type(MessageType::MsgHeartbeat);
        message.from = 1;
        message.to = 2;
        message.term = cached_prior.image.term + 1;
        session.messages.push_back(message);
        session.drain().unwrap();
        assert_eq!(session.nodes[&2].state, follower_before);
        assert!(session.messages.is_empty());
        drop(session);
        let mut factory = V2StoreFactory::new(root.clone());
        let mut stores = BTreeMap::new();
        for id in [1, 2, 3] {
            let mut store = factory.open(instance(id), None).unwrap();
            let state = store.load().unwrap();
            if id == 1 {
                assert_eq!(state.instance, instance(1));
                assert_eq!(state.revision, actual_prior_revision + 1);
                assert_eq!(state.image.checkpoint, Some(checkpoint.clone()));
                let machine = recovery::restore(&state, &instance(id)).unwrap();
                assert_eq!(
                    machine.history[&original.index],
                    (original_entry.clone(), ReplayResult::Application(original))
                );
            }
            stores.insert(id, store);
        }
        let mut recovered = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
        assert_eq!(recovered.outcome(original.request), Ok(Some(original)));
        assert_eq!(recovered.submit(conformance::create()), Ok(Some(original)));
        assert_eq!(
            recovered.replay(original_entry),
            Ok(ReplayResult::Application(original))
        );
        drop(recovered);
        drop(directory);
    }
}
