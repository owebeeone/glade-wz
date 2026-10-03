use crate::{codec, validation};
use glade_raft_durability_api::{Binding, DurableImage, StoreError, StoredState};
use std::io::Read;

pub(crate) fn recover(
    reader: &mut impl Read,
    expected: &Binding,
    minimum_revision: Option<u64>,
) -> Result<StoredState, StoreError> {
    let mut prior: Option<StoredState> = None;
    while let Some(state) = codec::read_record(reader)? {
        if !validation::binding_valid(&state.binding)
            || !validation::image_valid(&state.binding, &state.image)
        {
            return Err(StoreError::Quarantined);
        }
        if let Some(previous) = &prior {
            if state.binding != previous.binding
                || previous.revision.checked_add(1) != Some(state.revision)
                || validation::transition(&state.binding, &previous.image, &state.image).is_err()
            {
                return Err(StoreError::Quarantined);
            }
        } else {
            if state.revision != 0 || state.image != DurableImage::default() {
                return Err(StoreError::Quarantined);
            }
            if state.binding != *expected {
                return Err(StoreError::BindingMismatch);
            }
        }
        prior = Some(state);
    }
    let state = prior.ok_or(StoreError::Quarantined)?;
    if minimum_revision.is_some_and(|floor| state.revision < floor) {
        return Err(StoreError::Quarantined);
    }
    Ok(state)
}
