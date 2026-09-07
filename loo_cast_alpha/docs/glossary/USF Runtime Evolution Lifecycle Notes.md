# USF Runtime Evolution Lifecycle Notes

#tech_glossary

Related glossary terms:

- [USF Runtime Evolution Lifecycle](USF%20Runtime%20Evolution%20Lifecycle.md)
- [USF Definition Lifecycle](USF%20Definition%20Lifecycle.md)
- [Capability Declaration](Capability%20Declaration.md)
- [USF Runtime](USF%20Runtime.md)
- [Runtime Lock](Runtime%20Lock.md)
- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)
- [Runtime Intent Reconcile Commit Apply Mapping Notes](Runtime%20Intent%20Reconcile%20Commit%20Apply%20Mapping%20Notes.md)

Current execution-shape notes:

1. Definition side transitions through Runtime Lock before runtime progression.
2. Runtime flow is still framed as `intent -> reconcile -> commit -> apply`.
3. Runtime operates on active [[Capability Instance]]s materialized from capabilities established at Runtime Lock.
4. These instances invoke sanctioned callback closures through type/trait/callback-tailored `ctx` capability-object
   subgraphs.
5. `ctx` capability-object subgraphs are hierarchical (atomic + composite API graph nodes), filtered by include/exclude
   path declarations, and may
   dynamically open/close by runtime policy.
6. Callback invocation paths enforce resolved effective callback `ctx` path masks from allow/deny policy outcomes.
7. Lifecycle semantics remain intentionally high-level, but still constrained by Runtime Lock boundaries.

Current implementation reality check:

1. `loo_cast_alpha` is still spec-anchor first (`crates/core_mod/src/spec/*`), not a full runtime pipeline
   implementation.
2. The strongest executable signal for phased runtime behavior currently sits in legacy `core_mod_api`
   orchestration/workflow/chunk systems.
3. Intent/reconcile/commit/apply is therefore canonical as target semantics, with legacy code used as
   implementation evidence and mismatch detector.

Cross-module seam hints carried from legacy slice docs:

- `player`: emits anchor updates
- `chunk`: owns boundary/topology planning and emits lifecycle deltas
- `usf`: consumes intents and emits realization/runtime facts
- `rhai_binding`: script entrypoint and binding authority
- `core`/`render`/`debug`: orchestration, output, and observability surfaces

Legacy source pointers:

- `loo_cast_legacy/documents/markdown_summary/module_event_message_seams_focus_slice.md`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_plan_entrypoint_first_runtime.md`

Rustdoc anchors:

- `crates/core_mod/src/spec/mod.rs`
