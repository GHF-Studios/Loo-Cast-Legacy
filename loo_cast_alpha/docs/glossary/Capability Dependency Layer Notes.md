# Capability Dependency Layer Notes

#tech_glossary

Related glossary terms:

- [Capability](Capability.md)
- [Capability Declaration](Capability%20Declaration.md)
- [Rhai Capability](Rhai%20Capability.md)
- [Capability Runtime](Capability%20Runtime.md)
- [USF Instantiation Scripts](USF%20Instantiation%20Scripts.md)
- [USF Runtime](USF%20Runtime.md)
- [USF Contract Runtime Boundary Notes](USF%20Contract%20Runtime%20Boundary%20Notes.md)
- [Runtime Lock](Runtime%20Lock.md)

Current canonical framing (draft):

1. Rhai declaration and Rust runtime execution coexist in one runtime, but they are phase-separated.
2. Declaration phase: load typed source material, expose declaration/type/trait/callback-scoped `ctx`, emit capability
   node declaration material, validate, then lock-transition.
3. Execution phase: run [[Capability Instance]]s in USF runtime evolution after lock.
4. Callback invocation paths bridge these phases without collapsing them into one undifferentiated model.

Dependency layers (do not conflate):

1. Mod dependency layer:
   mod graph/provider ownership and compatibility.
2. Declaration dependency layer:
   declaration/type/trait/callback-scoped `ctx` path access and declaration-time dependency requirements.
3. Runtime interaction layer:
   post-lock interactions among [[Capability Instance]]s (channels/events/callback-driven behavior).

Declaration entrypoint access vs callback access:

1. Declaration entrypoints execute with declaration-scoped access (`decl_ctx`).
2. Rhai callbacks declared inside that artifact execute later with callback-scoped access (`cb_ctx`), not implicit
   carry-over from `decl_ctx`.
3. `decl_ctx` and each `cb_ctx` are projections from the same capability graph model, but they can differ in effective
   path masks after allow/deny resolution.
4. Callback path access can be locked/gated at runtime policy boundaries; previously available declaration paths are not
   automatically available to callbacks.
5. Access outside the resolved effective callback path mask is invalid and hard-fails on invocation.

Current rules:

1. Capability declarations depend on projected capability handles via `ctx`, not on direct runtime instance references.
2. Declaration dependencies are expressed as path/contract requirements, not ad-hoc capability-object pointers.
3. Missing or denied required `ctx` paths hard-fail during declaration execution/validation.
4. Unresolved providers or ownership conflicts hard-fail before activation completes.
5. Runtime interaction remains post-lock and cannot retroactively mutate declaration structure.
6. Seam/event mapping must be phase-tagged:
   declaration seams (`rhai_binding` and declaration validation/lock path) vs execution seams (
   `usf <-> chunk/player/core`).
7. Callback hooks are declaration outputs, and each hook must resolve to an effective callback access mask before
   [[Runtime Lock]].
8. That callback access mask is allow/deny (whitelist/blacklist) policy resolution, not necessarily an explicit
   per-callback request list.

Open policy surface (still active):

1. Exact declaration dependency grammar (`required`/`optional`, include/exclude precedence).
2. Cycle policy in declaration dependency resolution.
3. Error taxonomy and diagnostics shape across declaration and execution seam boundaries.
4. How callback access-mask policy inputs are declared per callback kind and resolved/validated against projection
   policy.
5. How dynamic path open/close policy is versioned and audited for callback contexts.

Legacy source pointers:

-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/documents/markdown_summary/scripting_runtime_reference.md`
- `loo_cast_legacy/documents/markdown_summary/module_event_message_seams_focus_slice.md`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/catalog.rs`

Rustdoc anchors:

- `crates/core_mod/src/spec/rhai.rs`
- `crates/core_mod/src/spec/mod.rs`
