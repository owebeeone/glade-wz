use glade_raft_q3_api::conformance;
mod common;
use common::{session, store};

#[test]
fn v2_checkpoint_exact_roundtrip_and_immutable_same_cut() {
    conformance::store_snapshot_roundtrip(&mut store());
}

#[test]
fn v2_suffix_replacement_bounds_and_commit_cut() {
    conformance::store_suffix_and_bounds(&mut store());
}

#[test]
fn v2_historical_same_term_vote_survives_removed_voter() {
    conformance::store_removed_vote_is_not_rewritten(&mut store());
}

#[test]
fn membership_authorized_namespace_and_exact_intent_retry() {
    conformance::membership_authority_and_retry(&mut session());
}

#[test]
fn learner_unavailable_cannot_promote_or_supply_quorum() {
    conformance::learner_unavailable_and_nonvoting(&mut session());
}

#[test]
fn joint_outgoing_majority_alone_cannot_accept() {
    conformance::joint_authority(&mut session(), vec![2, 4]);
}

#[test]
fn joint_incoming_majority_alone_cannot_accept() {
    conformance::joint_authority(&mut session(), vec![2, 3]);
}

#[test]
fn joint_restart_and_lost_configuration_reply_are_recoverable() {
    conformance::joint_lost_reply_restart(&mut session());
}

#[test]
fn removing_current_resource_home_refuses_without_rehoming() {
    conformance::removal_preserves_home(&mut session());
}

#[test]
fn snapshot_original_receipts_commands_and_exact_index_replay_survive() {
    conformance::snapshot_original_receipt(&mut session());
}

#[test]
fn snapshot_policy_retirement_and_name_reservation_survive() {
    conformance::snapshot_policy_retirement(&mut session());
}

#[test]
fn snapshot_valid_foreign_binding_corrupt_state_and_capacity_refuse() {
    conformance::snapshot_mismatch(&mut session());
}

#[test]
fn snapshot_movement_preserves_complete_readiness_and_generation_fence() {
    conformance::snapshot_movement_fence(&mut session());
}

#[test]
fn configuration_eligibility_lost_after_admission_retains_exact_refusal() {
    conformance::admitted_configuration_refusal_is_retained(&mut session());
}

#[test]
fn compacted_snapshot_restores_learner_before_joint_promotion() {
    conformance::snapshot_learner_catchup_and_membership(&mut session());
}

#[test]
fn joint_exit_refuses_current_outgoing_home_until_retirement() {
    conformance::joint_exit_rechecks_home(&mut session(), false, false);
}

#[test]
fn joint_exit_refuses_current_outgoing_home_until_qualified_movement() {
    conformance::joint_exit_rechecks_home(&mut session(), false, true);
}

#[test]
fn joint_exit_rechecks_placement_queued_after_admission() {
    conformance::joint_exit_rechecks_home(&mut session(), true, false);
}

#[test]
fn snapshot_original_accepted_refused_configuration_and_noop_replay_is_typed() {
    conformance::snapshot_typed_configuration_and_noop_replay(&mut session());
}
