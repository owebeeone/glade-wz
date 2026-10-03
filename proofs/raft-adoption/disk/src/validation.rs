use glade_raft_durability_api::{Binding, DurableImage, StoreError};

pub(crate) fn binding_valid(binding: &Binding) -> bool {
    binding.scope == 7
        && binding.application_profile == 1
        && (binding.voters == [1, 2] || binding.voters == [1, 2, 3])
        && binding.voters.contains(&binding.node)
}

pub(crate) fn image_valid(binding: &Binding, image: &DurableImage) -> bool {
    if image.commit > image.entries.len() as u64
        || (image.vote != 0 && !binding.voters.contains(&image.vote))
        || (image.term == 0 && (image.vote != 0 || !image.entries.is_empty()))
    {
        return false;
    }
    let mut previous_term = 0;
    for (offset, entry) in image.entries.iter().enumerate() {
        if entry.index != offset as u64 + 1
            || entry.term == 0
            || entry.term < previous_term
            || entry.term > image.term
            || entry.bytes.is_empty()
        {
            return false;
        }
        previous_term = entry.term;
    }
    true
}

pub(crate) fn transition(
    binding: &Binding,
    prior: &DurableImage,
    next: &DurableImage,
) -> Result<(), StoreError> {
    if !image_valid(binding, next)
        || next.term < prior.term
        || next.commit < prior.commit
        || (next.term == prior.term && prior.vote != 0 && next.vote != prior.vote)
        || next.entries.get(..prior.commit as usize) != prior.entries.get(..prior.commit as usize)
    {
        return Err(StoreError::InvalidImage);
    }
    Ok(())
}
