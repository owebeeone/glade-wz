//! Intentionally unqualified Q2 adapter scaffold. No persistence yet.
use glade_raft_durability_api::{Binding, DurableImage, StoreError, StoredState};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultPoint {
    BeforeWrite,
    PartialWrite,
    AfterWrite,
    BeforeSync,
    AfterSync,
}

pub struct DiskStore {
    _unqualified_handle: Option<std::fs::File>,
}

impl DiskStore {
    pub fn create_new(_path: &Path, _binding: Binding) -> Result<Self, StoreError> {
        Err(StoreError::NotQualified)
    }

    pub fn open(
        _path: &Path,
        _binding: Binding,
        _minimum_revision: Option<u64>,
    ) -> Result<Self, StoreError> {
        Err(StoreError::NotQualified)
    }

    pub fn with_fault(self, _fault: FaultPoint) -> Self {
        self
    }
}

impl glade_raft_durability_api::DurableStore for DiskStore {
    fn load(&mut self) -> Result<StoredState, StoreError> {
        Err(StoreError::NotQualified)
    }

    fn persist(
        &mut self,
        _expected_revision: u64,
        _image: DurableImage,
    ) -> Result<StoredState, StoreError> {
        Err(StoreError::NotQualified)
    }
}
