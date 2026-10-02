//! Q0 specifications for a future real-driver Q1 witness. Numeric trust fixtures
//! do not establish authentication, canonical Glade bytes, or physical durability.

use glade_raft_adoption_api::{
    Action, ApplyError, Command, CommittedMachine, Outcome, Receipt, Rejection, RequestId,
};
use glade_raft_adoption_proof::{Application, Cluster, Proposal};

fn request(principal: u64, sequence: u64) -> RequestId {
    RequestId {
        scope: 7,
        resource: 100,
        incarnation: 1,
        principal,
        sequence,
    }
}

fn create(sequence: u64) -> Command {
    Command {
        request: request(1, sequence),
        generation: 0,
        home: 0,
        policy_frontier: 0,
        action: Action::Create {
            name: 40,
            home: 1,
            payload: 11,
        },
    }
}

fn mutate(sequence: u64, payload: u64) -> Command {
    Command {
        request: request(10, sequence),
        generation: 1,
        home: 1,
        policy_frontier: 0,
        action: Action::Mutate { payload },
    }
}

fn cluster() -> Cluster {
    let mut cluster = Cluster::new(&[1, 2, 3]);
    cluster.campaign(1);
    cluster.drain();
    cluster
}

fn committed(cluster: &mut Cluster, voter: u64, command: Command) -> Receipt {
    let _ = cluster.propose(voter, command);
    cluster.drain();
    let receipt = cluster.reply(voter, command);
    assert!(
        receipt.is_some(),
        "RA-003: proposal needs a retained applied outcome"
    );
    receipt.unwrap()
}

fn accepted(receipt: Receipt) {
    assert!(
        matches!(receipt.outcome, Outcome::Accepted(_)),
        "{receipt:?}"
    );
}

#[test]
fn ra002_concurrent_canonical_name_conflict_is_ordered() {
    let mut cluster = cluster();
    let first = create(1);
    let mut second = create(2);
    second.request.resource = 200;
    cluster.propose(1, first);
    cluster.propose(1, second);
    cluster.drain();
    let first_outcome = cluster.outcome(1, first.request);
    let second_outcome = cluster.outcome(1, second.request);
    assert!(matches!(
        first_outcome,
        Some(Receipt {
            outcome: Outcome::Accepted(_),
            ..
        })
    ));
    assert!(matches!(
        second_outcome,
        Some(Receipt {
            outcome: Outcome::Rejected(Rejection::NameConflict),
            ..
        })
    ));
    assert_eq!(cluster.resource(1, 100).unwrap().payload, 11);
    assert!(cluster.resource(1, 200).is_none());
}

#[test]
fn ra002_leader_change_preserves_home_and_generation() {
    let mut cluster = cluster();
    accepted(committed(&mut cluster, 1, create(1)));
    let before = cluster.resource(1, 100).unwrap();
    cluster.isolate(1);
    cluster.campaign(2);
    cluster.drain();
    assert_eq!(cluster.leader(), Some(2));
    let after = cluster.resource(2, 100).unwrap();
    assert_eq!(
        (after.home, after.generation, after.payload),
        (before.home, before.generation, before.payload)
    );
}

#[test]
fn ra004_lost_reply_exact_retry_after_failover_recovers_original_receipt() {
    let mut cluster = cluster();
    accepted(committed(&mut cluster, 1, create(1)));
    let command = mutate(1, 23);
    // Do not consume the application response; recover retained state instead.
    cluster.propose(1, command);
    cluster.drain();
    assert!(cluster.outcome(1, command.request).is_some());
    let original = cluster.outcome(1, command.request).unwrap();
    cluster.isolate(1);
    cluster.campaign(2);
    cluster.drain();
    let retry = committed(&mut cluster, 2, command);
    assert_eq!(
        retry, original,
        "RA-004: exact retry retains the original index/outcome"
    );
    assert_eq!(cluster.resource(2, 100).unwrap().payload, 23);
}

#[test]
fn ra004_changed_retry_conflicts_and_principal_namespace_does_not_alias() {
    let setup = create(1);
    let user_command = mutate(2, 23);
    let changed_command = mutate(2, 24);
    let mut admin_command = mutate(2, 25);
    admin_command.request.principal = 1;
    assert_eq!(user_command.request, changed_command.request);
    assert_ne!(user_command.request, admin_command.request);
    assert_eq!(
        admin_command.request,
        RequestId {
            principal: 1,
            ..user_command.request
        }
    );
    assert_ne!(user_command.request, setup.request);
    assert_ne!(admin_command.request, setup.request);
    let mut cluster = cluster();
    accepted(committed(&mut cluster, 1, setup));
    let original = committed(&mut cluster, 1, user_command);
    accepted(original);
    let changed = committed(&mut cluster, 1, changed_command);
    assert_eq!(changed.outcome, Outcome::Rejected(Rejection::RetryConflict));
    assert_eq!(cluster.outcome(1, original.request), Some(original));
    accepted(committed(&mut cluster, 1, admin_command));
    assert_eq!(cluster.resource(1, 100).unwrap().payload, 25);
}

#[test]
fn ra004_reusing_create_identity_for_mutation_conflicts_without_overwriting_history() {
    let setup = create(1);
    let mut collision = mutate(1, 99);
    collision.request.principal = 1;
    assert_eq!(collision.request, setup.request);
    let mut cluster = cluster();
    let original = committed(&mut cluster, 1, setup);
    accepted(original);
    let refused = committed(&mut cluster, 1, collision);
    assert_eq!(refused.outcome, Outcome::Rejected(Rejection::RetryConflict));
    assert_eq!(cluster.resource(1, 100).unwrap().payload, 11);
    assert_eq!(cluster.outcome(1, setup.request), Some(original));
}

#[test]
fn ra003_ra005_isolated_old_leader_never_accepts_and_majority_move_fences_suffix() {
    let mut cluster = cluster();
    accepted(committed(&mut cluster, 1, create(1)));
    let pre_cut = mutate(1, 17);
    let retained = committed(&mut cluster, 1, pre_cut);
    accepted(retained);
    let old = mutate(2, 99);
    cluster.isolate(1);
    assert_eq!(cluster.propose(1, old), Proposal::Unknown);
    cluster.drain();
    assert!(cluster.outcome(1, old.request).is_none());
    cluster.campaign(2);
    cluster.drain();
    let moved = cluster.move_command(request(1, 2), 1, 1, 2);
    accepted(committed(&mut cluster, 2, moved));
    assert_eq!(committed(&mut cluster, 2, pre_cut), retained);
    cluster.heal();
    cluster.drain();
    let stale = committed(&mut cluster, 2, old);
    assert_eq!(stale.outcome, Outcome::Rejected(Rejection::StaleGeneration));
    for voter in [1, 2, 3] {
        let resource = cluster.resource(voter, 100).unwrap();
        assert_eq!(
            (resource.home, resource.generation, resource.payload),
            (2, 2, 17)
        );
    }
    let mut current = mutate(3, 31);
    current.generation = 2;
    current.home = 2;
    accepted(committed(&mut cluster, 2, current));
}

#[test]
fn ra005_ra007_incomplete_successor_refuses_until_verified_catchup() {
    let mut cluster = cluster();
    accepted(committed(&mut cluster, 1, create(1)));
    cluster.isolate(3);
    accepted(committed(&mut cluster, 1, mutate(1, 23)));
    assert!(cluster.applied(3) < cluster.applied(1));
    let incomplete = cluster.move_command(request(1, 2), 1, 1, 3);
    let refused = committed(&mut cluster, 1, incomplete);
    assert_eq!(
        refused.outcome,
        Outcome::Rejected(Rejection::IncompleteSuccessor)
    );
    assert_eq!(cluster.resource(1, 100).unwrap().home, 1);
    cluster.heal();
    cluster.drain();
    assert_eq!(cluster.applied(3), cluster.applied(1));
    let ready = cluster.move_command(request(1, 3), 1, 1, 3);
    accepted(committed(&mut cluster, 1, ready));
    assert_eq!(cluster.resource(1, 100).unwrap().home, 3);
    assert_eq!(cluster.resource(3, 100).unwrap().payload, 23);
}

#[test]
fn ra006_ordered_revocation_denies_delayed_mutation_but_permits_disclosed_retry() {
    let mut cluster = cluster();
    accepted(committed(&mut cluster, 1, create(1)));
    let original_command = mutate(1, 23);
    let original = committed(&mut cluster, 1, original_command);
    accepted(original);
    let policy = Command {
        request: request(1, 2),
        generation: 1,
        home: 1,
        policy_frontier: 0,
        action: Action::SetPermission {
            principal: 10,
            write: false,
            disclose: true,
        },
    };
    let frontier = committed(&mut cluster, 1, policy).index;
    let mut delayed = mutate(2, 99);
    delayed.policy_frontier = frontier;
    assert_eq!(
        committed(&mut cluster, 1, delayed).outcome,
        Outcome::Rejected(Rejection::Unauthorized)
    );
    assert_eq!(committed(&mut cluster, 1, original_command), original);
    let denied_disclosure = Command {
        request: request(1, 3),
        policy_frontier: frontier,
        action: Action::SetPermission {
            principal: 10,
            write: false,
            disclose: false,
        },
        ..policy
    };
    accepted(committed(&mut cluster, 1, denied_disclosure));
    cluster.propose(1, original_command);
    cluster.drain();
    assert!(
        cluster.reply(1, original_command).is_none(),
        "RA-006: serving hop denies current disclosure"
    );
    assert!(cluster.outcome(1, original_command.request).is_none());
}

#[test]
fn ra009_tombstone_fences_new_commands_but_retains_exact_retry() {
    let mut cluster = cluster();
    let original_create = create(1);
    let original = committed(&mut cluster, 1, original_create);
    accepted(original);
    let retire = Command {
        request: request(1, 2),
        generation: 1,
        home: 1,
        policy_frontier: 0,
        action: Action::Retire,
    };
    accepted(committed(&mut cluster, 1, retire));
    assert!(cluster.resource(1, 100).unwrap().retired);
    assert_eq!(
        committed(&mut cluster, 1, mutate(1, 99)).outcome,
        Outcome::Rejected(Rejection::Retired)
    );
    assert_eq!(
        committed(&mut cluster, 1, create(3)).outcome,
        Outcome::Rejected(Rejection::Retired)
    );
    let moved = cluster.move_command(request(1, 4), 1, 1, 2);
    assert_eq!(
        committed(&mut cluster, 1, moved).outcome,
        Outcome::Rejected(Rejection::Retired)
    );
    assert_eq!(committed(&mut cluster, 1, original_create), original);
}

#[test]
fn ra006_ra008_wrong_scope_and_opaque_effect_are_terminal_refusals() {
    let mut cluster = cluster();
    accepted(committed(&mut cluster, 1, create(1)));
    let mut wrong_scope = mutate(1, 99);
    wrong_scope.request.scope = 8;
    assert_eq!(
        committed(&mut cluster, 1, wrong_scope).outcome,
        Outcome::Rejected(Rejection::WrongScope)
    );
    let effect = Command {
        action: Action::ExternalEffect { code: 1 },
        ..mutate(2, 99)
    };
    assert_eq!(
        committed(&mut cluster, 1, effect).outcome,
        Outcome::Rejected(Rejection::UnsupportedEffect)
    );
    assert_eq!(cluster.resource(1, 100).unwrap().payload, 11);
}

#[test]
fn ra010_two_voters_cannot_progress_after_either_loss() {
    for lost in [1, 2] {
        let mut cluster = Cluster::new(&[1, 2]);
        cluster.campaign(1);
        cluster.drain();
        accepted(committed(&mut cluster, 1, create(1)));
        cluster.isolate(lost);
        let survivor = 3 - lost;
        cluster.campaign(survivor);
        cluster.drain();
        let command = mutate(1, 99);
        assert_eq!(cluster.propose(survivor, command), Proposal::Unknown);
        cluster.drain();
        assert!(cluster.outcome(survivor, command.request).is_none());
        assert_eq!(cluster.resource(survivor, 100).unwrap().payload, 11);
    }
}

#[test]
fn ra003_ra011_application_orders_noops_replay_and_rejects_gaps() {
    let mut application = Application::new();
    assert_eq!(application.apply(1, None), Ok(None));
    let command = create(1);
    let original = application.apply(2, Some(command));
    assert!(matches!(
        original,
        Ok(Some(Receipt {
            outcome: Outcome::Accepted(_),
            ..
        }))
    ));
    assert_eq!(application.apply(2, Some(command)), original);
    assert_eq!(application.lookup(command.request), original.unwrap());
    assert_eq!(application.apply(3, None), Ok(None));
    assert_eq!(
        application.apply(5, None),
        Err(ApplyError::IndexGap {
            expected: 4,
            received: 5
        })
    );
    let mut different = command;
    different.action = Action::Create {
        name: 40,
        home: 1,
        payload: 12,
    };
    assert_eq!(
        application.apply(2, Some(different)),
        Err(ApplyError::ConflictingReplay { index: 2 })
    );
}
