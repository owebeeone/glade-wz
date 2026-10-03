//! Private experimental journal framing. Length covers header, payload and CRC.
use glade_raft_durability_api::{Binding, DurableImage, StoreError, StoredEntry, StoredState};
use std::io::{ErrorKind, Read};

const MAGIC: &[u8; 8] = b"GQ2JRNL1";
const MAX_RECORD: usize = 16 * 1024 * 1024;
const FRAMING: usize = 24;

fn crc64(bytes: &[u8]) -> u64 {
    let mut crc = 0_u64;
    for byte in bytes {
        crc ^= u64::from(*byte) << 56;
        for _ in 0..8 {
            crc = if crc & (1 << 63) != 0 {
                (crc << 1) ^ 0x42F0_E1EB_A9EA_3693
            } else {
                crc << 1
            };
        }
    }
    crc
}

fn checked_size(state: &StoredState) -> Result<usize, StoreError> {
    // Nine scalar words, voter words, and three scalar words per log entry.
    let mut size = FRAMING
        .checked_add(9 * 8)
        .and_then(|value| value.checked_add(state.binding.voters.len().checked_mul(8)?))
        .ok_or(StoreError::CapacityExhausted)?;
    for entry in &state.image.entries {
        size = size
            .checked_add(24)
            .and_then(|value| value.checked_add(entry.bytes.len()))
            .ok_or(StoreError::CapacityExhausted)?;
        if size > MAX_RECORD {
            return Err(StoreError::CapacityExhausted);
        }
    }
    if size > MAX_RECORD {
        return Err(StoreError::CapacityExhausted);
    }
    Ok(size)
}

pub(crate) fn encode(state: &StoredState) -> Result<Vec<u8>, StoreError> {
    let size = checked_size(state)?;
    let mut bytes = Vec::with_capacity(size);
    bytes.extend(MAGIC);
    bytes.extend((size as u64).to_le_bytes());
    for word in [
        state.revision,
        state.binding.scope,
        state.binding.node,
        state.binding.application_profile,
        state.binding.voters.len() as u64,
    ] {
        bytes.extend(word.to_le_bytes());
    }
    for voter in &state.binding.voters {
        bytes.extend(voter.to_le_bytes());
    }
    for word in [
        state.image.term,
        state.image.vote,
        state.image.commit,
        state.image.entries.len() as u64,
    ] {
        bytes.extend(word.to_le_bytes());
    }
    for entry in &state.image.entries {
        for word in [entry.index, entry.term, entry.bytes.len() as u64] {
            bytes.extend(word.to_le_bytes());
        }
        bytes.extend(&entry.bytes);
    }
    let crc = crc64(&bytes);
    bytes.extend(crc.to_le_bytes());
    debug_assert_eq!(bytes.len(), size);
    Ok(bytes)
}

struct Cursor<'a>(&'a [u8]);
impl Cursor<'_> {
    fn take(&mut self, count: usize) -> Result<&[u8], StoreError> {
        let taken = self.0.get(..count).ok_or(StoreError::Quarantined)?;
        self.0 = &self.0[count..];
        Ok(taken)
    }

    fn word(&mut self) -> Result<u64, StoreError> {
        let bytes: [u8; 8] = self
            .take(8)?
            .try_into()
            .map_err(|_| StoreError::Quarantined)?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn count(&mut self) -> Result<usize, StoreError> {
        usize::try_from(self.word()?).map_err(|_| StoreError::Quarantined)
    }
}

fn decode(payload: &[u8]) -> Result<StoredState, StoreError> {
    let mut cursor = Cursor(payload);
    let revision = cursor.word()?;
    let scope = cursor.word()?;
    let node = cursor.word()?;
    let application_profile = cursor.word()?;
    let voter_count = cursor.count()?;
    if !(2..=3).contains(&voter_count) {
        return Err(StoreError::Quarantined);
    }
    let mut voters = Vec::with_capacity(voter_count);
    for _ in 0..voter_count {
        voters.push(cursor.word()?);
    }
    let term = cursor.word()?;
    let vote = cursor.word()?;
    let commit = cursor.word()?;
    let entry_count = cursor.count()?;
    if entry_count > cursor.0.len() / 24 {
        return Err(StoreError::Quarantined);
    }
    let mut entries = Vec::with_capacity(entry_count);
    for _ in 0..entry_count {
        let index = cursor.word()?;
        let term = cursor.word()?;
        let count = cursor.count()?;
        let bytes = cursor.take(count)?.to_vec();
        entries.push(StoredEntry { index, term, bytes });
    }
    if !cursor.0.is_empty() {
        return Err(StoreError::Quarantined);
    }
    Ok(StoredState {
        revision,
        binding: Binding {
            scope,
            node,
            voters,
            application_profile,
        },
        image: DurableImage {
            term,
            vote,
            commit,
            entries,
        },
    })
}

fn read_exact(reader: &mut impl Read, bytes: &mut [u8]) -> Result<(), StoreError> {
    reader.read_exact(bytes).map_err(|error| {
        if error.kind() == ErrorKind::UnexpectedEof {
            StoreError::Quarantined
        } else {
            StoreError::Io
        }
    })
}

pub(crate) fn read_record(reader: &mut impl Read) -> Result<Option<StoredState>, StoreError> {
    let mut header = [0_u8; 16];
    // One byte distinguishes clean EOF from any partial next record.
    loop {
        match reader.read(&mut header[..1]) {
            Ok(0) => {
                return Ok(None);
            }
            Ok(_) => {
                break;
            }
            Err(error) if error.kind() == ErrorKind::Interrupted => {
                continue;
            }
            Err(_) => {
                return Err(StoreError::Io);
            }
        }
    }
    read_exact(reader, &mut header[1..])?;
    if header[..8] != *MAGIC {
        return Err(StoreError::Quarantined);
    }
    let length = u64::from_le_bytes(
        header[8..]
            .try_into()
            .map_err(|_| StoreError::Quarantined)?,
    );
    let size = usize::try_from(length).map_err(|_| StoreError::Quarantined)?;
    if !(FRAMING..=MAX_RECORD).contains(&size) {
        return Err(StoreError::Quarantined);
    }
    // At most one bounded record is allocated; the whole journal is streamed.
    let mut record = Vec::with_capacity(size);
    record.extend(header);
    record.resize(size, 0);
    read_exact(reader, &mut record[16..])?;
    let crc_position = size - 8;
    let expected = u64::from_le_bytes(
        record[crc_position..]
            .try_into()
            .map_err(|_| StoreError::Quarantined)?,
    );
    if crc64(&record[..crc_position]) != expected {
        return Err(StoreError::Quarantined);
    }
    decode(&record[16..crc_position]).map(Some)
}
