//! External runner kills this test process, avoiding Rust child-process globals.
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use glade_raft_adoption_api::{Action, Command, RequestId};
use glade_raft_adoption_proof::Cluster;
use glade_raft_disk::DiskStore;
use glade_raft_durability_api::{Binding, DurableImage, DurableStore, StoreError, StoredState};

fn command(sequence: u64) -> Command {
    Command {
        request: RequestId {
            scope: 7,
            resource: 100,
            incarnation: 1,
            principal: 1,
            sequence,
        },
        generation: if sequence == 1 { 0 } else { 1 },
        home: if sequence == 1 { 0 } else { 1 },
        policy_frontier: 0,
        action: if sequence == 1 {
            Action::Create {
                name: 40,
                home: 1,
                payload: 11,
            }
        } else {
            Action::Mutate { payload: 23 }
        },
    }
}

fn stop(marker: &str) {
    println!("{marker}");
    std::io::stdout().flush().unwrap();
    // Parent keeps stdin open until SIGKILL; no destructor or graceful stop runs.
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    panic!("parent must kill the active worker");
}

struct CutStore {
    inner: DiskStore,
    cut: bool,
}
impl DurableStore for CutStore {
    fn load(&mut self) -> Result<StoredState, StoreError> {
        self.inner.load()
    }
    fn persist(&mut self, revision: u64, image: DurableImage) -> Result<StoredState, StoreError> {
        let cut = self.cut && image.commit >= 3;
        let result = self.inner.persist(revision, image)?;
        if cut {
            stop("Q2_COMMIT_BEFORE_APPLY");
        }
        Ok(result)
    }
}

#[test]
#[ignore = "execute with the bounded external process-crash runner"]
fn q2_process_worker() {
    // Test-harness entry inputs only; production providers receive explicit paths.
    let root = std::env::var("GLADE_Q2_ROOT").unwrap();
    let mode = std::env::var("GLADE_Q2_MODE").unwrap();
    let mut stores: BTreeMap<u64, Box<dyn DurableStore>> = BTreeMap::new();
    for node in 1..=3 {
        let binding = Binding {
            scope: 7,
            node,
            voters: vec![1, 2, 3],
            application_profile: 1,
        };
        let path = Path::new(&root).join(format!("voter-{node}"));
        let inner = if mode.starts_with("write") {
            DiskStore::create_new(&path, binding)
        } else {
            DiskStore::open(&path, binding, None)
        }
        .unwrap();
        stores.insert(
            node,
            Box::new(CutStore {
                inner,
                cut: node == 1 && mode == "write-cut",
            }),
        );
    }
    let mut cluster = Cluster::recover(&[1, 2, 3], stores).ok().unwrap();
    if mode.starts_with("write") {
        cluster.campaign(1);
        cluster.drain();
        for sequence in [1, 2] {
            cluster.propose(1, command(sequence));
            cluster.drain();
            assert!(cluster.reply(1, command(sequence)).is_some());
        }
        assert_eq!(cluster.resource(1, 100).unwrap().payload, 23);
        stop("Q2_ACK 3 23");
    } else {
        assert_eq!(cluster.resource(1, 100).unwrap().payload, 23);
        let original = cluster.outcome(1, command(2).request).unwrap();
        assert_eq!(original.index, 3);
        cluster.campaign(1);
        cluster.drain();
        cluster.propose(1, command(2));
        cluster.drain();
        assert_eq!(cluster.reply(1, command(2)), Some(original));
        println!("Q2_RECOVERED 3 23");
    }
}
