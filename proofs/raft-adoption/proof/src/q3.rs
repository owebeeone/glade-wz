//! Bounded Q3 actual-RawNode session with injected local stores and lifecycle.
mod carrier;
pub(crate) mod encoding;
mod machine;
mod recovery;
mod session;
mod storage;
mod voter;
use glade_raft_q3_api::{Command, StoreLifecycle};
use raft::eraftpb::Message;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub struct Q3Session {
    nodes: BTreeMap<u64, voter::Voter>,
    lifecycle: Box<dyn StoreLifecycle>,
    messages: VecDeque<Message>,
    disconnected: BTreeSet<u64>,
    observed: BTreeSet<u64>,
    authorities: Vec<u64>,
    lose_reply: bool,
    queued: Option<Command>,
}

mod machine_tests;

mod voter_tests;
