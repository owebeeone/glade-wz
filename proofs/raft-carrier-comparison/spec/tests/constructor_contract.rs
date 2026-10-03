use glade_carrier_api::{Carrier, ElectionNode};
use glade_carrier_spec::fixture::Fixture;

#[test]
fn constructor_compiler_witness_uses_separately_created_contexts() {
    let first = Fixture::new(Carrier::RaftRs, 301);
    let second = Fixture::new(Carrier::OpenRaft, 302);
    let first_inputs = first.inputs(1, 1);
    let second_inputs = second.inputs(1, 1);
    assert_ne!(first_inputs.sources.scope, second_inputs.sources.scope);
    let first_node = glade_carrier_raft_rs::RefusingRaftRs::new(first_inputs).unwrap();
    let second_node = glade_carrier_openraft::RefusingOpenRaft::new(second_inputs).unwrap();
    fn consumes(_: &dyn ElectionNode) {}
    consumes(&first_node);
    consumes(&second_node);
}
