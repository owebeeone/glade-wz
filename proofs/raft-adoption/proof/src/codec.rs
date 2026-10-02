//! Private complete-payload fixture encoding; not a Glade wire protocol.
use crate::application::Readiness;
use glade_raft_adoption_api::{Action, Command, RequestId};

pub(crate) fn encode(command: Command, readiness: Option<Readiness>) -> Vec<u8> {
    let mut data = vec![1];
    for word in [
        command.request.scope,
        command.request.resource,
        command.request.incarnation,
        command.request.principal,
        command.request.sequence,
        command.generation,
        command.home,
        command.policy_frontier,
    ] {
        data.extend_from_slice(&word.to_le_bytes());
    }
    let words = match command.action {
        Action::Create {
            name,
            home,
            payload,
        } => {
            data.push(1);
            vec![name, home, payload]
        }
        Action::Mutate { payload } => {
            data.push(2);
            vec![payload]
        }
        Action::Move {
            home,
            successor_applied,
        } => {
            data.push(3);
            data.push(u8::from(successor_applied.is_some()));
            vec![home, successor_applied.unwrap_or(0)]
        }
        Action::Retire => {
            data.push(4);
            Vec::new()
        }
        Action::SetPermission {
            principal,
            write,
            disclose,
        } => {
            data.push(5);
            data.push(u8::from(write));
            data.push(u8::from(disclose));
            vec![principal]
        }
        Action::ExternalEffect { code } => {
            data.push(6);
            vec![code]
        }
    };
    for word in words {
        data.extend_from_slice(&word.to_le_bytes());
    }
    data.push(u8::from(readiness.is_some()));
    if let Some(witness) = readiness {
        data.extend_from_slice(&witness.home.to_le_bytes());
        data.extend_from_slice(&witness.frontier.to_le_bytes());
    }
    data
}

struct Decoder<'a>(&'a [u8]);

impl Decoder<'_> {
    fn byte(&mut self) -> Option<u8> {
        let (first, tail) = self.0.split_first()?;
        self.0 = tail;
        Some(*first)
    }

    fn boolean(&mut self) -> Option<bool> {
        match self.byte()? {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    fn word(&mut self) -> Option<u64> {
        let bytes: [u8; 8] = self.0.get(..8)?.try_into().ok()?;
        self.0 = &self.0[8..];
        Some(u64::from_le_bytes(bytes))
    }
}

pub(crate) fn decode(data: &[u8]) -> Option<(Command, Option<Readiness>)> {
    let mut input = Decoder(data);
    if input.byte()? != 1 {
        return None;
    }
    let request = RequestId {
        scope: input.word()?,
        resource: input.word()?,
        incarnation: input.word()?,
        principal: input.word()?,
        sequence: input.word()?,
    };
    let generation = input.word()?;
    let home = input.word()?;
    let policy_frontier = input.word()?;
    let action = match input.byte()? {
        1 => Action::Create {
            name: input.word()?,
            home: input.word()?,
            payload: input.word()?,
        },
        2 => Action::Mutate {
            payload: input.word()?,
        },
        3 => {
            let present = input.boolean()?;
            let home = input.word()?;
            let frontier = input.word()?;
            if !present && frontier != 0 {
                return None;
            }
            Action::Move {
                home,
                successor_applied: present.then_some(frontier),
            }
        }
        4 => Action::Retire,
        5 => {
            let write = input.boolean()?;
            let disclose = input.boolean()?;
            Action::SetPermission {
                principal: input.word()?,
                write,
                disclose,
            }
        }
        6 => Action::ExternalEffect {
            code: input.word()?,
        },
        _ => return None,
    };
    let readiness = if input.boolean()? {
        Some(Readiness {
            home: input.word()?,
            frontier: input.word()?,
        })
    } else {
        None
    };
    if !input.0.is_empty() {
        return None;
    }
    Some((
        Command {
            request,
            generation,
            home,
            policy_frontier,
            action,
        },
        readiness,
    ))
}

mod tests {
    #[test]
    fn complete_commands_and_private_evidence_round_trip() {
        use super::*;

        let fixture = |action| Command {
            request: RequestId {
                scope: 7,
                resource: 100,
                incarnation: 1,
                principal: 1,
                sequence: 9,
            },
            generation: 4,
            home: 3,
            policy_frontier: 19,
            action,
        };
        let actions = [
            Action::Create {
                name: 40,
                home: 1,
                payload: u64::MAX,
            },
            Action::Mutate { payload: 123 },
            Action::Move {
                home: 2,
                successor_applied: Some(21),
            },
            Action::Move {
                home: 2,
                successor_applied: None,
            },
            Action::Retire,
            Action::SetPermission {
                principal: 10,
                write: false,
                disclose: true,
            },
            Action::ExternalEffect { code: 33 },
        ];
        for action in actions {
            for evidence in [
                None,
                Some(Readiness {
                    home: 2,
                    frontier: 21,
                }),
            ] {
                let command = fixture(action);
                assert_eq!(
                    decode(&encode(command, evidence)),
                    Some((command, evidence))
                );
            }
        }
    }

    #[test]
    fn malformed_or_noncanonical_entries_are_not_silently_applied() {
        use super::*;

        let fixture = |action| Command {
            request: RequestId {
                scope: 7,
                resource: 100,
                incarnation: 1,
                principal: 1,
                sequence: 9,
            },
            generation: 4,
            home: 3,
            policy_frontier: 19,
            action,
        };
        let command = fixture(Action::Mutate { payload: 456 });
        let bytes = encode(command, None);
        assert!(decode(&bytes).is_some());
        for length in 0..bytes.len() {
            assert!(decode(&bytes[..length]).is_none());
        }
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert!(decode(&trailing).is_none());
        let mut version = bytes.clone();
        version[0] = 255;
        assert!(decode(&version).is_none());
        let mut action = bytes.clone();
        action[65] = 255;
        assert!(decode(&action).is_none());
        let mut boolean = encode(
            fixture(Action::Move {
                home: 2,
                successor_applied: None,
            }),
            None,
        );
        boolean[66] = 2;
        assert!(decode(&boolean).is_none());
    }
}
