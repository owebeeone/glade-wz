//! Detector self-tests only: synthetic observations never qualify a carrier.
use glade_carrier_api::*;
use glade_carrier_spec::{fixture::Fixture, oracle};
fn view(scope: NodeScope, leader: NodeKey) -> ElectionView {
    ElectionView {
        scope,
        state: NodeState::Running,
        role: if scope.node == leader {
            Role::Leader
        } else {
            Role::Follower
        },
        leader: Some(leader),
        vote: Some(VoteIdentity {
            carrier: scope.carrier,
            opaque: vec![1, 1],
        }),
        epoch: Some(ElectionEpoch {
            carrier: scope.carrier,
            group: scope.node.group,
            term: 1,
        }),
        committed: Vec::new(),
    }
}
#[test]
fn synthetic_leader_without_engine_vote_traffic_is_rejected() {
    let fixture = Fixture::new(Carrier::RaftRs, 3);
    let leader = fixture.scope(1, 1).node;
    let views = vec![
        view(fixture.scope(1, 1), leader),
        view(fixture.scope(2, 1), leader),
        view(fixture.scope(3, 1), leader),
    ];
    assert!(
        oracle::leader(&views, &[])
            .unwrap_err()
            .contains("vote traffic")
    );
}
#[test]
fn static_fixed_stream_ignoring_supplied_script_is_rejected() {
    let fixture = Fixture::new(Carrier::RaftRs, 3);
    let scope = fixture.scope(2, 1);
    let range = fixture.range();
    let trace = [Event::SourceDraw {
        scope,
        purpose: DrawPurpose::Constructor,
        range,
        value: range.lower,
    }];
    assert!(
        oracle::required_draws(&trace, scope, range, &[range.lower + 3])
            .unwrap_err()
            .contains("supplied stream")
    );
}
#[test]
fn bypassed_required_reset_draw_is_rejected() {
    let fixture = Fixture::new(Carrier::RaftRs, 3);
    let scope = fixture.scope(1, 1);
    let range = fixture.range();
    let trace = [Event::SourceDraw {
        scope,
        purpose: DrawPurpose::Constructor,
        range,
        value: 10,
    }];
    assert!(
        oracle::required_draws(&trace, scope, range, &[10, 19])
            .unwrap_err()
            .contains("access footprint")
    );
}
#[test]
fn early_vote_removing_reviewed_lease_or_greater_log_delay_is_rejected() {
    let fixture = Fixture::new(Carrier::OpenRaft, 3);
    let scope = fixture.scope(1, 1);
    let message = MessageView {
        token: MessageToken {
            issuer: scope,
            destination: fixture.scope(2, 1),
            sequence: 1,
        },
        kind: ProtocolKind::VoteRequest,
        state: MessageState::Held,
        rpc: None,
        reply_to: None,
    };
    for required_deadline in [100, 200] {
        assert!(
            oracle::eligibility(
                LogicalInstant {
                    domain: scope.domain,
                    nanos: 99
                },
                LogicalInstant {
                    domain: scope.domain,
                    nanos: required_deadline
                },
                &[message]
            )
            .unwrap_err()
            .contains("before reviewed")
        );
    }
}
#[test]
fn implicit_advancement_of_another_nodes_clock_is_rejected() {
    let fixture = Fixture::new(Carrier::RaftRs, 3);
    let before: Vec<_> = (1..=3)
        .map(|node| {
            let clock = LogicalInstant {
                domain: fixture.scope(node, 1).domain,
                nanos: 0,
            };
            NodeInventory {
                clock,
                work: Vec::new(),
                messages: Vec::new(),
                pending_rpcs: Vec::new(),
                timing: TimingBoundary {
                    before: clock,
                    first_eligible: LogicalInstant { nanos: 10, ..clock },
                    sampled: 10,
                    resume: ResumePolicy::HostSingleTick,
                },
            }
        })
        .collect();
    let mut after = before.clone();
    for node in &mut after {
        node.clock.nanos = 1;
    }
    assert!(
        oracle::independent_clocks(&before, &after, 0)
            .unwrap_err()
            .contains("unselected")
    );
}
#[test]
fn paused_background_work_is_rejected() {
    let fixture = Fixture::new(Carrier::OpenRaft, 3);
    let id = WorkId {
        scope: fixture.scope(1, 1),
        sequence: 1,
    };
    let before = [WorkView {
        id,
        deadline: None,
        state: WorkState::Pending,
    }];
    let after = [WorkView {
        state: WorkState::Complete,
        ..before[0]
    }];
    assert!(
        oracle::paused_work(&before, &after)
            .unwrap_err()
            .contains("background")
    );
}

#[test]
fn incompatible_standard_epoch_leaders_are_rejected_even_when_full_votes_differ() {
    let fixture = Fixture::new(Carrier::OpenRaft, 4);
    let first = fixture.scope(1, 1).node;
    let second = fixture.scope(2, 1).node;
    let traffic = [MessageView {
        token: MessageToken {
            issuer: fixture.scope(1, 1),
            destination: fixture.scope(2, 1),
            sequence: 1,
        },
        kind: ProtocolKind::VoteRequest,
        state: MessageState::Consumed,
        rpc: None,
        reply_to: None,
    }];
    let mut history = oracle::History::default();
    let before: Vec<_> = (1..=3)
        .map(|node| view(fixture.scope(node, 1), first))
        .collect();
    history.check(&before, &traffic).unwrap();
    let after: Vec<_> = (1..=3)
        .map(|node| {
            let mut view = view(fixture.scope(node, 1), second);
            view.vote.as_mut().unwrap().opaque = vec![1, 2];
            view
        })
        .collect();
    assert!(
        history
            .check(&after, &traffic)
            .unwrap_err()
            .contains("election epoch")
    );
}
