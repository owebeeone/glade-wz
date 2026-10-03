use glade_carrier_api::*;

#[test]
fn complete_time_range_and_invalid_token_inputs_compile() {
    let scope = NodeScope {
        carrier: Carrier::RaftRs,
        session: 1,
        node: NodeKey { group: 9, node: 1 },
        incarnation: 1,
        domain: ClockDomain(101),
    };
    let instant = LogicalInstant {
        domain: scope.domain,
        nanos: 4,
    };
    let backwards = LogicalInstant {
        nanos: 3,
        ..instant
    };
    let foreign_time = LogicalInstant {
        domain: ClockDomain(999),
        nanos: 5,
    };
    let overflow_input = u64::MAX;
    let invalid_range = SampleRange {
        lower: 3,
        upper_exclusive: 3,
    };
    assert_ne!(instant.domain, foreign_time.domain);
    assert!(backwards.nanos < instant.nanos);
    assert_eq!(overflow_input, u64::MAX);
    assert_eq!(invalid_range.lower, invalid_range.upper_exclusive);
    let work = WorkId { scope, sequence: 7 };
    let foreign = WorkId {
        scope: NodeScope {
            incarnation: 2,
            ..scope
        },
        ..work
    };
    assert_ne!(work, foreign);
    let message = MessageToken {
        destination: scope,
        issuer: scope,
        sequence: 7,
    };
    assert_ne!(message.destination, foreign.scope);
}

#[test]
fn owned_future_and_complete_error_event_matches_compile() {
    let future: OwnedTask = Box::pin(async {});
    let scope = NodeScope {
        carrier: Carrier::OpenRaft,
        session: 1,
        node: NodeKey { group: 9, node: 1 },
        incarnation: 1,
        domain: ClockDomain(101),
    };
    let work = WorkSpec::At {
        deadline: LogicalInstant {
            domain: scope.domain,
            nanos: 1,
        },
        task: future,
    };
    match work {
        WorkSpec::Runnable(_) => {}
        WorkSpec::At { deadline, task } => {
            assert_eq!(deadline.nanos, 1);
            drop(task);
        }
    }
    fn source(error: SourceError) {
        match error {
            SourceError::InvalidRange
            | SourceError::Overflow
            | SourceError::ScriptExhausted
            | SourceError::OutOfRange
            | SourceError::BackwardTime
            | SourceError::WrongDomain
            | SourceError::InvalidToken
            | SourceError::Stopped
            | SourceError::NotDue
            | SourceError::RegistrationFailed
            | SourceError::ClockFailed
            | SourceError::EntropyFailed
            | SourceError::TaskFailed => {}
        }
    }
    fn driver(error: DriverError) {
        match error {
            DriverError::NotQualified
            | DriverError::InvalidConfiguration
            | DriverError::InvalidToken
            | DriverError::Stopped
            | DriverError::NotDue
            | DriverError::Protocol
            | DriverError::Source(_) => {}
        }
    }
    source(SourceError::TaskFailed);
    driver(DriverError::NotQualified);
    fn events(event: Event) {
        match event {
            Event::SourceRead { .. }
            | Event::SourceDraw { .. }
            | Event::WorkRegistered { .. }
            | Event::WorkPolled { .. }
            | Event::WorkCancelled { .. }
            | Event::Outbound { .. }
            | Event::RpcPending { .. }
            | Event::RpcResolved { .. }
            | Event::Observation(_)
            | Event::SourceFailed(_)
            | Event::Stopped { .. } => {}
        }
    }
    events(Event::SourceFailed(SourceError::EntropyFailed));
    fn bounds(
        _: &dyn ElectionSources,
        _: &mut dyn ScheduledWork,
        _: &dyn ElectionNode,
        _: &dyn MemoryProtocolStore,
        _: &dyn TransportEndpoint,
        _: &dyn InstanceLogger,
    ) {
    }
    let _consumer = bounds;
    fn correlated_reply(
        endpoint: &mut dyn TransportEndpoint,
        rpc: RpcId,
        kind: ProtocolKind,
        bytes: ProtocolMessage,
    ) -> Result<MessageToken, DriverError> {
        endpoint.respond(rpc, kind, bytes)
    }
    let _typed_reply_consumer = correlated_reply;
}
