use glade_raft_durability_api::{Binding, DurableImage, StoreError, StoredState, conformance};

// Contract-faithful local model; not evidence of filesystem durability.
struct Model(StoredState);

impl glade_raft_durability_api::DurableStore for Model {
    fn load(&mut self) -> Result<StoredState, StoreError> {
        Ok(self.0.clone())
    }

    fn persist(
        &mut self,
        expected_revision: u64,
        image: DurableImage,
    ) -> Result<StoredState, StoreError> {
        if expected_revision != self.0.revision {
            return Err(StoreError::RevisionConflict {
                expected: expected_revision,
                actual: self.0.revision,
            });
        }
        let prior = &self.0.image;
        if image.term < prior.term
            || image.commit < prior.commit
            || image.commit > image.entries.len() as u64
            || (image.vote != 0 && !self.0.binding.voters.contains(&image.vote))
            || (image.term == prior.term && prior.vote != 0 && image.vote != prior.vote)
            || (image.term == 0 && (image.vote != 0 || !image.entries.is_empty()))
        {
            return Err(StoreError::InvalidImage);
        }
        let mut previous_term = 0;
        for (offset, entry) in image.entries.iter().enumerate() {
            if entry.index != offset as u64 + 1
                || entry.term == 0
                || entry.term < previous_term
                || entry.term > image.term
                || entry.bytes.is_empty()
            {
                return Err(StoreError::InvalidImage);
            }
            previous_term = entry.term;
        }
        if image.entries.get(..prior.commit as usize) != prior.entries.get(..prior.commit as usize)
        {
            return Err(StoreError::InvalidImage);
        }
        self.0.revision = self
            .0
            .revision
            .checked_add(1)
            .ok_or(StoreError::CapacityExhausted)?;
        self.0.image = image;
        Ok(self.0.clone())
    }
}

#[test]
fn dyn_contract_success_failure_and_edge_cases_against_model() {
    let mut model = Model(StoredState {
        revision: 0,
        binding: Binding {
            scope: 7,
            node: 1,
            voters: vec![1, 2, 3],
            application_profile: 1,
        },
        image: DurableImage::default(),
    });
    conformance::exercise_store(&mut model);
}
