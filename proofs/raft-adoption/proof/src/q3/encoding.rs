//! Private bounded version-2 ordered intent/readiness encoding.
use glade_raft_q3_api::{Change, ConfigIntent, ConfigKey, Error};
pub(super) const LIMIT: usize = 16 * 1024 * 1024;
pub(super) struct Reader<'a>(pub(super) &'a [u8]);
impl Reader<'_> {
    pub(super) fn take(&mut self, n: usize) -> Result<&[u8], Error> {
        let bytes = self.0.get(..n).ok_or(Error::Quarantined)?;
        self.0 = &self.0[n..];
        Ok(bytes)
    }
    pub(crate) fn word(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(
            self.take(8)?.try_into().map_err(|_| Error::Quarantined)?,
        ))
    }
    pub(crate) fn bytes(&mut self) -> Result<Vec<u8>, Error> {
        let count = usize::try_from(self.word()?).map_err(|_| Error::Quarantined)?;
        if count > LIMIT {
            return Err(Error::CapacityExhausted);
        }
        Ok(self.take(count)?.to_vec())
    }
}
pub(crate) fn word(out: &mut Vec<u8>, value: u64) {
    out.extend(value.to_le_bytes());
}
pub(crate) fn bytes(out: &mut Vec<u8>, bytes: &[u8]) {
    word(out, bytes.len() as u64);
    out.extend(bytes);
}
pub(super) fn intent(out: &mut Vec<u8>, value: &ConfigIntent) {
    for scalar in [
        value.key.scope,
        value.key.group,
        value.key.principal,
        value.key.sequence,
        value.expected_configuration,
    ] {
        word(out, scalar);
    }
    match &value.change {
        Change::AddLearner { node } => {
            word(out, 1);
            word(out, *node);
        }
        Change::EnterJoint { voters } => {
            word(out, 2);
            word(out, voters.len() as u64);
            for voter in voters {
                word(out, *voter);
            }
        }
        Change::LeaveJoint => {
            word(out, 3);
        }
    }
}
pub(super) fn read_intent(input: &mut Reader<'_>) -> Result<ConfigIntent, Error> {
    let key = ConfigKey {
        scope: input.word()?,
        group: input.word()?,
        principal: input.word()?,
        sequence: input.word()?,
    };
    let expected_configuration = input.word()?;
    let change = match input.word()? {
        1 => Change::AddLearner {
            node: input.word()?,
        },
        2 => {
            let count = input.word()?;
            if count == 0 || count > 4 {
                return Err(Error::Quarantined);
            }
            let mut voters = Vec::new();
            for _ in 0..count {
                voters.push(input.word()?);
            }
            if !voters.iter().all(|id| (1..=4).contains(id))
                || !voters.windows(2).all(|pair| pair[0] < pair[1])
            {
                return Err(Error::Quarantined);
            }
            Change::EnterJoint { voters }
        }
        3 => Change::LeaveJoint,
        _ => {
            return Err(Error::Quarantined);
        }
    };
    Ok(ConfigIntent {
        key,
        expected_configuration,
        change,
    })
}
#[derive(Clone)]
pub(super) struct PromotionProof {
    pub(super) node: u64,
    pub(super) index: u64,
    pub(super) term: u64,
    pub(super) state: Vec<u8>,
}
pub(super) fn context(value: &ConfigIntent, proofs: &[PromotionProof]) -> Vec<u8> {
    let mut out = vec![2];
    intent(&mut out, value);
    word(&mut out, proofs.len() as u64);
    for proof in proofs {
        for value in [7, 70, 2, proof.node, proof.index, proof.term] {
            word(&mut out, value);
        }
        bytes(&mut out, &proof.state);
    }
    out
}
pub(super) fn parse_context(bytes: &[u8]) -> Result<(ConfigIntent, Vec<PromotionProof>), Error> {
    let mut input = Reader(bytes);
    if input.take(1)? != [2] {
        return Err(Error::Quarantined);
    }
    let intent = read_intent(&mut input)?;
    let count = input.word()?;
    if count > 4 {
        return Err(Error::Quarantined);
    }
    let mut proofs = Vec::new();
    for _ in 0..count {
        if [input.word()?, input.word()?, input.word()?] != [7, 70, 2] {
            return Err(Error::Quarantined);
        }
        proofs.push(PromotionProof {
            node: input.word()?,
            index: input.word()?,
            term: input.word()?,
            state: input.bytes()?,
        });
    }
    if !input.0.is_empty() {
        return Err(Error::Quarantined);
    }
    Ok((intent, proofs))
}
