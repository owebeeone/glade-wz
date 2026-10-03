use glade_raft_adoption_proof::q3::Q3Session;
use glade_raft_disk::v2::{V2DiskStore, V2StoreFactory};
use glade_raft_q3_api::{
    Checkpoint, CheckpointStore, Command, ConfigIntent, ConfigKey, ConfigReceipt, Control, Error,
    Image, Instance, Receipt, ReplayResult, RequestId, Resource, StoreLifecycle, StoredEntry, View,
    conformance,
};
use std::collections::BTreeMap;
use std::path::PathBuf;

pub struct Fixture {
    root: PathBuf,
}

impl Fixture {
    pub fn new() -> Self {
        let name = std::thread::current()
            .name()
            .unwrap_or("q3")
            .replace("::", "-");
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!("q3-{}-{name}", std::process::id()));
        std::fs::create_dir(&root).expect("exclusive explicit fixture directory");
        Self { root }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.root).expect("fixture cleanup");
    }
}

pub fn instance(node: u64) -> Instance {
    Instance {
        scope: 7,
        group: 70,
        node,
        application_profile: 2,
    }
}

pub fn genesis() -> Image {
    Image {
        term: 0,
        vote: 0,
        commit: 0,
        applied: 0,
        configuration: conformance::stable(),
        checkpoint: None,
        suffix: Vec::new(),
    }
}

pub fn store() -> OwnedStore {
    let fixture = Fixture::new();
    let store = V2DiskStore::create_new(&fixture.root.join("node-1"), instance(1), genesis())
        .expect("real explicit V2 store genesis");
    OwnedStore {
        store,
        _fixture: fixture,
    }
}

pub struct OwnedStore {
    store: V2DiskStore,
    _fixture: Fixture,
}

impl CheckpointStore for OwnedStore {
    fn load(&mut self) -> Result<glade_raft_q3_api::State, glade_raft_q3_api::Error> {
        self.store.load()
    }
    fn publish(
        &mut self,
        revision: u64,
        image: Image,
    ) -> Result<glade_raft_q3_api::State, glade_raft_q3_api::Error> {
        self.store.publish(revision, image)
    }
}

pub fn session() -> OwnedSession {
    let fixture = Fixture::new();
    let mut factory = V2StoreFactory::new(fixture.root.clone());
    let mut stores = BTreeMap::new();
    for node in [1, 2, 3] {
        stores.insert(
            node,
            factory
                .create(instance(node), genesis())
                .expect("fixture initial authority"),
        );
    }
    let session = Q3Session::recover(stores, Box::new(factory), vec![1, 2])
        .expect("real RawNode session recovery and explicit campaign");
    OwnedSession {
        session,
        _fixture: fixture,
    }
}

pub struct OwnedSession {
    session: Q3Session,
    _fixture: Fixture,
}

impl glade_raft_q3_api::QualificationSession for OwnedSession {
    fn control(&mut self, _input: Control) -> Result<(), Error> {
        self.session.control(_input)
    }

    fn view(&self) -> Result<View, Error> {
        self.session.view()
    }

    fn configure(&mut self, _intent: ConfigIntent) -> Result<Option<ConfigReceipt>, Error> {
        self.session.configure(_intent)
    }

    fn configuration_outcome(&self, _key: ConfigKey) -> Result<Option<ConfigReceipt>, Error> {
        self.session.configuration_outcome(_key)
    }

    fn submit(&mut self, _command: Command) -> Result<Option<Receipt>, Error> {
        self.session.submit(_command)
    }

    fn outcome(&self, _request: RequestId) -> Result<Option<Receipt>, Error> {
        self.session.outcome(_request)
    }

    fn resource(&self, _id: u64) -> Result<Option<Resource>, Error> {
        self.session.resource(_id)
    }

    fn checkpoint(&self) -> Result<Checkpoint, Error> {
        self.session.checkpoint()
    }

    fn install(&mut self, _checkpoint: Checkpoint) -> Result<(), Error> {
        self.session.install(_checkpoint)
    }

    fn applied_entry(&self, _index: u64) -> Result<StoredEntry, Error> {
        self.session.applied_entry(_index)
    }

    fn replay(&mut self, _entry: StoredEntry) -> Result<ReplayResult, Error> {
        self.session.replay(_entry)
    }
}
