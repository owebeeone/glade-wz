//! Deterministic complete original history and ordered configuration/application.
pub(super) use super::carrier::{change, parse};
use super::encoding::{self, PromotionProof, Reader, bytes, word};
use crate::application::{Application, q3_evidence};
use glade_raft_q3_api::{
    Change, Checkpoint, ConfigIntent, ConfigKey, ConfigOutcome, ConfigReceipt, ConfigRejection,
    Configuration, Error, GroupIdentity, ReplayResult, StoredEntry, conformance,
};
use protobuf::Message as ProtobufMessage;
use raft::eraftpb::{ConfChangeV2, EntryType};
use std::collections::BTreeMap;

pub(super) struct Machine {
    pub(super) application: Application,
    pub(super) configuration: Configuration,
    pub(super) history: BTreeMap<u64, (StoredEntry, ReplayResult)>,
    pub(super) configurations: BTreeMap<ConfigKey, ConfigReceipt>,
}
impl Machine {
    pub(super) fn new() -> Self {
        Self {
            application: Application::new(),
            configuration: conformance::stable(),
            history: BTreeMap::new(),
            configurations: BTreeMap::new(),
        }
    }
    pub(super) fn applied(&self) -> u64 {
        self.history.last_key_value().map_or(0, |(index, _)| *index)
    }
    pub(super) fn admission(&self, value: &ConfigIntent) -> Result<(), Error> {
        if value.key.scope != 7 || value.key.group != 70 {
            return Err(Error::WrongBinding);
        }
        if !matches!(value.key.principal, 1 | 2) {
            return Err(Error::Unauthorized);
        }
        if value.expected_configuration != self.configuration.index {
            return Err(Error::StaleConfiguration);
        }
        match &value.change {
            Change::AddLearner { node } => {
                if *node != 4
                    || self.configuration.voters.contains(node)
                    || self.configuration.voters_outgoing.contains(node)
                {
                    return Err(Error::InvalidImage);
                }
                if !self.configuration.voters_outgoing.is_empty() {
                    return Err(Error::JointInProgress);
                }
            }
            Change::EnterJoint { voters } => {
                if !self.configuration.voters_outgoing.is_empty() {
                    return Err(Error::JointInProgress);
                }
                if voters.is_empty()
                    || !voters.iter().all(|node| (1..=4).contains(node))
                    || !voters.windows(2).all(|pair| pair[0] < pair[1])
                {
                    return Err(Error::InvalidImage);
                }
                if !self.application.homes_fit(voters) {
                    return Err(Error::HomeInUse);
                }
            }
            Change::LeaveJoint => {
                if self.configuration.voters_outgoing.is_empty() {
                    return Err(Error::NotJoint);
                }
            }
        }
        Ok(())
    }
    pub(super) fn apply(
        &mut self,
        stored: StoredEntry,
    ) -> Result<(ReplayResult, Option<ConfChangeV2>), Error> {
        if stored.index
            != self
                .applied()
                .checked_add(1)
                .ok_or(Error::CapacityExhausted)?
        {
            return Err(Error::Quarantined);
        }
        if self
            .history
            .last_key_value()
            .is_some_and(|(_, (previous, _))| stored.term < previous.term)
        {
            return Err(Error::Quarantined);
        }
        let entry = parse(&stored)?;
        let mut accepted_change = None;
        let result = match entry.get_entry_type() {
            EntryType::EntryNormal => {
                let (command, readiness) = if entry.data.is_empty() {
                    (None, None)
                } else {
                    let (command, proof) =
                        crate::codec::decode(&entry.data).ok_or(Error::Quarantined)?;
                    (Some(command), proof)
                };
                let result = self
                    .application
                    .apply_committed(entry.index, command, readiness)
                    .map_err(|_| Error::Quarantined)?;
                result.map_or(
                    ReplayResult::Noop { index: entry.index },
                    ReplayResult::Application,
                )
            }
            EntryType::EntryConfChangeV2 => {
                let carrier =
                    ConfChangeV2::parse_from_bytes(&entry.data).map_err(|_| Error::Quarantined)?;
                let (intent, proofs) = encoding::parse_context(&entry.context)?;
                if carrier != change(&intent.change, &self.configuration) {
                    return Err(Error::Quarantined);
                }
                let refusal = self.configuration_refusal(&intent, &proofs)?;
                self.application
                    .apply_committed(entry.index, None, None)
                    .map_err(|_| Error::Quarantined)?;
                if refusal.is_none() {
                    match &intent.change {
                        Change::AddLearner { node } => {
                            if !self.configuration.learners.contains(node) {
                                self.configuration.learners.push(*node);
                                self.configuration.learners.sort_unstable();
                            }
                        }
                        Change::EnterJoint { voters } => {
                            self.configuration.voters_outgoing = self.configuration.voters.clone();
                            self.configuration.voters = voters.clone();
                            self.configuration
                                .learners
                                .retain(|node| !voters.contains(node));
                        }
                        Change::LeaveJoint => {
                            self.configuration.voters_outgoing.clear();
                        }
                    }
                    self.configuration.index = entry.index;
                    let mut voters = self.configuration.voters.clone();
                    voters.extend(&self.configuration.voters_outgoing);
                    voters.sort_unstable();
                    voters.dedup();
                    self.application.set_voters(voters);
                    accepted_change = Some(carrier);
                }
                let receipt = ConfigReceipt {
                    intent: intent.clone(),
                    outcome: refusal.map_or(ConfigOutcome::Accepted, ConfigOutcome::Refused),
                    index: entry.index,
                    configuration: self.configuration.clone(),
                };
                if self
                    .configurations
                    .insert(intent.key, receipt.clone())
                    .is_some()
                {
                    return Err(Error::Quarantined);
                }
                ReplayResult::Configuration(receipt)
            }
            _ => {
                return Err(Error::Quarantined);
            }
        };
        self.history.insert(stored.index, (stored, result.clone()));
        Ok((result, accepted_change))
    }
    fn configuration_refusal(
        &self,
        value: &ConfigIntent,
        proofs: &[PromotionProof],
    ) -> Result<Option<ConfigRejection>, Error> {
        if let Err(error) = self.admission(value) {
            return Ok(Some(match error {
                Error::Unauthorized => ConfigRejection::Unauthorized,
                Error::StaleConfiguration => ConfigRejection::StaleConfiguration,
                Error::HomeInUse => ConfigRejection::HomeInUse,
                Error::JointInProgress => ConfigRejection::JointInProgress,
                Error::NotJoint => ConfigRejection::NotJoint,
                Error::WrongBinding | Error::InvalidImage => ConfigRejection::InvalidChange,
                _ => {
                    return Err(Error::Quarantined);
                }
            }));
        }
        if let Change::EnterJoint { voters } = &value.change {
            let required: Vec<_> = voters
                .iter()
                .filter(|node| !self.configuration.voters.contains(node))
                .copied()
                .collect();
            if proofs.len() != required.len() {
                return Ok(Some(ConfigRejection::IncompleteLearner));
            }
            let evidence = self.snapshot_bytes()?;
            let term = self
                .history
                .last_key_value()
                .map_or(0, |(_, (entry, _))| entry.term);
            for (proof, node) in proofs.iter().zip(required) {
                if proof.node != node
                    || proof.index != self.applied()
                    || proof.term != term
                    || proof.state != evidence
                    || !self.configuration.learners.contains(&node)
                {
                    return Ok(Some(ConfigRejection::IncompleteLearner));
                }
            }
        } else if !proofs.is_empty() {
            return Err(Error::Quarantined);
        }
        if matches!(value.change, Change::LeaveJoint)
            && !self.application.homes_fit(&self.configuration.voters)
        {
            return Ok(Some(ConfigRejection::HomeInUse));
        }
        Ok(None)
    }
    pub(super) fn snapshot_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::new();
        bytes(&mut out, b"GQ3APP02");
        word(&mut out, 7);
        word(&mut out, 70);
        word(&mut out, 2);
        word(&mut out, self.history.len() as u64);
        for (entry, result) in self.history.values() {
            word(&mut out, entry.index);
            word(&mut out, entry.term);
            bytes(&mut out, &entry.bytes);
            result_bytes(&mut out, result);
            if out.len() > encoding::LIMIT {
                return Err(Error::CapacityExhausted);
            }
        }
        configuration_bytes(&mut out, &self.configuration);
        bytes(&mut out, &self.application.complete_evidence());
        if out.len() > encoding::LIMIT {
            return Err(Error::CapacityExhausted);
        }
        Ok(out)
    }
    pub(super) fn checkpoint(&self) -> Result<Checkpoint, Error> {
        let (index, (entry, _)) = self.history.last_key_value().ok_or(Error::Missing)?;
        Ok(Checkpoint {
            binding: GroupIdentity {
                scope: 7,
                group: 70,
                application_profile: 2,
            },
            index: *index,
            term: entry.term,
            configuration: self.configuration.clone(),
            application: self.snapshot_bytes()?,
        })
    }
    pub(super) fn restore(cp: &Checkpoint) -> Result<Self, Error> {
        if cp.index == u64::MAX || cp.term == u64::MAX || cp.application.len() > encoding::LIMIT {
            return Err(Error::CapacityExhausted);
        }
        if cp.binding
            != (GroupIdentity {
                scope: 7,
                group: 70,
                application_profile: 2,
            })
        {
            return Err(Error::WrongBinding);
        }
        let mut input = Reader(&cp.application);
        if input.bytes()? != b"GQ3APP02"
            || [input.word()?, input.word()?, input.word()?] != [7, 70, 2]
        {
            return Err(Error::Quarantined);
        }
        let count = input.word()?;
        if count != cp.index || count > input.0.len() as u64 / 24 {
            return Err(Error::Quarantined);
        }
        let mut machine = Self::new();
        for _ in 0..count {
            let entry = StoredEntry {
                index: input.word()?,
                term: input.word()?,
                bytes: input.bytes()?,
            };
            let (result, _) = machine.apply(entry)?;
            let mut expected = Vec::new();
            result_bytes(&mut expected, &result);
            if input.take(expected.len())? != expected {
                return Err(Error::Quarantined);
            }
        }
        let mut expected = Vec::new();
        configuration_bytes(&mut expected, &machine.configuration);
        bytes(&mut expected, &machine.application.complete_evidence());
        if input.0 != expected
            || machine.configuration != cp.configuration
            || machine
                .history
                .last_key_value()
                .map(|(_, (entry, _))| entry.term)
                != Some(cp.term)
        {
            return Err(Error::Quarantined);
        }
        Ok(machine)
    }
}
pub(super) fn configuration_bytes(out: &mut Vec<u8>, value: &Configuration) {
    for list in [
        &value.voters,
        &value.learners,
        &value.voters_outgoing,
        &value.learners_next,
    ] {
        word(out, list.len() as u64);
        for node in list {
            word(out, *node);
        }
    }
    word(out, u64::from(value.auto_leave));
    word(out, value.index);
}
pub(super) fn result_bytes(out: &mut Vec<u8>, value: &ReplayResult) {
    match value {
        ReplayResult::Noop { index } => {
            word(out, 0);
            word(out, *index);
        }
        ReplayResult::Application(receipt) => {
            word(out, 1);
            q3_evidence::receipt(out, *receipt);
        }
        ReplayResult::Configuration(receipt) => {
            word(out, 2);
            encoding::intent(out, &receipt.intent);
            word(out, receipt.index);
            match receipt.outcome {
                ConfigOutcome::Accepted => {
                    word(out, 0);
                }
                ConfigOutcome::Refused(reason) => {
                    word(out, 1);
                    word(out, reason as u64);
                }
            }
            configuration_bytes(out, &receipt.configuration);
        }
    }
}
