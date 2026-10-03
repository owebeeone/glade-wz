use glade_raft_q3_api::{Configuration, Error, Image, Instance};

pub(super) fn instance_valid(value: &Instance) -> bool {
    value.scope == 7
        && value.group == 70
        && (1..=4).contains(&value.node)
        && value.application_profile == 2
}

fn canonical(nodes: &[u64]) -> bool {
    nodes.len() <= 4
        && nodes.iter().all(|node| (1..=4).contains(node))
        && nodes.windows(2).all(|pair| pair[0] < pair[1])
}

pub(super) fn configuration_valid(value: &Configuration) -> bool {
    let lists = [
        &value.voters,
        &value.learners,
        &value.voters_outgoing,
        &value.learners_next,
    ];
    !value.voters.is_empty()
        && lists.into_iter().all(|list| canonical(list))
        && !value.auto_leave
        && value.learners_next.is_empty()
        && value
            .learners
            .iter()
            .all(|id| !value.voters.contains(id) && !value.voters_outgoing.contains(id))
}

pub(super) fn image_valid(instance: &Instance, image: &Image) -> bool {
    if !configuration_valid(&image.configuration)
        || image.configuration.index > image.applied
        || image.vote > 4
        || image.applied > image.commit
    {
        return false;
    }
    let (cut, mut term) = if let Some(checkpoint) = &image.checkpoint {
        if checkpoint.binding.scope != instance.scope
            || checkpoint.binding.group != instance.group
            || checkpoint.binding.application_profile != instance.application_profile
            || checkpoint.index == 0
            || checkpoint.term == 0
            || checkpoint.term > image.term
            || !configuration_valid(&checkpoint.configuration)
            || checkpoint.configuration.index > checkpoint.index
            || checkpoint.application.is_empty()
            || checkpoint.index > image.applied
        {
            return false;
        }
        (checkpoint.index, checkpoint.term)
    } else {
        (0, 0)
    };
    let Some(last) = cut.checked_add(image.suffix.len() as u64) else {
        return false;
    };
    if image.commit > last
        || (image.term == 0 && (image.vote != 0 || last != 0 || image.configuration.index != 0))
    {
        return false;
    }
    let mut index = cut;
    for entry in &image.suffix {
        let Some(next) = index.checked_add(1) else {
            return false;
        };
        if entry.index != next
            || entry.term == 0
            || entry.term < term
            || entry.term > image.term
            || entry.bytes.is_empty()
        {
            return false;
        }
        index = next;
        term = entry.term;
    }
    true
}

pub(super) fn validate(instance: &Instance, image: &Image) -> Result<(), Error> {
    if image.term == u64::MAX
        || image.configuration.index == u64::MAX
        || image
            .checkpoint
            .as_ref()
            .is_some_and(|cp| cp.index == u64::MAX || cp.term == u64::MAX)
    {
        return Err(Error::CapacityExhausted);
    }
    if !image_valid(instance, image) {
        return Err(Error::InvalidImage);
    }
    Ok(())
}

pub(super) fn transition(instance: &Instance, prior: &Image, next: &Image) -> Result<(), Error> {
    validate(instance, next)?;
    let before = prior.checkpoint.as_ref().map_or(0, |cp| cp.index);
    let after = next.checkpoint.as_ref().map_or(0, |cp| cp.index);
    if next.term < prior.term
        || next.commit < prior.commit
        || next.applied < prior.applied
        || next.configuration.index < prior.configuration.index
        || after < before
        || (next.term == prior.term && prior.vote != 0 && next.vote != prior.vote)
        || (next.configuration.index == prior.configuration.index
            && next.configuration != prior.configuration)
        || (before > 0 && before == after && prior.checkpoint != next.checkpoint)
    {
        return Err(Error::InvalidImage);
    }
    if let Some(checkpoint) = &next.checkpoint {
        let committed_term = prior
            .suffix
            .iter()
            .find(|entry| entry.index == prior.commit)
            .map(|entry| entry.term)
            .or_else(|| prior.checkpoint.as_ref().map(|cp| cp.term))
            .unwrap_or(0);
        if (checkpoint.index > prior.commit && checkpoint.term < committed_term)
            || prior
                .checkpoint
                .as_ref()
                .is_some_and(|old| checkpoint.term < old.term)
        {
            return Err(Error::InvalidImage);
        }
        if checkpoint.index <= prior.commit
            && let Some(entry) = prior
                .suffix
                .iter()
                .find(|entry| entry.index == checkpoint.index)
            && checkpoint.term != entry.term
        {
            return Err(Error::InvalidImage);
        }
    }
    for entry in prior
        .suffix
        .iter()
        .filter(|entry| entry.index <= prior.commit && entry.index > after)
    {
        let offset =
            usize::try_from(entry.index - after - 1).map_err(|_| Error::CapacityExhausted)?;
        if next.suffix.get(offset) != Some(entry) {
            return Err(Error::InvalidImage);
        }
    }
    Ok(())
}
