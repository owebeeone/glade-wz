//! Canonical full materialized evidence for the private Q3 semantic checkpoint.
use super::Application;
use crate::q3::encoding::{bytes, word};
use glade_raft_adoption_api::{Outcome, Receipt, Resource};

pub(crate) fn resource(out: &mut Vec<u8>, value: Resource) {
    for value in [
        value.id,
        value.name,
        value.incarnation,
        value.generation,
        value.home,
        value.payload,
        u64::from(value.retired),
    ] {
        word(out, value);
    }
}
pub(crate) fn receipt(out: &mut Vec<u8>, value: Receipt) {
    for value in [
        value.request.scope,
        value.request.resource,
        value.request.incarnation,
        value.request.principal,
        value.request.sequence,
        value.index,
    ] {
        word(out, value);
    }
    match value.outcome {
        Outcome::Accepted(value) => {
            word(out, 1);
            resource(out, value);
        }
        Outcome::Rejected(value) => {
            word(out, 2);
            word(out, value as u64);
        }
    }
}
impl Application {
    pub(crate) fn set_voters(&mut self, voters: Vec<u64>) {
        self.voters = voters;
    }
    pub(crate) fn homes_fit(&self, voters: &[u64]) -> bool {
        self.resources
            .values()
            .all(|resource| resource.retired || voters.contains(&resource.home))
    }
    pub(crate) fn original_command(
        &self,
        request: glade_raft_adoption_api::RequestId,
    ) -> Option<glade_raft_adoption_api::Command> {
        self.outcomes.get(&request).map(|(command, _)| *command)
    }
    pub(crate) fn complete_evidence(&self) -> Vec<u8> {
        let mut out = Vec::new();
        word(&mut out, 2);
        word(&mut out, self.applied);
        word(&mut out, self.policy_frontier);
        word(&mut out, self.voters.len() as u64);
        for voter in &self.voters {
            word(&mut out, *voter);
        }
        word(&mut out, self.resources.len() as u64);
        for value in self.resources.values() {
            resource(&mut out, *value);
        }
        word(&mut out, self.names.len() as u64);
        for (name, id) in &self.names {
            word(&mut out, *name);
            word(&mut out, *id);
        }
        word(&mut out, self.outcomes.len() as u64);
        for (command, result) in self.outcomes.values() {
            bytes(&mut out, &crate::codec::encode(*command, None));
            receipt(&mut out, *result);
        }
        word(&mut out, self.permissions.len() as u64);
        for ((id, incarnation, principal), (write, disclose)) in &self.permissions {
            for value in [
                *id,
                *incarnation,
                *principal,
                u64::from(*write),
                u64::from(*disclose),
            ] {
                word(&mut out, value);
            }
        }
        word(&mut out, self.replies.len() as u64);
        for (command, result) in &self.replies {
            bytes(&mut out, &crate::codec::encode(*command, None));
            receipt(&mut out, *result);
        }
        out
    }
}
