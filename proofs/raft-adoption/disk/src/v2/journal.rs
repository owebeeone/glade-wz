use super::{codec, validation};
use glade_raft_q3_api::{Error, Instance, State};
use std::io::Read;
pub(super) fn recover(
    reader: &mut impl Read,
    expected: &Instance,
    floor: Option<u64>,
) -> Result<State, Error> {
    let mut prior: Option<State> = None;
    while let Some(state) = codec::read_record(reader)? {
        if !validation::instance_valid(&state.instance)
            || validation::validate(&state.instance, &state.image).is_err()
        {
            return Err(Error::Quarantined);
        }
        if let Some(previous) = &prior {
            if state.instance != previous.instance
                || previous.revision.checked_add(1) != Some(state.revision)
                || validation::transition(&state.instance, &previous.image, &state.image).is_err()
            {
                return Err(Error::Quarantined);
            }
        } else {
            if state.revision != 0 {
                return Err(Error::Quarantined);
            }
            if state.instance != *expected {
                return Err(Error::WrongBinding);
            }
        }
        prior = Some(state);
    }
    let state = prior.ok_or(Error::Quarantined)?;
    if floor.is_some_and(|floor| state.revision < floor) {
        return Err(Error::Quarantined);
    }
    Ok(state)
}
