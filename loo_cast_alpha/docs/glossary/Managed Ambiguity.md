---
canonical_name: Managed Ambiguity
status: WIP-draft
aliases: []
---

Managed Ambiguity means uncertainty is treated as explicit design material rather than hidden noise.
During active model discovery, unresolved edges are named and tracked while hard invariants are still preserved.
This avoids premature over-locking and also avoids unstructured drift.
Managed ambiguity is not permission to leave contradictions invisible.
It should produce named open questions, ledger entries, TODOs, or phase-bound decisions, and it should identify which
parts are still uncertain without weakening already-hard boundaries.

Useful examples:

- keep `panic-fast` vs `fail-fast` wording unresolved while still documenting concrete failure modes
- keep slot-policy names provisional while still forbidding cycles and unresolved singleton-critical ownership
- keep runtime dynamism policy open while still forbidding arbitrary post-lock graph mutation

See also:

- [[Closed Runtime and Open Design]]
- [[Polycentric Pillars]]

#glossary
