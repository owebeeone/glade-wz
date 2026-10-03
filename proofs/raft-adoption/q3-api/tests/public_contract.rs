use glade_raft_q3_api::{CheckpointStore, Error, QualificationSession, ReplayResult, StoredEntry};

#[test]
fn dyn_consumers_compile_without_carrier_or_disk_types() {
    fn store_consumer(store: &mut dyn CheckpointStore) {
        let _result = store.load();
    }
    fn session_consumer(session: &mut dyn QualificationSession) {
        let _result = session.view();
    }
    fn replay_consumer(
        session: &mut dyn QualificationSession,
        entry: StoredEntry,
    ) -> Result<u64, Error> {
        // Exhaustive compiler consumer distinguishes complete original types;
        // configuration outcome is not folded into an absent application reply.
        match session.replay(entry)? {
            ReplayResult::Application(receipt) => {
                let _complete_request = receipt.request;
                let _complete_outcome = receipt.outcome;
                Ok(receipt.index)
            }
            ReplayResult::Configuration(receipt) => {
                let _original_intent = receipt.intent;
                let _original_accepted_or_refused = receipt.outcome;
                let _original_configuration = receipt.configuration;
                Ok(receipt.index)
            }
            ReplayResult::Noop { index } => Ok(index),
        }
    }
    // These concrete function-pointer types force both dyn consumer signatures
    // through compilation. No behavioral provider is claimed by this witness.
    let _store: fn(&mut dyn CheckpointStore) = store_consumer;
    let _session: fn(&mut dyn QualificationSession) = session_consumer;
    let _replay: fn(&mut dyn QualificationSession, StoredEntry) -> Result<u64, Error> =
        replay_consumer;
}

#[test]
fn create_fixture_has_zero_preconditions_and_complete_selected_home() {
    use glade_raft_q3_api::{Action, Command, RequestId, conformance};
    assert_eq!(
        conformance::create(),
        Command {
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
                payload: 11
            },
        }
    );
}

#[test]
fn injected_lifecycle_consumer_compiles_without_paths_or_carrier() {
    use glade_raft_q3_api::{Image, Instance, StoreLifecycle};
    fn consumer(factory: &mut dyn StoreLifecycle, instance: Instance, image: Image) {
        let _created: Result<Box<dyn CheckpointStore>, Error> =
            factory.create(instance.clone(), image);
        let _opened: Result<Box<dyn CheckpointStore>, Error> = factory.open(instance, Some(0));
    }
    let _consumer: fn(&mut dyn StoreLifecycle, Instance, Image) = consumer;
}
