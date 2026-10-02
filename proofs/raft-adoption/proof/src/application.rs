use std::collections::BTreeMap;

use glade_raft_adoption_api::{
    Action, ApplyError, Command, CommittedMachine, Outcome, Receipt, Rejection, RequestId, Resource,
};

/// Driver-owned evidence, never minted from a public frontier field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Readiness {
    pub(crate) home: u64,
    pub(crate) frontier: u64,
}

#[derive(Clone, Copy)]
struct Applied {
    command: Option<Command>,
    readiness: Option<Readiness>,
    receipt: Option<Receipt>,
}

/// Deterministic committed-log host with complete retained numeric data.
/// This bounded experiment uses memory and unbounded retained evidence;
/// capacity exhaustion, compaction and durable recovery remain unqualified.
pub struct Application {
    voters: Vec<u64>,
    applied: u64,
    history: BTreeMap<u64, Applied>,
    resources: BTreeMap<u64, Resource>,
    names: BTreeMap<u64, u64>,
    outcomes: BTreeMap<RequestId, (Command, Receipt)>,
    replies: Vec<(Command, Receipt)>,
    permissions: BTreeMap<(u64, u64, u64), (bool, bool)>,
    policy_frontier: u64,
}

impl Application {
    pub fn new() -> Self {
        Self::with_voters(&[1, 2, 3])
    }

    pub(crate) fn with_voters(voters: &[u64]) -> Self {
        Self {
            voters: voters.to_vec(),
            applied: 0,
            history: BTreeMap::new(),
            resources: BTreeMap::new(),
            names: BTreeMap::new(),
            outcomes: BTreeMap::new(),
            replies: Vec::new(),
            permissions: BTreeMap::new(),
            policy_frontier: 0,
        }
    }

    pub(crate) fn applied(&self) -> u64 {
        self.applied
    }

    pub(crate) fn policy_frontier(&self) -> u64 {
        self.policy_frontier
    }

    pub(crate) fn resource(&self, id: u64) -> Option<Resource> {
        self.resources.get(&id).copied()
    }

    pub(crate) fn may_disclose(&self, request: RequestId) -> bool {
        self.permission(request).1
    }

    pub(crate) fn reply(&self, command: Command) -> Option<Receipt> {
        self.replies.iter().rev().find_map(|(attempt, receipt)| {
            if *attempt == command {
                Some(*receipt)
            } else {
                None
            }
        })
    }

    fn permission(&self, request: RequestId) -> (bool, bool) {
        self.permissions
            .get(&(request.resource, request.incarnation, request.principal))
            .copied()
            .unwrap_or_else(|| {
                let allowed = matches!(request.principal, 1 | 10);
                (allowed, allowed)
            })
    }

    pub(crate) fn apply_committed(
        &mut self,
        index: u64,
        command: Option<Command>,
        readiness: Option<Readiness>,
    ) -> Result<Option<Receipt>, ApplyError> {
        if let Some(previous) = self.history.get(&index) {
            if previous.command != command || previous.readiness != readiness {
                return Err(ApplyError::ConflictingReplay { index });
            }
            return Ok(previous.receipt);
        }
        let expected = self.applied + 1;
        if index != expected {
            return Err(ApplyError::IndexGap {
                expected,
                received: index,
            });
        }
        let receipt = command.map(|command| {
            let receipt = if let Some((original, receipt)) = self.outcomes.get(&command.request) {
                if *original == command {
                    *receipt
                } else {
                    Receipt {
                        request: command.request,
                        index,
                        outcome: Outcome::Rejected(Rejection::RetryConflict),
                    }
                }
            } else {
                let outcome = match self.execute(index, command, readiness) {
                    Ok(resource) => Outcome::Accepted(resource),
                    Err(reason) => Outcome::Rejected(reason),
                };
                let receipt = Receipt {
                    request: command.request,
                    index,
                    outcome,
                };
                self.outcomes.insert(command.request, (command, receipt));
                receipt
            };
            self.replies.push((command, receipt));
            receipt
        });
        self.history.insert(
            index,
            Applied {
                command,
                readiness,
                receipt,
            },
        );
        self.applied = index;
        Ok(receipt)
    }

    fn execute(
        &mut self,
        index: u64,
        command: Command,
        readiness: Option<Readiness>,
    ) -> Result<Resource, Rejection> {
        if command.request.scope != 7 {
            return Err(Rejection::WrongScope);
        }
        if matches!(command.action, Action::ExternalEffect { .. }) {
            return Err(Rejection::UnsupportedEffect);
        }
        if let Action::Create {
            name,
            home,
            payload,
        } = command.action
        {
            return self.create(command, name, home, payload);
        }
        let mut resource = self
            .resource(command.request.resource)
            .ok_or(Rejection::UnknownResource)?;
        if resource.incarnation != command.request.incarnation {
            return Err(Rejection::IncarnationConflict);
        }
        if resource.retired {
            return Err(Rejection::Retired);
        }
        if resource.generation != command.generation {
            return Err(Rejection::StaleGeneration);
        }
        if resource.home != command.home {
            return Err(Rejection::WrongHome);
        }
        if command.policy_frontier != self.policy_frontier {
            return Err(Rejection::PolicyFrontier);
        }
        let administrator = command.request.principal == 1;
        match command.action {
            Action::Mutate { payload } => {
                if !self.permission(command.request).0 {
                    return Err(Rejection::Unauthorized);
                }
                resource.payload = payload;
            }
            Action::Move {
                home,
                successor_applied,
            } => {
                if !administrator {
                    return Err(Rejection::Unauthorized);
                }
                let witness = readiness.ok_or(Rejection::IncompleteSuccessor)?;
                if !self.voters.contains(&home)
                    || witness.home != home
                    || Some(witness.frontier) != successor_applied
                    || witness.frontier != index - 1
                {
                    return Err(Rejection::IncompleteSuccessor);
                }
                resource.home = home;
                resource.generation += 1;
            }
            Action::Retire => {
                if !administrator {
                    return Err(Rejection::Unauthorized);
                }
                resource.retired = true;
            }
            Action::SetPermission {
                principal,
                write,
                disclose,
            } => {
                if !administrator {
                    return Err(Rejection::Unauthorized);
                }
                self.permissions.insert(
                    (resource.id, resource.incarnation, principal),
                    (write, disclose),
                );
                self.policy_frontier = index;
            }
            Action::Create { .. } | Action::ExternalEffect { .. } => unreachable!(),
        }
        self.resources.insert(resource.id, resource);
        Ok(resource)
    }

    fn create(
        &mut self,
        command: Command,
        name: u64,
        home: u64,
        payload: u64,
    ) -> Result<Resource, Rejection> {
        if let Some(resource) = self.resource(command.request.resource) {
            if resource.incarnation != command.request.incarnation {
                return Err(Rejection::IncarnationConflict);
            }
            if resource.retired {
                return Err(Rejection::Retired);
            }
            return Err(Rejection::NameConflict);
        }
        if command.request.principal != 1 {
            return Err(Rejection::Unauthorized);
        }
        if command.policy_frontier != self.policy_frontier {
            return Err(Rejection::PolicyFrontier);
        }
        if command.generation != 0 {
            return Err(Rejection::StaleGeneration);
        }
        if command.home != 0 || !self.voters.contains(&home) {
            return Err(Rejection::WrongHome);
        }
        if self.names.contains_key(&name) {
            return Err(Rejection::NameConflict);
        }
        let resource = Resource {
            id: command.request.resource,
            name,
            incarnation: command.request.incarnation,
            generation: 1,
            home,
            payload,
            retired: false,
        };
        self.names.insert(name, resource.id);
        self.resources.insert(resource.id, resource);
        Ok(resource)
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl CommittedMachine for Application {
    fn apply(
        &mut self,
        index: u64,
        command: Option<Command>,
    ) -> Result<Option<Receipt>, ApplyError> {
        self.apply_committed(index, command, None)
    }

    fn lookup(&self, request: RequestId) -> Option<Receipt> {
        self.outcomes.get(&request).map(|(_, receipt)| *receipt)
    }
}
