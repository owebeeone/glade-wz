use glade_raft_q3_api::{CheckpointStore, QualificationSession};

#[test]
fn dyn_consumers_compile_without_carrier_or_disk_types() {
    fn store_consumer(store: &mut dyn CheckpointStore) {
        let _result = store.load();
    }
    fn session_consumer(session: &mut dyn QualificationSession) {
        let _result = session.view();
    }
    // These concrete function-pointer types force both dyn consumer signatures
    // through compilation. No behavioral provider is claimed by this witness.
    let _store: fn(&mut dyn CheckpointStore) = store_consumer;
    let _session: fn(&mut dyn QualificationSession) = session_consumer;
}
