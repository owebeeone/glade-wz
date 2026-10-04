# IC-1 Consistency root-classification addendum

**Date:** 2026-10-04. **Role:** Restored originating Consistency reviewer process; not the original live reviewer.

**Verified tuple, unchanged at start and end:**

- Root: `f0d0b1ebc396e70d5f0e2beb817eb5da232230fb`
- Glade: `6a0cc5a78da38a023615f6adc5fc354bba4af0b4`
- Glial: `348eed97cd1ee4f677ea2866dfabe5a81cbebee1`
- Discovery: `1054cfbb6871f4e51c6d9e80bfa0a1fe77956d69`
- External Gyld: `64666e8b1caadde8922b9d42163afbab90655c65`

**Classification: original Consistency P2-1 is ARCHITECTURAL.**

At original Glade `885249a2a093e082aad6e1dc9936a7fd5c54052b`, `contracts/crdt-admission-core/src/types.rs:263–278` represents lookup requests/replies using only immutable commit identity and associated fields. After L1 returns Unknown, L2 for the unchanged plan has identical representable identity. No implementation confined to that event boundary can distinguish a duplicate L1 reply from a legitimate L2 reply.

The defect therefore concerns the adequacy of the kernel–host continuation interface and state model, rather than an incorrect implementation of an adequate interface. Repair requires changing both sides of the boundary and defining invocation allocation, retirement and restoration semantics.

The corrected `types.rs:274–291,389–391` introduces separate `lookup_id` identity and corresponding outstanding-state ownership. Contract `253–267` defines its relationship to immutable `plan_id`, retirement and stale-reply handling; contract `90` defines restored high-water obligations. These are architectural interface corrections even though they are bounded and preserve the accepted admission semantics.

My earlier phrase “internal continuation-interface root” should be read with this explicit architectural classification. Its closure verdict remains unchanged. Architectural roots remain part of the review history after correction.

**Root accounting:** This independently classified lookup root, the supplied original Safety architectural retention-recovery root, and the supplied fresh Safety architectural terminal-negative commit-lifecycle root total **three architectural roots on the IC-1 typed-contract object**, assuming the supplied distinct-root classifications stand. That accounting reaches the review-loop’s third-root stop condition. I independently adjudicated only the lookup classification here; the other two classifications are legitimate merged-phase inputs supplied by the lane owner, not findings independently re-reviewed in this addendum.

Original Consistency P2-2 remains non-architectural regression coverage; P3-1 remains non-architectural normative-wording inconsistency.

No files were changed, no tests/builds/network/Git mutations were performed, and no current peer reports were read.