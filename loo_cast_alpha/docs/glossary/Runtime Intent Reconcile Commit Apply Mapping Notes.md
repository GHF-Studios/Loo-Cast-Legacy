# Runtime Intent Reconcile Commit Apply Mapping Notes

#tech_glossary

Related glossary terms:

- [USF Runtime Evolution Lifecycle](USF%20Runtime%20Evolution%20Lifecycle.md)
- [USF Runtime Evolution Lifecycle Notes](USF%20Runtime%20Evolution%20Lifecycle%20Notes.md)
- [Capability Runtime](Capability%20Runtime.md)
- [Runtime Lock](Runtime%20Lock.md)
- [Project Runtime Representation](Project%20Runtime%20Representation.md)
- [Workflow Framework](Workflow%20Framework.md)
- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)
- [Rhai Value Semantics and AccessCell Notes](Rhai%20Value%20Semantics%20and%20AccessCell%20Notes.md)
- [Rhai Reflection Macro Surface Notes](Rhai%20Reflection%20Macro%20Surface%20Notes.md)

Current working target pressure (diagram-informed):

1. Definition and freeze complete before runtime progression.
2. Runtime tick path is intent-centric:
   emit -> route -> batch -> reconcile -> evaluate -> commit/apply.
3. Rejection path is panic-fast unless an explicit sentinel branch is intentionally defined.

Primary historical signals:

- `loo_cast_legacy/documents/intention_records/scripting_records/41_lifecycle_intent_to_commit.puml`
- `loo_cast_legacy/documents/intention_records/scripting_records/32_flow_runtime_tick_orchestration.puml`
- `loo_cast_legacy/documents/intention_records/scripting_records/11_context_authority_boundaries.puml`

Observed implementation slices (confidence-graded):

1. `loo_cast_alpha` (current workspace default):
   currently doc/spec-anchor first.
   Runtime behavior is mostly represented as anchor types under `crates/core_mod/src/spec/*`, not an implemented
   runtime pipeline.
2. `loo_cast_legacy/core_mod_api` (implementation signal, not canonical target by itself):
   has explicit phase ordering and orchestration sets (`InputGather -> Intent -> Simulation -> BoundaryResolve ->
   ChunkOrchestration -> ...`) and chunk batch lifecycle systems.
   It also has workflow-stage relay/runtime machinery and request retry queues.
   This is meaningful execution signal, but it does not yet constitute a fully explicit project-wide
   intent/reconcile/commit/apply contract.
3. Debug observability in legacy:
   debug tabs for `IntentBuffer` and `IntentCommit` exist as placeholders (`todo`) rather than finalized runtime
   observability surfaces.

Rhai reflection/registration signals (legacy, active paths):

1. A custom inventory-driven runtime binding graph is built from reflection metadata entries and hard-fails on
   duplicate/missing critical pairs.
2. Engine bootstrap registers top-level modules from that graph.
3. Script preprocess enforces strict alias/use-path checks against known global symbols and reserved keywords.
4. The MVP USF script bootstrap already enforces profile-like allowed `use` roots and panic-fast violations.
5. Proc-macro surfaces (`core_engine_macros`) generate reflection metadata for modules/types/functions/traits/generics.

Key source pointers:

- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/meta/registry.rs`
- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/bind/engine_ext.rs`
- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/engine/preprocess.rs`
- `loo_cast_legacy/core_mod_api/src/backend/usf/script_mvp.rs`
- `loo_cast_legacy/core_engine_macros/src/rhai_binding/reflection/mod.rs`

Important divergence to account for:

1. `temp_stuff/TMP_rhai_semantic_reset_quarantine` contains high-signal policy direction
   (dispatch catalogs, AccessCell posture, bridge playbooks), but `temp_stuff` is explicitly non-canonical.
2. Some quarantine paths referenced in those notes are not present in the active legacy tree and must be treated as
   target-direction signal, not current-state fact.
3. Legacy `script_mvp` currently references script asset paths under `core_mod/assets/{scale,metric,phenomenon,...}`
   that are not present in the current legacy asset tree snapshot.
   Treat profile/preprocess semantics there as framework signal, not as guaranteed up-to-date wiring.

Practical interpretation for current glossary work:

1. Keep intent/reconcile/commit/apply as canonical lifecycle intent.
2. Keep lock-time ownership resolution semantically separate from runtime callback/API gating.
3. Treat legacy runtime code as implementation evidence and mismatch detector, not source-of-truth.
4. Promote only durable, reconciled outcomes from legacy/temp signals into glossary terms.
5. Keep reflection macro surface as a high-signal peer signal (alongside other working frameworks such as workflow);
   runtime intent pipeline docs should compose with it, not overshadow it.
6. Treat capabilities as intent emitters/request relays, not as the owner of canonical state progression.
7. Keep reconcile/commit/apply outside the capability graph unless explicitly reopened.
