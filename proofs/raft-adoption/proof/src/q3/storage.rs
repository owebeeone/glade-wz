//! Actual carrier Storage over the coherent durable checkpoint and suffix.
use glade_raft_q3_api::{Checkpoint, Configuration, Image};
use protobuf::Message as ProtobufMessage;
use raft::eraftpb::{ConfState, Entry, HardState, Snapshot};
use raft::{GetEntriesContext, RaftState, Storage, StorageError};

pub(super) fn conf(value: &Configuration) -> ConfState {
    ConfState {
        voters: value.voters.clone(),
        learners: value.learners.clone(),
        voters_outgoing: value.voters_outgoing.clone(),
        learners_next: value.learners_next.clone(),
        auto_leave: value.auto_leave,
        ..ConfState::default()
    }
}
pub(super) fn snapshot(value: &Checkpoint) -> Snapshot {
    let mut snapshot = Snapshot {
        data: value.application.clone().into(),
        ..Snapshot::default()
    };
    let metadata = snapshot.mut_metadata();
    metadata.index = value.index;
    metadata.term = value.term;
    metadata.set_conf_state(conf(&value.configuration));
    snapshot
}
pub(super) struct Q3Storage {
    pub(super) image: Image,
    pub(super) entries: Vec<Entry>,
}
impl Q3Storage {
    pub(super) fn new(image: Image) -> Self {
        let entries = image
            .suffix
            .iter()
            .map(|entry| Entry::parse_from_bytes(&entry.bytes).expect("validated original Entry"))
            .collect();
        Self { image, entries }
    }
    fn cut(&self) -> u64 {
        self.image.checkpoint.as_ref().map_or(0, |cp| cp.index)
    }
}
impl Storage for Q3Storage {
    fn initial_state(&self) -> raft::Result<RaftState> {
        Ok(RaftState {
            hard_state: HardState {
                term: self.image.term,
                vote: self.image.vote,
                commit: self.image.commit,
                ..HardState::default()
            },
            conf_state: conf(&self.image.configuration),
        })
    }
    fn entries(
        &self,
        low: u64,
        high: u64,
        max_size: impl Into<Option<u64>>,
        _context: GetEntriesContext,
    ) -> raft::Result<Vec<Entry>> {
        if low <= self.cut() {
            return Err(StorageError::Compacted.into());
        }
        if high
            > self
                .last_index()?
                .checked_add(1)
                .ok_or(StorageError::Unavailable)?
            || low > high
        {
            return Err(StorageError::Unavailable.into());
        }
        let start = usize::try_from(low - self.cut() - 1).map_err(|_| StorageError::Unavailable)?;
        let end = usize::try_from(high - self.cut() - 1).map_err(|_| StorageError::Unavailable)?;
        let mut entries = self
            .entries
            .get(start..end)
            .ok_or(StorageError::Unavailable)?
            .to_vec();
        raft::util::limit_size(&mut entries, max_size.into());
        Ok(entries)
    }
    fn term(&self, index: u64) -> raft::Result<u64> {
        if index == self.cut() {
            return Ok(self.image.checkpoint.as_ref().map_or(0, |cp| cp.term));
        }
        if index < self.cut() {
            return Err(StorageError::Compacted.into());
        }
        self.entries
            .get(usize::try_from(index - self.cut() - 1).map_err(|_| StorageError::Unavailable)?)
            .map(|entry| entry.term)
            .ok_or(StorageError::Unavailable.into())
    }
    fn first_index(&self) -> raft::Result<u64> {
        self.cut()
            .checked_add(1)
            .ok_or(StorageError::Unavailable.into())
    }
    fn last_index(&self) -> raft::Result<u64> {
        self.cut()
            .checked_add(self.entries.len() as u64)
            .ok_or(StorageError::Unavailable.into())
    }
    fn snapshot(&self, requested: u64, _to: u64) -> raft::Result<Snapshot> {
        let cp = self
            .image
            .checkpoint
            .as_ref()
            .ok_or(StorageError::SnapshotTemporarilyUnavailable)?;
        if cp.index < requested {
            return Err(StorageError::SnapshotTemporarilyUnavailable.into());
        }
        Ok(snapshot(cp))
    }
}
