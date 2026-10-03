//! Shared executable B0 consumer scenarios, still deliberately RED.
use super::*;
pub fn automatic_election_without_campaign(provider: &impl Provider) {
    let mut cluster = Cluster::new(provider, false);
    let leader = cluster.fair_leader("B0-01 elected-leader invariant");
    assert!((1..=3).contains(&leader.node));
    assert!(
        cluster
            .fixture
            .messages()
            .iter()
            .any(|message| message.kind == ProtocolKind::VoteRequest)
    );
    // No campaign/trigger operation exists in the consumer boundary.
}

fn boundary_point(
    node: &mut dyn ElectionNode,
    at: LogicalInstant,
    first_eligible: Option<LogicalInstant>,
    eligibility: &mut Option<(LogicalInstant, WorkId)>,
) -> Vec<WorkId> {
    point::drain_point(|selected| {
        if let Some(work) = selected {
            node.drive(work)?; // Exactly one selected poll/tick; no delivery/time here.
            if let Some(first_eligible) = first_eligible {
                let view = node.observe()?;
                if at.nanos < first_eligible.nanos {
                    assert_eq!(
                        view.role,
                        Role::Follower,
                        "B0-04 no early election after any selected action"
                    );
                } else if matches!(view.role, Role::PreCandidate | Role::Candidate)
                    && eligibility.is_none()
                {
                    *eligibility = Some((at, work));
                }
            }
        }
        let inventory = node.inventory()?;
        assert_eq!(
            inventory.clock, at,
            "B0-04 fixed-instant drain cannot change time"
        );
        if let Some(first_eligible) = first_eligible {
            let votes = inventory
                .messages
                .iter()
                .any(|message| message.kind == ProtocolKind::VoteRequest);
            if at.nanos < first_eligible.nanos {
                assert!(
                    !votes,
                    "B0-04 no early vote emission after any selected action"
                );
            } else if votes {
                assert!(
                    eligibility.is_some(),
                    "B0-04 eligibility precedes scheduled send completion"
                );
            }
        }
        Ok(inventory.work)
    })
    .expect("B0-04 current-inventory bounded one-action drain")
}

pub fn deadline_and_vote_rules_preserved(provider: &impl Provider) {
    let mut cluster = Cluster::new(provider, false);
    let initial = cluster.nodes[0].inventory().unwrap_or_else(|error| {
        panic!("B0-04 recorded engine eligibility boundary required: {error:?}")
    });
    // Initialize pending core/ticker work at the constructor's unchanged time.
    let mut eligibility = None;
    let mut selected = vec![(
        initial.clock,
        boundary_point(
            cluster.nodes[0].as_mut(),
            initial.clock,
            None,
            &mut eligibility,
        ),
    )];
    let inventory = cluster.nodes[0].inventory().unwrap();
    let timing = inventory.timing;
    assert!(inventory.clock.nanos <= timing.before.nanos);
    assert!(timing.before.nanos < timing.first_eligible.nanos);
    assert_eq!(timing.before.domain, cluster.fixture.scope(1, 1).domain);
    assert_eq!(timing.first_eligible.domain, timing.before.domain);
    assert_eq!(
        cluster.nodes[0].observe().unwrap().role,
        Role::Follower,
        "B0-04 initialized follower prerequisite"
    );
    assert!(
        !cluster
            .fixture
            .messages()
            .iter()
            .any(|message| message.kind == ProtocolKind::VoteRequest)
    );
    // Progress actual controlled host/ticker opportunities. A jump is not a
    // substitute for elapsed raft-rs ticks; partial boundary points stay exact.
    for target in [timing.before, timing.first_eligible] {
        let now = cluster.nodes[0].inventory().unwrap().clock;
        for to in point::time_points(now, target) {
            cluster.nodes[0].advance(to).unwrap();
            selected.push((
                to,
                boundary_point(
                    cluster.nodes[0].as_mut(),
                    to,
                    Some(timing.first_eligible),
                    &mut eligibility,
                ),
            ));
        }
        if target == timing.before {
            assert_eq!(
                cluster.nodes[0].observe().unwrap().role,
                Role::Follower,
                "B0-04 no early election at boundary"
            );
            assert!(
                !cluster
                    .fixture
                    .messages()
                    .iter()
                    .any(|message| message.kind == ProtocolKind::VoteRequest)
            );
        }
    }
    assert!(
        eligibility.is_some(),
        "B0-04 actual eligible election action; selected={selected:?}"
    );
    assert!(
        cluster
            .fixture
            .messages()
            .iter()
            .any(|message| message.kind == ProtocolKind::VoteRequest),
        "B0-04 subsequent scheduled vote emission; eligibility={eligibility:?}; selected={selected:?}"
    );
    // Exact per-pin eligibility mapping still requires real adapted-source tests.
}

pub fn split_vote_eventual_useful_schedule(provider: &impl Provider) {
    let mut cluster = Cluster::new(provider, true);
    let initial = cluster.views("B0-05 no-quorum prefix observation");
    assert!(oracle::leader(&initial, &cluster.fixture.messages()).is_err());
    for _ in 0..25 {
        for index in 0..3 {
            let now = cluster.fixture.source(index as u64 + 1, 1).clock();
            cluster.nodes[index]
                .advance(LogicalInstant {
                    nanos: now.nanos + QUANTUM_NS,
                    ..now
                })
                .unwrap();
            for work in cluster.nodes[index].inventory().unwrap().work {
                if work.state == WorkState::Runnable {
                    cluster.nodes[index].drive(work.id).unwrap();
                }
            }
        }
        assert!(
            oracle::leader(
                &cluster.views("B0-05 held split traffic"),
                &cluster.fixture.messages()
            )
            .is_err()
        );
    }
    let competing: std::collections::BTreeSet<_> = cluster
        .fixture
        .messages()
        .iter()
        .filter(|message| message.kind == ProtocolKind::VoteRequest)
        .map(|message| message.token.issuer.node)
        .collect();
    assert!(
        competing.len() >= 2,
        "B0-05 must retain engine traffic from competing candidates"
    );
    assert!(
        cluster
            .views("B0-05 actual competing roles")
            .iter()
            .filter(|view| matches!(view.role, Role::Candidate | Role::PreCandidate))
            .count()
            >= 2,
        "B0-05 actual engine candidate/pre-candidate prefix"
    );
    cluster.paused[1] = true;
    cluster.round("B0-05 independent node-2 withheld work");
    cluster.paused[1] = false;
    cluster.fair_leader("B0-05 eventual useful scheduling");
}

pub fn minority_cannot_elect(provider: &impl Provider) {
    let mut cluster = Cluster::new(provider, false);
    cluster.isolated[0] = true;
    let initial = cluster.views("B0-06 minority cannot establish quorum");
    assert!(oracle::leader(&initial[..1], &cluster.fixture.messages()).is_err());
    for _ in 0..100 {
        cluster.round("B0-06 bidirectional isolation");
    }
    let leader = cluster.fair_leader("B0-06 majority liveness");
    assert_ne!(leader.node, 1);
    assert!(
        oracle::leader(
            &cluster.views("B0-06 isolated local role is insufficient")[..1],
            &cluster.fixture.messages()
        )
        .is_err()
    );
}

pub fn delayed_duplicate_reordered_vote_append(provider: &impl Provider) {
    let mut cluster = Cluster::new(provider, false);
    let old = cluster.fair_leader("B0-07 initial actual election");
    cluster.paused[old.node as usize - 1] = true;
    for _ in 0..30 {
        cluster.round("B0-07 held old reply prefix");
    }
    let retained: Vec<_> = cluster
        .fixture
        .messages()
        .into_iter()
        .filter(|message| {
            message.state == MessageState::Held
                && matches!(
                    message.kind,
                    ProtocolKind::VoteResponse | ProtocolKind::AppendResponse
                )
        })
        .collect();
    assert!(
        !retained.is_empty(),
        "B0-07 schedule must retain real old replies"
    );
    let new = cluster.fair_leader("B0-07 automatic failover");
    assert_ne!(old, new);
    cluster.paused[old.node as usize - 1] = false;
    for message in retained.into_iter().rev() {
        let from = message.token.issuer.node.node as usize - 1;
        let duplicate = cluster.nodes[from]
            .transport(TransportAction::Duplicate(message.token))
            .unwrap();
        let copied = duplicate
            .iter()
            .find_map(|event| {
                if let Event::Outbound { message } = event {
                    Some(message.token)
                } else {
                    None
                }
            })
            .expect("fresh duplicate token");
        assert_ne!(copied, message.token);
        for token in [copied, message.token] {
            let queued = cluster
                .fixture
                .messages()
                .into_iter()
                .find(|message| message.token == token)
                .unwrap();
            cluster.deliver(queued);
        }
        cluster
            .history
            .check(
                &cluster.views("B0-07 old message validation"),
                &cluster.fixture.messages(),
            )
            .unwrap();
    }
    cluster.fair_leader("B0-07 final useful schedule");
}
