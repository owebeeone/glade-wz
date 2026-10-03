//! Explicit external SIGKILL worker; not part of ordinary test execution.
use glade_raft_adoption_proof::q3::Q3Session;
use glade_raft_disk::v2::V2StoreFactory;
use glade_raft_q3_api::{
    Action, Change, CheckpointStore, Command, ConfigReceipt, Control, Error, Image, Instance,
    QualificationSession, ReplayResult, State, StoreLifecycle, StoredEntry, conformance,
};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::path::PathBuf;

fn instance(node: u64) -> Instance {
    Instance {
        scope: 7,
        group: 70,
        node,
        application_profile: 2,
    }
}
fn initial() -> Image {
    Image {
        term: 0,
        vote: 0,
        commit: 0,
        applied: 0,
        configuration: conformance::stable(),
        checkpoint: None,
        suffix: Vec::new(),
    }
}
fn pause(cut: &str) {
    println!("CUT {cut}");
    std::io::stdout().flush().unwrap();
    let mut byte = [0];
    std::io::stdin()
        .read_exact(&mut byte)
        .expect("parent must SIGKILL this paused worker");
    panic!("kill worker unexpectedly resumed");
}
struct CutStore {
    store: Box<dyn CheckpointStore>,
    kind: String,
    node: u64,
}
impl CheckpointStore for CutStore {
    fn load(&mut self) -> Result<State, Error> {
        self.store.load()
    }
    fn publish(&mut self, revision: u64, image: Image) -> Result<State, Error> {
        let saved = self.store.publish(revision, image)?;
        if self.kind == "joint-before-apply"
            && self.node == 1
            && saved.image.commit == 4
            && saved.image.applied < 4
        {
            let entry = saved
                .image
                .suffix
                .iter()
                .find(|entry| entry.index == 4)
                .expect("complete original joint entry");
            println!("CONFIG_ENTRY {entry:?}");
            pause("joint-before-apply");
        }
        if self.kind == "snapshot-before-apply"
            && self.node == 4
            && saved
                .image
                .checkpoint
                .as_ref()
                .is_some_and(|cp| cp.index == 3)
        {
            pause("snapshot-before-apply");
        }
        Ok(saved)
    }
}
struct Factory {
    inner: V2StoreFactory,
    kind: String,
}
impl StoreLifecycle for Factory {
    fn create(
        &mut self,
        instance: Instance,
        image: Image,
    ) -> Result<Box<dyn CheckpointStore>, Error> {
        let node = instance.node;
        Ok(Box::new(CutStore {
            store: self.inner.create(instance, image)?,
            kind: self.kind.clone(),
            node,
        }))
    }
    fn open(
        &mut self,
        instance: Instance,
        floor: Option<u64>,
    ) -> Result<Box<dyn CheckpointStore>, Error> {
        let node = instance.node;
        Ok(Box::new(CutStore {
            store: self.inner.open(instance, floor)?,
            kind: self.kind.clone(),
            node,
        }))
    }
}
fn app_original(session: &mut Q3Session, command: Command, index: u64) {
    let receipt = session
        .outcome(command.request)
        .unwrap()
        .expect("original application outcome");
    assert_eq!(receipt.index, index);
    println!("APP_COMMAND {command:?}");
    println!("APP_RECEIPT {receipt:?}");
    println!("APP_ENTRY {:?}", session.applied_entry(index).unwrap());
}
fn config_original(session: &Q3Session, receipt: &ConfigReceipt) {
    println!("CONFIG_INTENT {:?}", receipt.intent);
    println!("CONFIG_RECEIPT {receipt:?}");
    println!(
        "CONFIG_ENTRY {:?}",
        session.applied_entry(receipt.index).unwrap()
    );
}
#[test]
#[ignore = "Q3 actual SIGKILL tier; execute only through process-crash-q3.py"]
fn q3_process_worker() {
    let root = PathBuf::from(std::env::var("Q3_ROOT").expect("injected root"));
    let mode = std::env::var("Q3_MODE").expect("injected mode");
    let kind = std::env::var("Q3_KIND").expect("injected cut");
    assert!(matches!(
        kind.as_str(),
        "ack" | "joint-before-apply" | "snapshot-before-apply"
    ));
    let mut factory = Factory {
        inner: V2StoreFactory::new(root.clone()),
        kind: if mode == "start" {
            kind.clone()
        } else {
            String::new()
        },
    };
    let mut stores = BTreeMap::new();
    for node in [1, 2, 3] {
        let store = if mode == "start" {
            factory.create(instance(node), initial())
        } else {
            factory.open(instance(node), None)
        }
        .unwrap();
        stores.insert(node, store);
    }
    if mode == "recover" && root.join("node-4").exists() {
        stores.insert(4, factory.open(instance(4), None).unwrap());
    }
    let mut session = Q3Session::recover(stores, Box::new(factory), vec![1, 2]).unwrap();
    if mode == "start" {
        conformance::accepted_create(&mut session);
        if kind == "snapshot-before-apply" {
            let cp = session.checkpoint().unwrap();
            session.install(cp).unwrap();
            let added = session
                .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
                .unwrap()
                .unwrap();
            let mutation = conformance::command(2, Action::Mutate { payload: 23 });
            conformance::submit_accepted(
                &mut session,
                mutation,
                glade_raft_q3_api::Resource {
                    payload: 23,
                    ..conformance::created_resource(1)
                },
            );
            app_original(&mut session, mutation, 4);
            config_original(&session, &added);
            std::io::stdout().flush().unwrap();
            session.control(Control::CatchUp { node: 4 }).unwrap();
            panic!("snapshot cut hook did not fire");
        }
        let added = session
            .configure(conformance::intent(1, 0, Change::AddLearner { node: 4 }))
            .unwrap()
            .unwrap();
        session.control(Control::CatchUp { node: 4 }).unwrap();
        let joint = conformance::intent(
            2,
            added.index,
            Change::EnterJoint {
                voters: vec![1, 2, 4],
            },
        );
        if kind == "joint-before-apply" {
            app_original(&mut session, conformance::create(), 2);
            println!("CONFIG_INTENT {joint:?}");
            std::io::stdout().flush().unwrap();
            session.configure(joint).unwrap();
            panic!("joint cut hook did not fire");
        }
        let joint = session.configure(joint).unwrap().unwrap();
        let mutation = conformance::command(2, Action::Mutate { payload: 23 });
        conformance::submit_accepted(
            &mut session,
            mutation,
            glade_raft_q3_api::Resource {
                payload: 23,
                ..conformance::created_resource(1)
            },
        );
        let cp = session.checkpoint().unwrap();
        session.install(cp).unwrap();
        app_original(&mut session, mutation, 5);
        config_original(&session, &joint);
        pause("ack");
    }
    assert_eq!(mode, "recover");
    let (command, index, config_index, intent) = match kind.as_str() {
        "joint-before-apply" => (
            conformance::create(),
            2,
            4,
            conformance::intent(
                2,
                3,
                Change::EnterJoint {
                    voters: vec![1, 2, 4],
                },
            ),
        ),
        "snapshot-before-apply" => (
            conformance::command(2, Action::Mutate { payload: 23 }),
            4,
            3,
            conformance::intent(1, 0, Change::AddLearner { node: 4 }),
        ),
        _ => (
            conformance::command(2, Action::Mutate { payload: 23 }),
            5,
            4,
            conformance::intent(
                2,
                3,
                Change::EnterJoint {
                    voters: vec![1, 2, 4],
                },
            ),
        ),
    };
    println!("RECOVERED_APP_COMMAND {command:?}");
    println!(
        "RECOVERED_APP_LOOKUP {:?}",
        session.outcome(command.request).unwrap().unwrap()
    );
    println!(
        "RECOVERED_APP_RETRY {:?}",
        session.submit(command).unwrap().unwrap()
    );
    let entry = session.applied_entry(index).unwrap();
    println!("RECOVERED_APP_ENTRY {entry:?}");
    println!("RECOVERED_APP_REPLAY {:?}", session.replay(entry).unwrap());
    println!("RECOVERED_CONFIG_INTENT {intent:?}");
    println!(
        "RECOVERED_CONFIG_LOOKUP {:?}",
        session.configuration_outcome(intent.key).unwrap().unwrap()
    );
    println!(
        "RECOVERED_CONFIG_RETRY {:?}",
        session.configure(intent).unwrap().unwrap()
    );
    let entry = session.applied_entry(config_index).unwrap();
    println!("RECOVERED_CONFIG_ENTRY {entry:?}");
    println!(
        "RECOVERED_CONFIG_REPLAY {:?}",
        session.replay(entry).unwrap()
    );
    assert!(matches!(
        session.replay(session.applied_entry(1).unwrap()).unwrap(),
        ReplayResult::Noop { index: 1 }
    ));
    if kind == "snapshot-before-apply" {
        let view = session.view().unwrap();
        let learner = view.nodes.iter().find(|cut| cut.node == 4).unwrap();
        assert_eq!(learner.applied, view.committed);
        assert!(learner.snapshot_index.is_some());
    }
    let _original_entry_type: Option<StoredEntry> = None;
}
