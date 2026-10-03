//! Same ordinary executable consumers for both provider selections.
//! No case may treat NotQualified as success. Engine-specific source/cadence/
//! lease/greater-log assertions remain additional pre-acceptance obligations.
use crate::{
    Provider,
    fixture::{Fault, Fixture, QUANTUM_NS},
    oracle,
};
use glade_carrier_api::*;

const FAIR_ROUNDS: usize = 10_000;
struct Cluster {
    fixture: Fixture,
    nodes: Vec<Box<dyn ElectionNode>>,
    paused: Vec<bool>,
    isolated: Vec<bool>,
    history: oracle::History,
}
impl Cluster {
    fn new(provider: &impl Provider, equal: bool) -> Self {
        let mut fixture = Fixture::new(provider.carrier(), 1);
        fixture.equal_scripts = equal;
        let nodes = (1..=3)
            .map(|node| {
                provider
                    .construct(fixture.inputs(node, 1))
                    .expect("explicit supplied dependency constructor")
            })
            .collect();
        Self {
            fixture,
            nodes,
            paused: vec![false; 3],
            isolated: vec![false; 3],
            history: oracle::History::default(),
        }
    }
    fn views(&self, case: &str) -> Vec<ElectionView> {
        self.nodes
            .iter()
            .map(|node| {
                node.observe().unwrap_or_else(|error| {
                    panic!("{case}: required actual engine observation refused: {error:?}")
                })
            })
            .collect()
    }
    fn round(&mut self, case: &str) {
        for index in 0..3 {
            let current = self.fixture.source(index as u64 + 1, 1).clock();
            let to = LogicalInstant {
                nanos: current
                    .nanos
                    .checked_add(QUANTUM_NS)
                    .expect("bounded fixture time"),
                ..current
            };
            self.nodes[index].advance(to).unwrap_or_else(|error| {
                panic!("{case}: controlled clock action refused: {error:?}")
            });
            if self.paused[index] {
                continue;
            }
            let inventory = self.nodes[index]
                .inventory()
                .expect("visible complete engine task inventory");
            let mut work = inventory.work;
            work.sort_by_key(|work| work.id);
            for work in work {
                if work.state == WorkState::Runnable {
                    self.nodes[index]
                        .drive(work.id)
                        .expect("one scheduled action");
                }
            }
        }
        let mut messages = self.fixture.messages();
        messages.sort_by_key(|message| message.token.sequence);
        for message in messages {
            if !matches!(
                message.state,
                MessageState::Held | MessageState::Deliverable
            ) {
                continue;
            }
            let from = message.token.issuer.node.node as usize - 1;
            let to = message.token.destination.node.node as usize - 1;
            if self.isolated[from] != self.isolated[to] {
                self.nodes[from]
                    .transport(TransportAction::Drop(message.token))
                    .expect("explicit partition drop");
            } else if !self.paused[from] && !self.paused[to] {
                self.deliver(message);
            }
        }
        let views = self.views(case);
        self.history
            .check(&views, &self.fixture.messages())
            .expect("external prefix/vote-epoch invariant");
    }
    fn deliver(&mut self, message: MessageView) {
        let from = message.token.issuer.node.node as usize - 1;
        let to = message.token.destination.node.node as usize - 1;
        if message.state == MessageState::Held {
            self.nodes[from]
                .transport(TransportAction::Release(message.token))
                .expect("explicit release");
        }
        if let Some(rpc) = message.reply_to {
            assert_eq!(
                message.token.destination, rpc.scope,
                "explicit reply caller scope"
            );
            let before = self.nodes[to].inventory().unwrap();
            let result = self.nodes[to].transport(TransportAction::Resolve {
                rpc,
                reply: message.token,
            });
            if before.pending_rpcs.contains(&rpc) {
                result.expect("explicit correlated pending RPC resolution");
            } else {
                assert_eq!(
                    result,
                    Err(DriverError::InvalidToken),
                    "terminal RPC reply cannot progress"
                );
                assert_eq!(self.nodes[to].inventory().unwrap(), before);
                self.nodes[from]
                    .transport(TransportAction::Drop(message.token))
                    .expect("explicit late RPC reply drop");
            }
        } else {
            self.nodes[to]
                .receive(message.token)
                .expect("selected request/one-way engine message");
        }
    }
    fn fair_leader(&mut self, case: &str) -> NodeKey {
        // A provider refusal must fail the positive invariant, never skip scheduling.
        let views: Vec<_> = self
            .views(case)
            .into_iter()
            .enumerate()
            .filter(|(index, _)| !self.paused[*index])
            .map(|(_, view)| view)
            .collect();
        if let Ok(leader) = oracle::leader(&views, &self.fixture.messages()) {
            return leader;
        }
        for _ in 0..FAIR_ROUNDS {
            self.round(case);
            if let Ok(leader) = oracle::leader(
                &self
                    .views(case)
                    .into_iter()
                    .enumerate()
                    .filter(|(index, _)| !self.paused[*index])
                    .map(|(_, view)| view)
                    .collect::<Vec<_>>(),
                &self.fixture.messages(),
            ) {
                return leader;
            }
        }
        panic!(
            "{case}: no quorum election within {FAIR_ROUNDS} rounds; views={:?}, messages={:?}, trace={:?}",
            self.views(case),
            self.fixture.messages(),
            self.fixture.trace()
        );
    }
}

mod sources;
pub use sources::{
    constructor_and_reset_use_supplied_streams, source_failure_has_no_ambient_fallback,
};
mod elections;
pub use elections::{
    automatic_election_without_campaign, deadline_and_vote_rules_preserved,
    delayed_duplicate_reordered_vote_append, minority_cannot_elect,
    split_vote_eventual_useful_schedule,
};
mod lifecycle;
pub use lifecycle::{
    independent_clock_schedules, invalid_controls_and_stop_lifecycle,
    paused_leader_automatic_failover_and_resume,
};
