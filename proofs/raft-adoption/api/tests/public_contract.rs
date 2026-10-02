use glade_raft_adoption_api::{
    Action, ApplyError, Command, CommittedMachine, Outcome, Receipt, RequestId, Resource,
};

struct Recorder;

impl CommittedMachine for Recorder {
    fn apply(
        &mut self,
        _index: u64,
        command: Option<Command>,
    ) -> Result<Option<Receipt>, ApplyError> {
        assert!(command.is_none());
        Ok(None)
    }

    fn lookup(&self, _request: RequestId) -> Option<Receipt> {
        None
    }
}

#[test]
fn ra_contract_host_is_substitutable_without_raft_types() {
    let mut recorder = Recorder;
    let host: &mut dyn CommittedMachine = &mut recorder;
    assert_eq!(host.apply(1, None), Ok(None));
    let request = RequestId {
        scope: 7,
        resource: 100,
        incarnation: 1,
        principal: 1,
        sequence: 1,
    };
    assert_eq!(host.lookup(request), None);
}

#[test]
fn complete_boundary_types_are_owned_without_raft_or_transport_types() {
    let request = RequestId {
        scope: 7,
        resource: 100,
        incarnation: 1,
        principal: 1,
        sequence: 1,
    };
    let command = Command {
        request,
        generation: 0,
        home: 0,
        policy_frontier: 0,
        action: Action::Create {
            name: 40,
            home: 1,
            payload: 11,
        },
    };
    let resource = Resource {
        id: 100,
        name: 40,
        incarnation: 1,
        generation: 1,
        home: 1,
        payload: 11,
        retired: false,
    };
    let receipt = Receipt {
        request,
        index: 2,
        outcome: Outcome::Accepted(resource),
    };
    assert_eq!(command.request, receipt.request);
    assert_eq!(receipt.outcome, Outcome::Accepted(resource));
}
