//! Q3 fixture compatibility only; existing Q1a application semantics are retained.
use glade_raft_adoption_api::{CommittedMachine, Outcome, Rejection, Resource};
use glade_raft_adoption_proof::Application;
use glade_raft_q3_api::conformance;

#[test]
fn corrected_q3_create_is_accepted_by_the_existing_application() {
    let mut application = Application::new();
    let command = conformance::create();
    let receipt = application
        .apply(1, Some(command))
        .expect("ordered create")
        .expect("receipt");
    assert_eq!(receipt.request, command.request);
    assert_eq!(receipt.index, 1);
    assert_eq!(
        receipt.outcome,
        Outcome::Accepted(Resource {
            id: 100,
            name: 40,
            incarnation: 1,
            generation: 1,
            home: 1,
            payload: 11,
            retired: false,
        })
    );
    assert_eq!(application.lookup(command.request), Some(receipt));
}

#[test]
fn nonzero_create_generation_and_home_keep_the_original_refusals() {
    for (generation, home, rejection) in [
        (1, 0, Rejection::StaleGeneration),
        (0, 1, Rejection::WrongHome),
    ] {
        let mut application = Application::new();
        let mut command = conformance::create();
        command.generation = generation;
        command.home = home;
        let receipt = application
            .apply(1, Some(command))
            .expect("ordered refusal")
            .expect("receipt");
        assert_eq!(receipt.outcome, Outcome::Rejected(rejection));
        assert_eq!(application.lookup(command.request), Some(receipt));
    }
}
