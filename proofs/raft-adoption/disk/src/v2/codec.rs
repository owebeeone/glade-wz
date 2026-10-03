//! Bounded V2 full-image framing. No semantic interpretation of application data.
use glade_raft_q3_api::{
    Checkpoint, Configuration, Error, GroupIdentity, Image, Instance, State, StoredEntry,
};
use std::io::{ErrorKind, Read};
const MAGIC: &[u8; 8] = b"GQ3JRNL2";
const LIMIT: usize = 16 * 1024 * 1024;

fn crc(bytes: &[u8]) -> u64 {
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
struct Encoder(Vec<u8>);
impl Encoder {
    fn bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        if self
            .0
            .len()
            .checked_add(bytes.len())
            .and_then(|n| n.checked_add(8))
            .is_none_or(|n| n > LIMIT)
        {
            return Err(Error::CapacityExhausted);
        }
        self.0.extend(bytes);
        Ok(())
    }
    fn word(&mut self, value: u64) -> Result<(), Error> {
        self.bytes(&value.to_le_bytes())
    }
    fn list(&mut self, nodes: &[u64]) -> Result<(), Error> {
        self.word(nodes.len() as u64)?;
        for node in nodes {
            self.word(*node)?;
        }
        Ok(())
    }
    fn configuration(&mut self, value: &Configuration) -> Result<(), Error> {
        for list in [
            &value.voters,
            &value.learners,
            &value.voters_outgoing,
            &value.learners_next,
        ] {
            self.list(list)?;
        }
        self.word(u64::from(value.auto_leave))?;
        self.word(value.index)
    }
}
pub(super) fn encode(state: &State) -> Result<Vec<u8>, Error> {
    let mut out = Encoder(Vec::new());
    out.bytes(MAGIC)?;
    out.word(0)?;
    for word in [
        state.revision,
        state.instance.scope,
        state.instance.group,
        state.instance.node,
        state.instance.application_profile,
        state.image.term,
        state.image.vote,
        state.image.commit,
        state.image.applied,
    ] {
        out.word(word)?;
    }
    out.configuration(&state.image.configuration)?;
    out.word(u64::from(state.image.checkpoint.is_some()))?;
    if let Some(cp) = &state.image.checkpoint {
        for word in [
            cp.binding.scope,
            cp.binding.group,
            cp.binding.application_profile,
            cp.index,
            cp.term,
        ] {
            out.word(word)?;
        }
        out.configuration(&cp.configuration)?;
        out.word(cp.application.len() as u64)?;
        out.bytes(&cp.application)?;
    }
    out.word(state.image.suffix.len() as u64)?;
    for entry in &state.image.suffix {
        for word in [entry.index, entry.term, entry.bytes.len() as u64] {
            out.word(word)?;
        }
        out.bytes(&entry.bytes)?;
    }
    let length = out.0.len().checked_add(8).ok_or(Error::CapacityExhausted)?;
    out.0[8..16].copy_from_slice(&(length as u64).to_le_bytes());
    let checksum = crc(&out.0);
    out.0.extend(checksum.to_le_bytes());
    Ok(out.0)
}
struct Cursor<'a>(&'a [u8]);
impl Cursor<'_> {
    fn take(&mut self, count: usize) -> Result<&[u8], Error> {
        let bytes = self.0.get(..count).ok_or(Error::Quarantined)?;
        self.0 = &self.0[count..];
        Ok(bytes)
    }
    fn word(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(
            self.take(8)?.try_into().map_err(|_| Error::Quarantined)?,
        ))
    }
    fn count(&mut self) -> Result<usize, Error> {
        usize::try_from(self.word()?).map_err(|_| Error::Quarantined)
    }
    fn list(&mut self) -> Result<Vec<u64>, Error> {
        let count = self.count()?;
        if count > 4 {
            return Err(Error::Quarantined);
        }
        (0..count).map(|_| self.word()).collect()
    }
    fn configuration(&mut self) -> Result<Configuration, Error> {
        let voters = self.list()?;
        let learners = self.list()?;
        let voters_outgoing = self.list()?;
        let learners_next = self.list()?;
        let auto_leave = match self.word()? {
            0 => false,
            1 => true,
            _ => {
                return Err(Error::Quarantined);
            }
        };
        Ok(Configuration {
            voters,
            learners,
            voters_outgoing,
            learners_next,
            auto_leave,
            index: self.word()?,
        })
    }
}
fn decode(bytes: &[u8]) -> Result<State, Error> {
    let mut input = Cursor(bytes);
    let revision = input.word()?;
    let instance = Instance {
        scope: input.word()?,
        group: input.word()?,
        node: input.word()?,
        application_profile: input.word()?,
    };
    let term = input.word()?;
    let vote = input.word()?;
    let commit = input.word()?;
    let applied = input.word()?;
    let configuration = input.configuration()?;
    let checkpoint = match input.word()? {
        0 => None,
        1 => {
            let binding = GroupIdentity {
                scope: input.word()?,
                group: input.word()?,
                application_profile: input.word()?,
            };
            let index = input.word()?;
            let term = input.word()?;
            let configuration = input.configuration()?;
            let count = input.count()?;
            let application = input.take(count)?.to_vec();
            Some(Checkpoint {
                binding,
                index,
                term,
                configuration,
                application,
            })
        }
        _ => {
            return Err(Error::Quarantined);
        }
    };
    let count = input.count()?;
    if count > input.0.len() / 24 {
        return Err(Error::Quarantined);
    }
    let mut suffix = Vec::with_capacity(count);
    for _ in 0..count {
        let index = input.word()?;
        let term = input.word()?;
        let count = input.count()?;
        suffix.push(StoredEntry {
            index,
            term,
            bytes: input.take(count)?.to_vec(),
        });
    }
    if !input.0.is_empty() {
        return Err(Error::Quarantined);
    }
    Ok(State {
        revision,
        instance,
        image: Image {
            term,
            vote,
            commit,
            applied,
            configuration,
            checkpoint,
            suffix,
        },
    })
}
fn read_exact(reader: &mut impl Read, bytes: &mut [u8]) -> Result<(), Error> {
    reader.read_exact(bytes).map_err(|error| {
        if error.kind() == ErrorKind::UnexpectedEof {
            Error::Quarantined
        } else {
            Error::IoUnknown
        }
    })
}
pub(super) fn read_record(reader: &mut impl Read) -> Result<Option<State>, Error> {
    let mut header = [0_u8; 16];
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
                return Err(Error::IoUnknown);
            }
        }
    }
    read_exact(reader, &mut header[1..])?;
    if header[..8] != *MAGIC {
        return Err(Error::IncompatibleVersion);
    }
    let size = usize::try_from(u64::from_le_bytes(
        header[8..].try_into().map_err(|_| Error::Quarantined)?,
    ))
    .map_err(|_| Error::Quarantined)?;
    if !(24..=LIMIT).contains(&size) {
        return Err(Error::Quarantined);
    }
    let mut record = Vec::with_capacity(size);
    record.extend(header);
    record.resize(size, 0);
    read_exact(reader, &mut record[16..])?;
    let end = size - 8;
    let expected = u64::from_le_bytes(record[end..].try_into().map_err(|_| Error::Quarantined)?);
    if crc(&record[..end]) != expected {
        return Err(Error::Quarantined);
    }
    decode(&record[16..end]).map(Some)
}
