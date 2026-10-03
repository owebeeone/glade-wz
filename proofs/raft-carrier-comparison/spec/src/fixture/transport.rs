//! Pure controlled queue/RPC scaffolding; opaque bytes are never interpreted.
use glade_carrier_api::*;
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};
pub(super) struct RpcState {
    request: MessageToken,
    expected_peer: NodeScope,
    result: Option<Result<ProtocolMessage, RpcError>>,
    waker: Option<Waker>,
}
struct Response(Rc<RefCell<RpcState>>);
impl Future for Response {
    type Output = Result<ProtocolMessage, RpcError>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.borrow_mut();
        if let Some(result) = state.result.take() {
            Poll::Ready(result)
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
pub(super) struct Queued {
    pub(super) view: MessageView,
    message: ProtocolMessage,
}
pub(super) struct Network {
    pub(super) scopes: BTreeMap<u64, NodeScope>,
    pub(super) stopped: BTreeSet<NodeScope>,
    pub(super) sequence: u64,
    pub(super) messages: BTreeMap<MessageToken, Queued>,
    pub(super) rpcs: BTreeMap<RpcId, Rc<RefCell<RpcState>>>,
}
impl Network {
    fn live(&self, scope: NodeScope) -> bool {
        self.scopes.get(&scope.node.node) == Some(&scope) && !self.stopped.contains(&scope)
    }
    fn endpoint(&self, scope: NodeScope) -> Result<(), DriverError> {
        if self.scopes.get(&scope.node.node) != Some(&scope) {
            return Err(DriverError::InvalidToken);
        }
        if self.stopped.contains(&scope) {
            return Err(DriverError::Stopped);
        }
        Ok(())
    }
    fn token(&self, token: MessageToken) -> Result<(), DriverError> {
        if !self.live(token.issuer)
            || !self.live(token.destination)
            || !self.messages.contains_key(&token)
        {
            return Err(DriverError::InvalidToken);
        }
        Ok(())
    }
    fn rpc(&self, rpc: RpcId) -> Result<(), DriverError> {
        let state = self
            .rpcs
            .get(&rpc)
            .ok_or(DriverError::InvalidToken)?
            .borrow();
        self.token(state.request)?;
        if state.request.issuer != rpc.scope
            || state.expected_peer != state.request.destination
            || self
                .messages
                .get(&state.request)
                .is_none_or(|queued| queued.view.rpc != Some(rpc))
            || state.result.is_some()
        {
            return Err(DriverError::InvalidToken);
        }
        Ok(())
    }
}
// Complete mutations before invoking a saved wake, which may run caller code.
fn settle(state: Rc<RefCell<RpcState>>, result: Result<ProtocolMessage, RpcError>) {
    let waker = {
        let mut state = state.borrow_mut();
        state.result = Some(result);
        state.waker.take()
    };
    if let Some(waker) = waker {
        waker.wake();
    }
}
#[derive(Clone)]
pub struct Endpoint {
    pub(super) scope: NodeScope,
    pub(super) network: Rc<RefCell<Network>>,
}
impl Endpoint {
    fn emit_inner(
        &mut self,
        to: NodeKey,
        kind: ProtocolKind,
        message: ProtocolMessage,
        rpc: Option<RpcId>,
        reply_to: Option<RpcId>,
    ) -> Result<MessageToken, DriverError> {
        let mut network = self.network.borrow_mut();
        network.endpoint(self.scope)?;
        let destination = *network.scopes.get(&to.node).ok_or(DriverError::Protocol)?;
        if !network.live(destination) {
            return Err(DriverError::InvalidToken);
        }
        if to.group != self.scope.node.group || message.carrier != self.scope.carrier {
            return Err(DriverError::Protocol);
        }
        let sequence = network
            .sequence
            .checked_add(1)
            .ok_or(SourceError::Overflow)?;
        let token = MessageToken {
            issuer: self.scope,
            destination,
            sequence,
        };
        network.sequence = sequence;
        network.messages.insert(
            token,
            Queued {
                view: MessageView {
                    token,
                    kind,
                    state: MessageState::Held,
                    rpc,
                    reply_to,
                },
                message,
            },
        );
        Ok(token)
    }
}
impl TransportEndpoint for Endpoint {
    fn emit(
        &mut self,
        to: NodeKey,
        kind: ProtocolKind,
        message: ProtocolMessage,
    ) -> Result<MessageToken, DriverError> {
        self.emit_inner(to, kind, message, None, None)
    }
    fn request(
        &mut self,
        to: NodeKey,
        kind: ProtocolKind,
        message: ProtocolMessage,
    ) -> Result<PendingRpc, DriverError> {
        let sequence = {
            let network = self.network.borrow();
            network.endpoint(self.scope)?;
            network
                .sequence
                .checked_add(1)
                .ok_or(SourceError::Overflow)?
        };
        let id = RpcId {
            scope: self.scope,
            sequence,
        };
        let token = self.emit_inner(to, kind, message, Some(id), None)?;
        let state = Rc::new(RefCell::new(RpcState {
            request: token,
            expected_peer: token.destination,
            result: None,
            waker: None,
        }));
        self.network.borrow_mut().rpcs.insert(id, state.clone());
        Ok(PendingRpc {
            id,
            message: token,
            response: Box::pin(Response(state)),
        })
    }
    fn respond(
        &mut self,
        rpc: RpcId,
        kind: ProtocolKind,
        message: ProtocolMessage,
    ) -> Result<MessageToken, DriverError> {
        {
            let network = self.network.borrow();
            network.endpoint(self.scope)?;
            network.rpc(rpc)?;
            let state = network
                .rpcs
                .get(&rpc)
                .ok_or(DriverError::InvalidToken)?
                .borrow();
            if state.expected_peer != self.scope
                || state.request.issuer != rpc.scope
                || network.scopes.get(&self.scope.node.node) != Some(&self.scope)
                || network.scopes.get(&rpc.scope.node.node) != Some(&rpc.scope)
            {
                return Err(DriverError::InvalidToken);
            }
            let request = network
                .messages
                .get(&state.request)
                .ok_or(DriverError::InvalidToken)?;
            if request.view.state != MessageState::Consumed
                || request.view.token.destination != self.scope
            {
                return Err(DriverError::InvalidToken);
            }
            let expected_kind = match request.view.kind {
                ProtocolKind::VoteRequest => ProtocolKind::VoteResponse,
                ProtocolKind::AppendRequest => ProtocolKind::AppendResponse,
                ProtocolKind::Other => ProtocolKind::Other,
                _ => return Err(DriverError::Protocol),
            };
            if message.carrier != self.scope.carrier || kind != expected_kind {
                return Err(DriverError::Protocol);
            }
        }
        self.emit_inner(rpc.scope.node, kind, message, None, Some(rpc))
    }
    fn take(&mut self, token: MessageToken) -> Result<Inbound, DriverError> {
        if token.destination != self.scope {
            return Err(DriverError::InvalidToken);
        }
        let mut network = self.network.borrow_mut();
        network.endpoint(self.scope)?;
        network.token(token)?;
        let queued = network
            .messages
            .get_mut(&token)
            .ok_or(DriverError::InvalidToken)?;
        if queued.view.state != MessageState::Deliverable {
            return Err(DriverError::InvalidToken);
        }
        queued.view.state = MessageState::Consumed;
        Ok(Inbound {
            token,
            message: queued.message.clone(),
            rpc: queued.view.rpc,
            reply_to: queued.view.reply_to,
        })
    }
    fn control(&mut self, action: TransportAction) -> Result<Vec<Event>, DriverError> {
        let mut network = self.network.borrow_mut();
        network.endpoint(self.scope)?;
        match action {
            TransportAction::Release(token)
            | TransportAction::Hold(token)
            | TransportAction::Drop(token)
            | TransportAction::Duplicate(token) => {
                if token.issuer != self.scope {
                    return Err(DriverError::InvalidToken);
                }
                network.token(token)?;
                let queued = network
                    .messages
                    .get(&token)
                    .ok_or(DriverError::InvalidToken)?;
                if matches!(
                    queued.view.state,
                    MessageState::Consumed | MessageState::Dropped
                ) {
                    return Err(DriverError::InvalidToken);
                }
                if matches!(action, TransportAction::Duplicate(_)) {
                    let sequence = network
                        .sequence
                        .checked_add(1)
                        .ok_or(SourceError::Overflow)?;
                    let mut view = queued.view;
                    view.token.sequence = sequence;
                    view.state = MessageState::Held;
                    let message = queued.message.clone();
                    network.sequence = sequence;
                    network
                        .messages
                        .insert(view.token, Queued { view, message });
                    return Ok(vec![Event::Outbound { message: view }]);
                }
                network
                    .messages
                    .get_mut(&token)
                    .ok_or(DriverError::InvalidToken)?
                    .view
                    .state = match action {
                    TransportAction::Release(_) => MessageState::Deliverable,
                    TransportAction::Hold(_) => MessageState::Held,
                    _ => MessageState::Dropped,
                };
                Ok(Vec::new())
            }
            TransportAction::Resolve { rpc, reply } => {
                if rpc.scope != self.scope || reply.destination != self.scope {
                    return Err(DriverError::InvalidToken);
                }
                network.token(reply)?;
                network.rpc(rpc)?;
                let queued = network
                    .messages
                    .get(&reply)
                    .ok_or(DriverError::InvalidToken)?;
                if queued.view.state != MessageState::Deliverable {
                    return Err(DriverError::InvalidToken);
                }
                let pending = network
                    .rpcs
                    .get(&rpc)
                    .ok_or(DriverError::InvalidToken)?
                    .borrow();
                if queued.view.reply_to != Some(rpc)
                    || queued.view.rpc.is_some()
                    || queued.view.token.issuer != pending.expected_peer
                    || queued.view.token.destination != pending.request.issuer
                    || network.scopes.get(&pending.expected_peer.node.node)
                        != Some(&pending.expected_peer)
                    || network.scopes.get(&rpc.scope.node.node) != Some(&rpc.scope)
                    || queued.message.carrier != self.scope.carrier
                {
                    return Err(DriverError::InvalidToken);
                }
                let result = queued.message.clone();
                drop(pending);
                let state = network.rpcs.remove(&rpc).ok_or(DriverError::InvalidToken)?;
                network
                    .messages
                    .get_mut(&reply)
                    .ok_or(DriverError::InvalidToken)?
                    .view
                    .state = MessageState::Consumed;
                drop(network);
                settle(state, Ok(result));
                Ok(vec![Event::RpcResolved { rpc }])
            }
            TransportAction::Timeout(rpc) | TransportAction::Cancel(rpc) => {
                if rpc.scope != self.scope {
                    return Err(DriverError::InvalidToken);
                }
                network.rpc(rpc)?;
                let state = network.rpcs.remove(&rpc).ok_or(DriverError::InvalidToken)?;
                drop(network);
                settle(
                    state,
                    Err(if matches!(action, TransportAction::Timeout(_)) {
                        RpcError::TimedOut
                    } else {
                        RpcError::Cancelled
                    }),
                );
                Ok(vec![Event::RpcResolved { rpc }])
            }
        }
    }
    fn messages(&self) -> Vec<MessageView> {
        self.network
            .borrow()
            .messages
            .values()
            .filter(|queued| {
                queued.view.token.issuer == self.scope
                    || queued.view.token.destination == self.scope
            })
            .map(|queued| queued.view)
            .collect()
    }
    fn pending_rpcs(&self) -> Vec<RpcId> {
        self.network
            .borrow()
            .rpcs
            .keys()
            .filter(|rpc| rpc.scope == self.scope)
            .copied()
            .collect()
    }
    fn stop(&mut self) -> Result<(), DriverError> {
        let detached: Vec<_> = {
            let mut network = self.network.borrow_mut();
            if network.scopes.get(&self.scope.node.node) != Some(&self.scope) {
                return Err(DriverError::InvalidToken);
            }
            network.stopped.insert(self.scope);
            let owned: Vec<_> = network
                .rpcs
                .keys()
                .filter(|rpc| rpc.scope == self.scope)
                .copied()
                .collect();
            let detached = owned
                .into_iter()
                .filter_map(|rpc| network.rpcs.remove(&rpc))
                .collect();
            for queued in network.messages.values_mut() {
                if (queued.view.token.issuer == self.scope
                    || queued.view.token.destination == self.scope)
                    && !matches!(
                        queued.view.state,
                        MessageState::Consumed | MessageState::Dropped
                    )
                {
                    queued.view.state = MessageState::Dropped;
                }
            }
            detached
        };
        for state in detached {
            settle(state, Err(RpcError::Cancelled));
        }
        Ok(())
    }
}
