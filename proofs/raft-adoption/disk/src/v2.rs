//! V2 append-only complete-image journal, separate from the accepted Q2 format.
use crate::FaultPoint;
use glade_raft_q3_api::{CheckpointStore, Error, Image, Instance, State};
mod codec;
mod journal;
mod validation;
use std::fs::{File, OpenOptions, TryLockError};
use std::io::{ErrorKind, Write};
use std::path::Path;

pub struct V2DiskStore {
    file: File,
    state: State,
    poisoned: bool,
    fault: Option<FaultPoint>,
}

fn lock(file: &File) -> Result<(), Error> {
    file.try_lock().map_err(|error| match error {
        TryLockError::WouldBlock => Error::Locked,
        TryLockError::Error(_) => Error::IoUnknown,
    })
}

impl V2DiskStore {
    pub fn create_new(path: &Path, instance: Instance, initial: Image) -> Result<Self, Error> {
        if !validation::instance_valid(&instance) {
            return Err(Error::WrongBinding);
        }
        validation::validate(&instance, &initial)?;
        let initial_state = State {
            revision: 0,
            instance: instance.clone(),
            image: initial,
        };
        let record = codec::encode(&initial_state)?;
        let parent = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let parent_directory = File::open(parent).map_err(|_| Error::IoUnknown)?;
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create_new(true)
            .open(path)
            .map_err(|error| {
                if error.kind() == ErrorKind::AlreadyExists {
                    Error::AlreadyExists
                } else {
                    Error::IoUnknown
                }
            })?;
        lock(&file)?;
        let state = initial_state;
        // Any creation failure leaves its file as evidence; never delete/reset.
        file.write_all(&record).map_err(|_| Error::IoUnknown)?;
        file.sync_all().map_err(|_| Error::IoUnknown)?;
        parent_directory.sync_all().map_err(|_| Error::IoUnknown)?;
        Ok(Self {
            file,
            state,
            poisoned: false,
            fault: None,
        })
    }

    pub fn open(
        path: &Path,
        instance: Instance,
        minimum_revision: Option<u64>,
    ) -> Result<Self, Error> {
        if !validation::instance_valid(&instance) {
            return Err(Error::WrongBinding);
        }
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(path)
            .map_err(|error| {
                if error.kind() == ErrorKind::NotFound {
                    Error::Missing
                } else {
                    Error::IoUnknown
                }
            })?;
        lock(&file)?;
        let state = journal::recover(&mut file, &instance, minimum_revision)?;
        // Complete unknown writes may be visible without having reached their
        // former sync barrier. Establish it before admitting recovered service.
        file.sync_all().map_err(|_| Error::IoUnknown)?;
        let parent = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| Error::IoUnknown)?;
        Ok(Self {
            file,
            state,
            poisoned: false,
            fault: None,
        })
    }

    pub fn inject_once(&mut self, fault: FaultPoint) {
        self.fault = Some(fault);
    }

    fn write_record(&mut self, record: &[u8], fault: Option<FaultPoint>) -> Result<(), Error> {
        if fault == Some(FaultPoint::PartialWrite) {
            self.file
                .write_all(&record[..record.len() / 2])
                .map_err(|_| Error::IoUnknown)?;
            return Err(Error::IoUnknown);
        }
        self.file.write_all(record).map_err(|_| Error::IoUnknown)?;
        if matches!(fault, Some(FaultPoint::AfterWrite | FaultPoint::BeforeSync)) {
            return Err(Error::IoUnknown);
        }
        self.file.sync_all().map_err(|_| Error::IoUnknown)?;
        if fault == Some(FaultPoint::AfterSync) {
            return Err(Error::IoUnknown);
        }
        Ok(())
    }
}

impl glade_raft_q3_api::CheckpointStore for V2DiskStore {
    fn load(&mut self) -> Result<State, Error> {
        if self.poisoned {
            return Err(Error::Poisoned);
        }
        Ok(self.state.clone())
    }

    fn publish(&mut self, expected_revision: u64, image: Image) -> Result<State, Error> {
        if self.poisoned {
            return Err(Error::Poisoned);
        }
        if expected_revision != self.state.revision {
            return Err(Error::RevisionConflict {
                expected: expected_revision,
                actual: self.state.revision,
            });
        }
        validation::transition(&self.state.instance, &self.state.image, &image)?;
        let revision = self
            .state
            .revision
            .checked_add(1)
            .ok_or(Error::CapacityExhausted)?;
        let state = State {
            revision,
            instance: self.state.instance.clone(),
            image,
        };
        let record = codec::encode(&state)?;
        let fault = self.fault.take();
        if fault == Some(FaultPoint::BeforeWrite) {
            return Err(Error::IoUnknown);
        }
        // Unknown publication cannot permit the live instance to serve again.
        self.poisoned = true;
        self.write_record(&record, fault)?;
        self.state = state;
        self.poisoned = false;
        Ok(self.state.clone())
    }
}

/// Caller-injected owned directory; factory operations do not confer authority.
pub struct V2StoreFactory {
    root: std::path::PathBuf,
}
impl V2StoreFactory {
    pub fn new(root: std::path::PathBuf) -> Self {
        Self { root }
    }
}
impl glade_raft_q3_api::StoreLifecycle for V2StoreFactory {
    fn create(
        &mut self,
        instance: Instance,
        image: Image,
    ) -> Result<Box<dyn CheckpointStore>, Error> {
        Ok(Box::new(V2DiskStore::create_new(
            &self.root.join(format!("node-{}", instance.node)),
            instance,
            image,
        )?))
    }
    fn open(
        &mut self,
        instance: Instance,
        floor: Option<u64>,
    ) -> Result<Box<dyn CheckpointStore>, Error> {
        Ok(Box::new(V2DiskStore::open(
            &self.root.join(format!("node-{}", instance.node)),
            instance,
            floor,
        )?))
    }
}

mod parser_tests;
