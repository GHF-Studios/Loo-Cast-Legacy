# Rhai Value Semantics and AccessCell Notes

#tech_glossary

Status note:
This page is provisional legacy/quarantine evidence, not settled target doctrine.
The value-semantics and `AccessCell` model should be preserved as design signal while the final bridge shape remains
open.

Related glossary terms:

- [USF Instantiation Scripts](USF%20Instantiation%20Scripts.md)
- [USF Runtime](USF%20Runtime.md)
- [Runtime Substrate](Runtime%20Substrate.md)
- [Rhai Reflection Macro Surface Notes](Rhai%20Reflection%20Macro%20Surface%20Notes.md)
- [Rhai Bridge Domains and Access Provider Notes](Rhai%20Bridge%20Domains%20and%20Access%20Provider%20Notes.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)

Quarantine extraction (`TMP_rhai_semantic_reset_quarantine`) with high signal but provisional authority:

1. Value semantics vocabulary is explicit:
   `Clone`, `Owned`, `Ref`, `Mut`, `ScopedOwned`, `ScopedRef`, `ScopedMut`.
2. These modes describe runtime access behavior, not persistent storage classes.
3. `AccessCell` is the runtime borrowing boundary between Rust and script-facing flows.
4. Scoped modes rely on provider-managed frame/window lifecycles.
5. Stale/invalid access and lifecycle contract violations are panic-fast.

Concrete behavior extracted from the quarantine implementation:

1. `AccessCell` state machine is explicit: `Taken`, `Available`, `Writing`, and `Reading { ref_count }`.
2. Transition operations are explicit and validated (`start_read`, `end_read`, `start_write`, `end_write`, `take`).
3. Guard discipline is strict: dropping read/write guards without explicit `end_*` invalidation panics.
4. Busy contention uses bounded retry/yield and then panics on exceeded wait budget.
5. Unsupported provider methods/argument shapes in `AccessCellProvider` implementations panic immediately.
6. Scoped provider usage relies on same-frame lifecycle discipline (`start_access` and `end_access` in one execution
   window).

`AccessCell` behavior surface that matters for future rewrite/migration:

- explicit start/end read and write transitions
- no overlap of write with any active read/write path
- optional take/invalidation path for scoped borrow recovery
- contention guardrail (bounded busy wait, then panic)

Provider-bridge signal worth preserving:

1. Bridge/runtime code uses `AccessCellProvider` to gate access to `World`, `Commands`, `EntityCommands`, query/message
   facades, and related scoped wrappers.
2. Provider contracts are method-keyed and request-shaped; invalid method names or mismatched request payloads
   hard-fail.
3. Lifetime-erased access windows are intentionally localized and explicitly closed.
4. This pattern aligns with script-safe contextual projection semantics, even though concrete profile plumbing is still
   incomplete.

Current tree divergence to account for:

1. Quarantine notes describe a richer `value_semantics` + `AccessCell` runtime surface.
2. Active legacy `rhai_binding` still references value-semantics metadata in reflection types, but full runtime
   `value_semantics` module coverage is not currently mirrored in the active backend tree.
3. Treat this as an explicit migration/reconciliation gap, not as settled architecture.

Current design tension (intentional and unresolved):

1. Thin-bridge direction:
   keep Rhai-facing semantics minimal and treat facade/bridge as mostly translation glue. Simpler but less flexible
   rhai-facing semantics.
2. Rich-semantics direction:
   retain a deeper value-semantics model (`Scoped*`, lifecycle windows, explicit access policy) as a first-class
   scripting contract. More complex but also more flexible rhai-facing semantics.
3. Integration constraint:
   whichever direction wins must compose cleanly with [[Capability Declaration]]s and
   declaration/type/trait/callback-tailored `ctx` capability-object subgraphs.
4. Reflection layer constraint:
   value-semantics decisions must still fit the macro reflection/registration topology documented in
   [Rhai Reflection Macro Surface Notes](Rhai%20Reflection%20Macro%20Surface%20Notes.md).

Both directions have merit; final shape is still open and should be decided after more runtime + ergonomics review.

Legacy source pointers:

-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/documents/markdown_summary/rhai_value_semantics.md`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/value_semantics/modes.rs`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/value_semantics/access_cell.rs`

- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/meta/monomorphized/type_.rs`
- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/meta/monomorphized/generic_.rs`

Rustdoc anchors:

- `crates/core_mod/src/spec/rhai.rs`
