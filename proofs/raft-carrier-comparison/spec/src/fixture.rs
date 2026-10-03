//! Explicit instance-owned test fixture assembly; no election algorithm.
mod sources;
mod transport;
mod work;
use glade_carrier_api::*;
use sources::SourceState;
pub use sources::{Fault, SourceProbe};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
pub use transport::Endpoint;
use transport::Network;
pub use work::WorkProbe;
pub const QUANTUM_NS: u64 = 1_000_000;
struct Memory(ProtocolImage);
impl MemoryProtocolStore for Memory {
    fn load(&self) -> Result<ProtocolImage, DriverError> {
        Ok(self.0.clone())
    }
    fn publish(&mut self, image: ProtocolImage) -> Result<(), DriverError> {
        if image.carrier != self.0.carrier {
            return Err(DriverError::Protocol);
        }
        self.0 = image;
        Ok(())
    }
}
struct Logger(Rc<RefCell<Vec<Event>>>);
impl InstanceLogger for Logger {
    fn record(&mut self, event: Event) {
        self.0.borrow_mut().push(event);
    }
}

pub struct Fixture {
    pub carrier: Carrier,
    pub session: u64,
    pub equal_scripts: bool,
    probes: RefCell<BTreeMap<(u64, u64), (SourceProbe, WorkProbe)>>,
    trace: Rc<RefCell<Vec<Event>>>,
    network: Rc<RefCell<Network>>,
}
impl Fixture {
    pub fn new(carrier: Carrier, session: u64) -> Self {
        let network = Rc::new(RefCell::new(Network {
            scopes: BTreeMap::new(),
            sequence: 0,
            messages: BTreeMap::new(),
            rpcs: BTreeMap::new(),
        }));
        let fixture = Self {
            carrier,
            session,
            equal_scripts: false,
            probes: RefCell::new(BTreeMap::new()),
            trace: Rc::new(RefCell::new(Vec::new())),
            network,
        };
        for node in 1..=3 {
            fixture
                .network
                .borrow_mut()
                .scopes
                .insert(node, fixture.scope(node, 1));
        }
        fixture
    }
    pub fn scope(&self, node: u64, incarnation: u64) -> NodeScope {
        NodeScope {
            carrier: self.carrier,
            session: self.session,
            node: NodeKey { group: 7, node },
            incarnation,
            domain: ClockDomain(
                self.session
                    .checked_mul(100)
                    .unwrap()
                    .checked_add(node.checked_mul(10).unwrap())
                    .unwrap()
                    .checked_add(incarnation)
                    .unwrap(),
            ),
        }
    }
    pub fn range(&self) -> SampleRange {
        match self.carrier {
            Carrier::RaftRs => SampleRange {
                lower: 10,
                upper_exclusive: 20,
            },
            Carrier::OpenRaft => SampleRange {
                lower: 10,
                upper_exclusive: 20,
            },
        }
    }
    pub fn inputs(&self, node: u64, incarnation: u64) -> NodeInputs {
        let scope = self.scope(node, incarnation);
        let range = self.range();
        let first = if self.equal_scripts {
            range.lower
        } else {
            range.lower + (node - 1) * (range.upper_exclusive - range.lower) / 3
        };
        let samples = [
            first,
            range.lower,
            range.upper_exclusive - 1,
            range.lower + 1,
        ]
        .into_iter()
        .cycle()
        .take(30_000)
        .map(Ok)
        .collect();
        let sources = SourceProbe(Rc::new(RefCell::new(SourceState {
            scope,
            clock: LogicalInstant {
                domain: scope.domain,
                nanos: 0,
            },
            samples,
            fault: None,
            trace: self.trace.clone(),
        })));
        let work = WorkProbe::new(scope);
        assert!(
            self.probes
                .borrow_mut()
                .insert((node, incarnation), (sources.clone(), work.clone()))
                .is_none(),
            "context constructed once per node incarnation"
        );
        self.network.borrow_mut().scopes.insert(node, scope);
        let timing = match self.carrier {
            Carrier::RaftRs => TimingProfile::RaftRs {
                quantum_ns: QUANTUM_NS,
                election_ticks: 10,
                max_election_ticks_exclusive: 20,
                heartbeat_ticks: 1,
                pre_vote: true,
                check_quorum: true,
            },
            Carrier::OpenRaft => TimingProfile::OpenRaft {
                ticker_ns: 3 * QUANTUM_NS,
                min_election_ms: 10,
                max_election_ms_exclusive: 20,
                heartbeat_ms: 2,
                election_enabled: true,
                heartbeat_enabled: true,
            },
        };
        NodeInputs {
            sources: SourceContext::new(scope, Box::new(sources), Box::new(work)),
            store: Box::new(Memory(ProtocolImage {
                carrier: self.carrier,
                opaque: Vec::new(),
            })),
            endpoint: Box::new(self.endpoint(scope)),
            configuration: FixedConfiguration {
                group: 7,
                voters: [1, 2, 3],
                authorized_genesis: true,
                timing,
            },
            logger: Box::new(Logger(self.trace.clone())),
        }
    }
    pub fn source(&self, node: u64, incarnation: u64) -> SourceProbe {
        self.probes
            .borrow()
            .get(&(node, incarnation))
            .unwrap()
            .0
            .clone()
    }
    pub fn work(&self, node: u64, incarnation: u64) -> WorkProbe {
        self.probes
            .borrow()
            .get(&(node, incarnation))
            .unwrap()
            .1
            .clone()
    }
    pub fn endpoint(&self, scope: NodeScope) -> Endpoint {
        Endpoint {
            scope,
            network: self.network.clone(),
            stopped: Rc::new(RefCell::new(false)),
        }
    }
    pub fn trace(&self) -> Vec<Event> {
        self.trace.borrow().clone()
    }
    pub fn messages(&self) -> Vec<MessageView> {
        self.network
            .borrow()
            .messages
            .values()
            .map(|queued| queued.view)
            .collect()
    }
}
