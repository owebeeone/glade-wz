#[test]
fn actual_files_reject_torn_checksum_bounds_and_valid_checksum_structural_adversaries() {
    use super::*;
    use glade_raft_q3_api::conformance;
    fn checksum(bytes: &[u8]) -> u64 {
        let mut crc = 0_u64;
        for byte in bytes {
            crc ^= u64::from(*byte) << 56;
            for _ in 0..8 {
                crc = if crc & (1 << 63) != 0 {
                    (crc << 1) ^ 0x42F0_E1EB_A9EA_3693
                } else {
                    crc << 1
                };
            }
        }
        crc
    }
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-parser-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let path = root.join("node-1");
    let instance = Instance {
        scope: 7,
        group: 70,
        node: 1,
        application_profile: 2,
    };
    let initial = Image {
        term: 0,
        vote: 0,
        commit: 0,
        applied: 0,
        configuration: conformance::stable(),
        checkpoint: None,
        suffix: Vec::new(),
    };
    drop(V2DiskStore::create_new(&path, instance.clone(), initial).unwrap());
    let original = std::fs::read(&path).unwrap();
    assert_eq!(original.len(), 184);
    for cut in 0..original.len() {
        std::fs::write(&path, &original[..cut]).unwrap();
        assert!(
            matches!(
                V2DiskStore::open(&path, instance.clone(), None),
                Err(Error::Quarantined)
            ),
            "torn cut {cut}"
        );
        assert_eq!(std::fs::read(&path).unwrap(), original[..cut]);
    }
    for (offset, value) in [
        (16, 1),
        (16, u64::MAX),
        (56, u64::MAX),
        (64, 1),
        (72, 1),
        (80, 1),
        (88, 5),
        (104, 1),
        (144, 2),
        (152, 1),
        (160, 2),
        (168, u64::MAX),
    ] {
        let mut bad = original.clone();
        bad[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
        let end = bad.len() - 8;
        let crc = checksum(&bad[..end]);
        bad[end..].copy_from_slice(&crc.to_le_bytes());
        std::fs::write(&path, &bad).unwrap();
        assert!(
            matches!(
                V2DiskStore::open(&path, instance.clone(), None),
                Err(Error::Quarantined)
            ),
            "checksummed field {offset}={value}"
        );
        assert_eq!(std::fs::read(&path).unwrap(), bad);
    }
    for length in [u64::MAX, 16 * 1024 * 1024 + 1] {
        let mut bad = original.clone();
        bad[8..16].copy_from_slice(&length.to_le_bytes());
        std::fs::write(&path, &bad).unwrap();
        assert!(matches!(
            V2DiskStore::open(&path, instance.clone(), None),
            Err(Error::Quarantined)
        ));
    }
    let mut bad = original.clone();
    bad[24] ^= 1;
    std::fs::write(&path, &bad).unwrap();
    assert!(
        matches!(
            V2DiskStore::open(&path, instance, None),
            Err(Error::Quarantined)
        ),
        "bad checksum"
    );
    std::fs::remove_dir_all(root).unwrap();
}
