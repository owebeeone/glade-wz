//! Bounded Q1a memory, Q2 disk/recovery and Q3 configuration/snapshot experiment.
//!
//! Q3 uses actual raft-rs with injected V2 local stores, authorized numeric
//! fixture learner/joint transitions, and full original application/configuration
//! history inside checkpoints. Q3 acceptance status and exact reviewed scope are
//! recorded in dev-docs/GladeRaftQualification-ReviewCycle.md.
//! Q2/Q3 exercise named local APFS I/O and actual process-termination cuts while
//! the kernel remains running; power-loss and independent physical quorum
//! survival are not certified. No signatures, automatic elections, transport,
//! external effects or production integration are qualified.
//! Manual campaigns still use raft-rs's ambient election-timeout randomness;
//! the harness exercises no election ticks.
//!
//! Atomic Move is a partial RA-002/005 witness, not BeginMove/Activate. Its
//! private evidence comes from complete successor application and is checked
//! against the committed cut. Public frontier fields cannot mint evidence.

mod application;
mod cluster;
mod codec;
pub mod q3;

pub use application::Application;
pub use cluster::{Cluster, Proposal};

mod recovery;
mod voter;
