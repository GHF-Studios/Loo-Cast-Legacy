# Workflow Framework Premise Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Capability Declaration](Capability%20Declaration.md)
- [Rhai Capability](Rhai%20Capability.md)
- [Runtime Substrate](Runtime%20Substrate.md)
- [Modding Runtime](Modding%20Runtime.md)
- [Capability Runtime](Capability%20Runtime.md)
- [USF Runtime](USF%20Runtime.md)
- [Workflow Runtime Structure Notes](Workflow%20Runtime%20Structure%20Notes.md)
- [Composite Workflow Runtime Notes](Composite%20Workflow%20Runtime%20Notes.md)
- [Normal Workflow Runtime Notes](Normal%20Workflow%20Runtime%20Notes.md)
- [Workflow Type Request and Timeout Notes](Workflow%20Type%20Request%20and%20Timeout%20Notes.md)
- [Workflow Instance Runtime Notes](Workflow%20Instance%20Runtime%20Notes.md)
- [Workflow State Runtime Notes](Workflow%20State%20Runtime%20Notes.md)
- [Workflow Stage Runtime Notes](Workflow%20Stage%20Runtime%20Notes.md)
- [Workflow Stage Type Runtime Notes](Workflow%20Stage%20Type%20Runtime%20Notes.md)
- [Stage Buffer Runtime Notes](Stage%20Buffer%20Runtime%20Notes.md)
- [Stage Sender Cache Runtime Notes](Stage%20Sender%20Cache%20Runtime%20Notes.md)
- [Workflow Invariant Ledger Notes](Workflow%20Invariant%20Ledger%20Notes.md)
- [Workflow Usage Patterns Legacy Notes](Workflow%20Usage%20Patterns%20Legacy%20Notes.md)
- [Rhai Reflection Macro Surface Notes](Rhai%20Reflection%20Macro%20Surface%20Notes.md)

Status note:
Current working decision is that [[Workflow Framework]] remains a future-target Rust orchestration concept, while the
detailed runtime pages are legacy implementation signals.
Preserve the target pressure around Rust-side lifecycle orchestration, Bevy-visible typed stage systems, domain
separation, and normal/composite distinction; treat known bottlenecks and unsafe conventions as refactor debt.

Current premise slice (draft, intentionally not frozen):

1. Workflow framework scope is Rust-side orchestration semantics.
2. Stage execution should stay scheduler-visible as normal Bevy systems with typed `SystemParam` access.
3. Control-plane lifecycle handling and execution-plane stage logic are separate concerns.
4. `ECS`, `Render`, and `Async` are first-class distinct workflow domains.
5. `EcsWhile` and `RenderWhile` are core iterative variants for non-async contexts.
6. Workflow stages should orchestrate materialized Rust-side capability/runtime values, not raw Rhai engine internals.
7. If declaration/materialization progression is orchestrated by workflow, it should be mediated through explicit
   Rust-side capability contracts rather than ad hoc script engine calls.

Concurrent-domain reminder (Rust is the host boundary):

1. Capability declaration/materialization domain:
   USF + Rhai declaration semantics, profile gating, and capability declaration output shape.
2. Rust-native runtime orchestration domain:
   workflow framework mechanics, scheduling/lifecycle orchestration, and runtime execution management.
3. These domains share Rust as a compatibility boundary but keep distinct semantic contracts.

Terminology baseline used in this note cluster:

1. A workflow run is one runtime lifecycle unit for a typed workflow request.
2. `WorkflowInstance` is the runtime container for a workflow run.
3. `WorkflowState` is the lifecycle state machine for a workflow run.
4. `StageType` is the workflow-domain discriminator for the active stage in a workflow run.

Why this premise is currently high-signal:

1. Generated stage poll systems are normal Bevy systems (typed params), so scheduler conflict analysis and parallelism
   are preserved.
2. A significant part of current orchestration still uses exclusive `&mut World` systems, which is refactor debt rather
   than a useful premise.
3. The framework already separates domain stage families (`Ecs`/`Render`/`Async` + while variants), matching the
   intended mental model.
4. This is one high-signal working framework surface among multiple pillars (for example, Rhai reflection macros),
   not a hierarchy apex.

Current behavior coverage notes (documentation-first slice):

1. [Workflow Runtime Structure Notes](Workflow%20Runtime%20Structure%20Notes.md)
2. [Composite Workflow Runtime Notes](Composite%20Workflow%20Runtime%20Notes.md)
3. [Normal Workflow Runtime Notes](Normal%20Workflow%20Runtime%20Notes.md)
4. [Workflow Type Request and Timeout Notes](Workflow%20Type%20Request%20and%20Timeout%20Notes.md)
5. [Workflow Instance Runtime Notes](Workflow%20Instance%20Runtime%20Notes.md)
6. [Workflow State Runtime Notes](Workflow%20State%20Runtime%20Notes.md)
7. [Workflow Stage Runtime Notes](Workflow%20Stage%20Runtime%20Notes.md)
8. [Workflow Stage Type Runtime Notes](Workflow%20Stage%20Type%20Runtime%20Notes.md)
9. [Stage Buffer Runtime Notes](Stage%20Buffer%20Runtime%20Notes.md)
10. [Stage Sender Cache Runtime Notes](Stage%20Sender%20Cache%20Runtime%20Notes.md)
11. [Workflow Invariant Ledger Notes](Workflow%20Invariant%20Ledger%20Notes.md)
12. [Workflow Usage Patterns Legacy Notes](Workflow%20Usage%20Patterns%20Legacy%20Notes.md)

Near-term direction (still draft):

1. Keep stage execution in typed systems (do not collapse into exclusive global dispatch).
2. Incrementally move orchestration/control systems away from exclusive `&mut World` patterns where feasible.
3. Preserve first-class domain distinctions and while-stage semantics through refactor.
4. Refine and pressure-test run-model semantics (identity keys, concurrency policy, cancellation behavior).

Run identity + concurrency draft v0.1 (still draft, not frozen):

1. Workflow run identity is Rust-side only and must not be keyed by raw Rhai engine handles.
2. Each run has a stable `run_id` plus a semantic `concurrency_key`.
3. `concurrency_key` is derived from workflow kind + workflow domain (`ECS`/`Render`/`Async`) + target scope key.
4. Target scope keys point to runtime-owned targets/materialized artifacts, not declaration script file identities.
5. `EcsWhile`/`RenderWhile` iterations keep one `run_id`; iterations are lifecycle transitions, not new runs.
6. Admission policy: conflicting `concurrency_key` values serialize; non-conflicting keys can run in parallel.
7. Conflict handling policy can choose `reject`, `queue`, or `replace`; deterministic `queue` is the current default
   candidate.
8. Cancellation is cooperative and keyed by `run_id`; `completed`/`failed`/`cancelled` are terminal states.
9. Observability minimum per transition: `run_id`, `concurrency_key`, workflow domain, lifecycle transition, outcome.
10. If workflow orchestrates declaration/materialization progression, it should do so via explicit Rust capability
    contracts, not ad hoc script-engine calls.

Legacy source pointers:

- `loo_cast_legacy/core_mod_api/src/backend/workflow/mod.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/mod.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/core_function.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/core_type.rs`
