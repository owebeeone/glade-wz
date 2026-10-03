use glade_carrier_api::{DriverError, ElectionNode, NodeInputs};
use glade_carrier_openraft::RefusingOpenRaft;
#[test]
fn actual_provider_implements_required_trait_and_constructor() {
    fn contract<T: ElectionNode>() {}
    contract::<RefusingOpenRaft>();
    let _constructor: fn(NodeInputs) -> Result<RefusingOpenRaft, DriverError> =
        RefusingOpenRaft::new;
}
