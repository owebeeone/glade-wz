//! Shared B0 consumer/oracle. Normal dependencies are contracts only.
//! Fixture scheduling and transport are pure test scaffolding, never carriers.
pub mod b0;
pub mod fixture;
pub mod oracle;
use glade_carrier_api::{Carrier, DriverError, ElectionNode, NodeInputs};
pub trait Provider {
    fn carrier(&self) -> Carrier;
    fn construct(&self, inputs: NodeInputs) -> Result<Box<dyn ElectionNode>, DriverError>;
}
