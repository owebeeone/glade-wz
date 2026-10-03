use glade_carrier_api::{DriverError, ElectionNode, NodeInputs};
use glade_carrier_raft_rs::RefusingRaftRs;
#[test]
fn actual_provider_implements_required_trait_and_constructor() {
    fn contract<T: ElectionNode>() {}
    contract::<RefusingRaftRs>();
    let _constructor: fn(NodeInputs) -> Result<RefusingRaftRs, DriverError> = RefusingRaftRs::new;
}
