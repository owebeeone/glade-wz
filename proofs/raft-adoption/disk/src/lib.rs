//! Q2 local process-crash adapter: full-image journal, fixed logical binding.
//! This is not a certified power-loss or independent physical quorum guarantee.
mod codec;
mod journal;
pub mod v2;
mod validation;

use glade_raft_durability_api::{Binding, DurableImage, StoreError, StoredState};
use std::fs::{File, OpenOptions, TryLockError};
use std::io::{ErrorKind, Write};
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
    file: File,
    state: StoredState,
    poisoned: bool,
    fault: Option<FaultPoint>,
}

fn lock(file: &File) -> Result<(), StoreError> {
    file.try_lock().map_err(|error| match error {
        TryLockError::WouldBlock => StoreError::Locked,
        TryLockError::Error(_) => StoreError::Io,
    })
}

impl DiskStore {
    pub fn create_new(path: &Path, binding: Binding) -> Result<Self, StoreError> {
        if !validation::binding_valid(&binding) {
            return Err(StoreError::BindingMismatch);
        }
        let parent = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let parent_directory = File::open(parent).map_err(|_| StoreError::Io)?;
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create_new(true)
            .open(path)
            .map_err(|error| {
                if error.kind() == ErrorKind::AlreadyExists {
                    StoreError::AlreadyExists
                } else {
                    StoreError::Io
                }
            })?;
        lock(&file)?;
        let state = StoredState {
            revision: 0,
            binding,
            image: DurableImage::default(),
        };
        let record = codec::encode(&state)?;
        // Any creation failure leaves its file as evidence; never delete/reset.
        file.write_all(&record).map_err(|_| StoreError::Io)?;
        file.sync_all().map_err(|_| StoreError::Io)?;
        parent_directory.sync_all().map_err(|_| StoreError::Io)?;
        Ok(Self {
            file,
            state,
            poisoned: false,
            fault: None,
        })
    }

    pub fn open(
        path: &Path,
        binding: Binding,
        minimum_revision: Option<u64>,
    ) -> Result<Self, StoreError> {
        if !validation::binding_valid(&binding) {
            return Err(StoreError::BindingMismatch);
        }
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(path)
            .map_err(|error| {
                if error.kind() == ErrorKind::NotFound {
                    StoreError::Missing
                } else {
                    StoreError::Io
                }
            })?;
        lock(&file)?;
        let state = journal::recover(&mut file, &binding, minimum_revision)?;
        // Complete unknown writes may be visible without having reached their
        // former sync barrier. Establish it before admitting recovered service.
        file.sync_all().map_err(|_| StoreError::Io)?;
        let parent = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| StoreError::Io)?;
        Ok(Self {
            file,
            state,
            poisoned: false,
            fault: None,
        })
    }

    pub fn with_fault(mut self, fault: FaultPoint) -> Self {
        self.fault = Some(fault);
        self
    }

    fn publish(&mut self, record: &[u8], fault: Option<FaultPoint>) -> Result<(), StoreError> {
        if fault == Some(FaultPoint::PartialWrite) {
            self.file
                .write_all(&record[..record.len() / 2])
                .map_err(|_| StoreError::Io)?;
            return Err(StoreError::Io);
        }
        self.file.write_all(record).map_err(|_| StoreError::Io)?;
        if matches!(fault, Some(FaultPoint::AfterWrite | FaultPoint::BeforeSync)) {
            return Err(StoreError::Io);
        }
        self.file.sync_all().map_err(|_| StoreError::Io)?;
        if fault == Some(FaultPoint::AfterSync) {
            return Err(StoreError::Io);
        }
        Ok(())
    }
}

impl glade_raft_durability_api::DurableStore for DiskStore {
    fn load(&mut self) -> Result<StoredState, StoreError> {
        if self.poisoned {
            return Err(StoreError::Poisoned);
        }
        Ok(self.state.clone())
    }

    fn persist(
        &mut self,
        expected_revision: u64,
        image: DurableImage,
    ) -> Result<StoredState, StoreError> {
        if self.poisoned {
            return Err(StoreError::Poisoned);
        }
        if expected_revision != self.state.revision {
            return Err(StoreError::RevisionConflict {
                expected: expected_revision,
                actual: self.state.revision,
            });
        }
        validation::transition(&self.state.binding, &self.state.image, &image)?;
        let revision = self
            .state
            .revision
            .checked_add(1)
            .ok_or(StoreError::CapacityExhausted)?;
        let state = StoredState {
            revision,
            binding: self.state.binding.clone(),
            image,
        };
        let record = codec::encode(&state)?;
        let fault = self.fault.take();
        if fault == Some(FaultPoint::BeforeWrite) {
            return Err(StoreError::Io);
        }
        // Unknown publication cannot permit the live instance to serve again.
        self.poisoned = true;
        self.publish(&record, fault)?;
        self.state = state;
        self.poisoned = false;
        Ok(self.state.clone())
    }
}
