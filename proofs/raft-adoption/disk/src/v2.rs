//! Proposed Q3 provider allocation only. Every operation deliberately refuses.
//! No V2 adapter algorithm, authorization or durability is qualified yet.
use glade_raft_q3_api::{CheckpointStore, Error, Image, Instance, State};
use std::path::{Path, PathBuf};

pub struct V2DiskStore {
    // Shape only: refusing constructors never open or construct this handle.
    _file: std::fs::File,
}

impl V2DiskStore {
    pub fn create_new(_path: &Path, _instance: Instance, _initial: Image) -> Result<Self, Error> {
        Err(Error::NotQualified)
    }

    pub fn open(
        _path: &Path,
        _instance: Instance,
        _minimum_revision: Option<u64>,
    ) -> Result<Self, Error> {
        Err(Error::NotQualified)
    }

    pub fn inject_once(&mut self, _fault: crate::FaultPoint) {}
}

impl glade_raft_q3_api::CheckpointStore for V2DiskStore {
    fn load(&mut self) -> Result<State, Error> {
        Err(Error::NotQualified)
    }

    fn publish(&mut self, _expected_revision: u64, _image: Image) -> Result<State, Error> {
        Err(Error::NotQualified)
    }
}

/// Owned fixture root is injected at assembly; construction never creates it.
/// Host validation precedes each learner create. This factory grants no authority.
pub struct V2StoreFactory {
    root: PathBuf,
}

impl V2StoreFactory {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

impl glade_raft_q3_api::StoreLifecycle for V2StoreFactory {
    fn create(
        &mut self,
        instance: Instance,
        initial: Image,
    ) -> Result<Box<dyn CheckpointStore>, Error> {
        // This only wires the proposed concrete lifecycle; the provider refuses.
        let path = self.root.join(format!("node-{}", instance.node));
        Ok(Box::new(V2DiskStore::create_new(&path, instance, initial)?))
    }

    fn open(
        &mut self,
        instance: Instance,
        minimum_revision: Option<u64>,
    ) -> Result<Box<dyn CheckpointStore>, Error> {
        let path = self.root.join(format!("node-{}", instance.node));
        Ok(Box::new(V2DiskStore::open(
            &path,
            instance,
            minimum_revision,
        )?))
    }
}
