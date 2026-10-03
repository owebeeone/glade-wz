//! Intentionally RED. These exact shared consumers later select real adapters.
use glade_carrier_api::{Carrier, DriverError, ElectionNode, NodeInputs};
use glade_carrier_spec::{Provider, b0};

struct RaftRs;
impl Provider for RaftRs {
    fn carrier(&self) -> Carrier {
        Carrier::RaftRs
    }
    fn construct(&self, inputs: NodeInputs) -> Result<Box<dyn ElectionNode>, DriverError> {
        glade_carrier_raft_rs::RefusingRaftRs::new(inputs)
            .map(|node| Box::new(node) as Box<dyn ElectionNode>)
    }
}
struct OpenRaft;
impl Provider for OpenRaft {
    fn carrier(&self) -> Carrier {
        Carrier::OpenRaft
    }
    fn construct(&self, inputs: NodeInputs) -> Result<Box<dyn ElectionNode>, DriverError> {
        glade_carrier_openraft::RefusingOpenRaft::new(inputs)
            .map(|node| Box::new(node) as Box<dyn ElectionNode>)
    }
}

mod raft_rs {
    use super::*;
    #[test]
    fn b0_01() {
        b0::automatic_election_without_campaign(&RaftRs);
    }
    #[test]
    fn b0_02() {
        b0::constructor_and_reset_use_supplied_streams(&RaftRs);
    }
    #[test]
    fn b0_03() {
        b0::source_failure_has_no_ambient_fallback(&RaftRs);
    }
    #[test]
    fn b0_04() {
        b0::deadline_and_vote_rules_preserved(&RaftRs);
    }
    #[test]
    fn b0_05() {
        b0::split_vote_eventual_useful_schedule(&RaftRs);
    }
    #[test]
    fn b0_06() {
        b0::minority_cannot_elect(&RaftRs);
    }
    #[test]
    fn b0_07() {
        b0::delayed_duplicate_reordered_vote_append(&RaftRs);
    }
    #[test]
    fn b0_08() {
        b0::paused_leader_automatic_failover_and_resume(&RaftRs);
    }
    #[test]
    fn b0_09() {
        b0::independent_clock_schedules(&RaftRs);
    }
    #[test]
    fn b0_10() {
        b0::invalid_controls_and_stop_lifecycle(&RaftRs);
    }
}
mod openraft {
    use super::*;
    #[test]
    fn b0_01() {
        b0::automatic_election_without_campaign(&OpenRaft);
    }
    #[test]
    fn b0_02() {
        b0::constructor_and_reset_use_supplied_streams(&OpenRaft);
    }
    #[test]
    fn b0_03() {
        b0::source_failure_has_no_ambient_fallback(&OpenRaft);
    }
    #[test]
    fn b0_04() {
        b0::deadline_and_vote_rules_preserved(&OpenRaft);
    }
    #[test]
    fn b0_05() {
        b0::split_vote_eventual_useful_schedule(&OpenRaft);
    }
    #[test]
    fn b0_06() {
        b0::minority_cannot_elect(&OpenRaft);
    }
    #[test]
    fn b0_07() {
        b0::delayed_duplicate_reordered_vote_append(&OpenRaft);
    }
    #[test]
    fn b0_08() {
        b0::paused_leader_automatic_failover_and_resume(&OpenRaft);
    }
    #[test]
    fn b0_09() {
        b0::independent_clock_schedules(&OpenRaft);
    }
    #[test]
    fn b0_10() {
        b0::invalid_controls_and_stop_lifecycle(&OpenRaft);
    }
}
