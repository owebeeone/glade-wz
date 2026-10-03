//! Proposed Q3 actual-carrier provider allocation. All operations refuse.
//! Q1a/Q2 providers remain unchanged; this is not a permissive state model.
use glade_raft_q3_api::{
    Checkpoint, CheckpointStore, Command, ConfigIntent, ConfigKey, ConfigReceipt, Control, Error,
    Receipt, ReplayResult, RequestId, Resource, StoredEntry, View,
};

use glade_raft_q3_api::StoreLifecycle;
use std::collections::BTreeMap;

pub struct Q3Session;

impl Q3Session {
    /// Validate all retained history before constructing/serving RawNodes.
    /// The explicit authority principals belong to the bound fixture group.
    /// Host admission MUST precede lifecycle.create for any new learner.
    pub fn recover(
        _stores: BTreeMap<u64, Box<dyn CheckpointStore>>,
        _lifecycle: Box<dyn StoreLifecycle>,
        _configuration_authorities: Vec<u64>,
    ) -> Result<Self, Error> {
        Err(Error::NotQualified)
    }
}

impl glade_raft_q3_api::QualificationSession for Q3Session {
    fn control(&mut self, _input: Control) -> Result<(), Error> {
        Err(Error::NotQualified)
    }

    fn view(&self) -> Result<View, Error> {
        Err(Error::NotQualified)
    }

    fn configure(&mut self, _intent: ConfigIntent) -> Result<Option<ConfigReceipt>, Error> {
        Err(Error::NotQualified)
    }

    fn configuration_outcome(&self, _key: ConfigKey) -> Result<Option<ConfigReceipt>, Error> {
        Err(Error::NotQualified)
    }

    fn submit(&mut self, _command: Command) -> Result<Option<Receipt>, Error> {
        Err(Error::NotQualified)
    }

    fn outcome(&self, _request: RequestId) -> Result<Option<Receipt>, Error> {
        Err(Error::NotQualified)
    }

    fn resource(&self, _id: u64) -> Result<Option<Resource>, Error> {
        Err(Error::NotQualified)
    }

    fn checkpoint(&self) -> Result<Checkpoint, Error> {
        Err(Error::NotQualified)
    }

    fn install(&mut self, _checkpoint: Checkpoint) -> Result<(), Error> {
        Err(Error::NotQualified)
    }

    fn applied_entry(&self, _index: u64) -> Result<StoredEntry, Error> {
        Err(Error::NotQualified)
    }

    fn replay(&mut self, _entry: StoredEntry) -> Result<ReplayResult, Error> {
        Err(Error::NotQualified)
    }
}
