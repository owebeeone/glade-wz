//! Bounded memory-only Q1a experiment using actual synchronous raft-rs.
//!
//! Scope 7, administrator 1, writer 10 and fixed voters are trusted numeric
//! fixtures. No signatures, disk/restart, snapshots, automatic elections,
//! membership transitions, transport or production integration are qualified.
//! Manual campaigns do not eliminate raft-rs's ambient election-timeout
//! randomness; no election ticks are exercised here.
//!
//! Atomic Move is a partial RA-002/005 witness, not BeginMove/Activate. Its
//! private evidence comes from complete successor application and is checked
//! against the committed cut. Public frontier fields cannot mint evidence.

mod application;
mod cluster;
mod codec;

pub use application::Application;
pub use cluster::{Cluster, Proposal};

mod recovery;
mod voter;
