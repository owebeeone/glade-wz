//! Deliberately refusing Q3 review scaffold. No implementation is qualified.
use glade_raft_q3_api::{
    Checkpoint, CheckpointStore, Command, ConfigIntent, ConfigKey, ConfigReceipt, Control, Error,
    Image, QualificationSession, Receipt, RequestId, Resource, State, StoredEntry, View,
};

pub struct UnqualifiedStore;

impl CheckpointStore for UnqualifiedStore {
    fn load(&mut self) -> Result<State, Error> {
        Err(Error::NotQualified)
    }

    fn publish(&mut self, _expected_revision: u64, _image: Image) -> Result<State, Error> {
        Err(Error::NotQualified)
    }
}

pub struct UnqualifiedSession;

impl QualificationSession for UnqualifiedSession {
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

    fn replay(&mut self, _entry: StoredEntry) -> Result<Option<Receipt>, Error> {
        Err(Error::NotQualified)
    }
}
