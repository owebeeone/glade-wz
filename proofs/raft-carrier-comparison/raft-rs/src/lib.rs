//! Deliberately refusing compiler scaffold; no upstream carrier is linked.
use glade_carrier_api::{
    DriverError, ElectionView, Event, LogicalInstant, MessageToken, NodeInputs, NodeInventory,
    TransportAction, WorkId,
};
pub struct RefusingRaftRs {
    _inputs: NodeInputs,
}
impl RefusingRaftRs {
    /// Retains already supplied owned dependencies, performs no source/task access.
    pub fn new(inputs: NodeInputs) -> Result<Self, DriverError> {
        Ok(Self { _inputs: inputs })
    }
}
impl glade_carrier_api::ElectionNode for RefusingRaftRs {
    fn advance(&mut self, _to: LogicalInstant) -> Result<(), DriverError> {
        Err(DriverError::NotQualified)
    }
    fn advance_by(&mut self, _delta_ns: u64) -> Result<(), DriverError> {
        Err(DriverError::NotQualified)
    }
    fn drive(&mut self, _work: WorkId) -> Result<Vec<Event>, DriverError> {
        Err(DriverError::NotQualified)
    }
    fn receive(&mut self, _token: MessageToken) -> Result<(), DriverError> {
        Err(DriverError::NotQualified)
    }
    fn transport(&mut self, _action: TransportAction) -> Result<Vec<Event>, DriverError> {
        Err(DriverError::NotQualified)
    }
    fn observe(&self) -> Result<ElectionView, DriverError> {
        Err(DriverError::NotQualified)
    }
    fn inventory(&self) -> Result<NodeInventory, DriverError> {
        Err(DriverError::NotQualified)
    }
    fn stop(&mut self) -> Result<(), DriverError> {
        Err(DriverError::NotQualified)
    }
}
