# IC-3B full Code review — factual erratum

**Date:** 2026-10-04  
**Applies to:** `dev-docs/history/GladeIndependentCrdtProductionIntegrationPhysicalHostFull-ReviewCode-2.md`

The evidence paragraph incorrectly labels the three new retained-cut crash boundaries as observation boundaries. They are **original native Fence transaction boundaries**.

At pinned Glade HEAD, `node/src/independent/records/tests/retained_cut_crash.rs:61–66` selects:

1. `PairCut::Intent(FloorCut::BeforeTemp)` — before the Fence intent’s temporary-file creation.
2. `PairCut::Intent(FloorCut::DirectorySynced)` — after synchronization of the Fence intent publication.
3. `PairCut::Selection(FloorCut::DirectorySynced)` — after synchronization of the Fence selection publication.

The hook at lines 81–82 explicitly requires `kind == DiskOperation::Fence`. These deaths occur on the original Fence path following retained observation selection; they do not constitute three additional observation-publication cuts.

The pinned remediation-2 run log records this parent regression initially failing with reopen `Integrity`, subsequently passing, and passing again in the final 29-test records selection. Its loop covers all three boundaries. The separate 26 observation cuts and overall 151-cut accounting are unchanged.

**Verdict remains GO.** This corrects testimony about the physical boundaries exercised; it introduces no implementation finding or new remediation round. Verification was read-only, using source and recorded evidence. No tests, builds, writes, or peer reports were involved.

| Repository | START | END |
|---|---|---|
| Workspace root | `f0b1c325f9e0eff27b4eefadac8084bff3505e73` | `f0b1c325f9e0eff27b4eefadac8084bff3505e73` |
| Glade | `a47691598df648eb8c9554b27f3d06b0cffcf596` | `a47691598df648eb8c9554b27f3d06b0cffcf596` |
| Glial | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` | `348eed97cd1ee4f677ea2866dfabe5a81cbebee1` |
| Glade-discover | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` | `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69` |
| External Gyld | `400cedcf1fff74128f366af758a0c389a23435d7` | `400cedcf1fff74128f366af758a0c389a23435d7` |