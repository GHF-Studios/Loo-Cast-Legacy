# Stage Buffer Runtime Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
- [Workflow Stage Runtime Notes](Workflow%20Stage%20Runtime%20Notes.md)
- [Stage Sender Cache Runtime Notes](Stage%20Sender%20Cache%20Runtime%20Notes.md)
- [Workflow Runtime Structure Notes](Workflow%20Runtime%20Structure%20Notes.md)
- [Workflow Invariant Ledger Notes](Workflow%20Invariant%20Ledger%20Notes.md)

Stage buffers are per-workflow-domain queues carrying in-flight stage objects and optional stage data payloads.
Status note: this page is legacy implementation signal.
Per-domain buffering remains useful evidence; the old single-item poll behavior is a known problem, not target doctrine.

## Current Buffer Families

- `EcsStageBuffer`
- `RenderStageBuffer`
- `AsyncStageBuffer`
- `EcsWhileStageBuffer`
- `RenderWhileStageBuffer`

Each entry stores `(module_name, workflow_name, stage_index, stage_object, optional_stage_data)`.

## Current Behavior

1. Initialization/completion/wait systems push stage entries into domain buffers.
2. Domain poll systems pop entries and execute stage setup/run handlers.
3. Poll systems currently pop one entry per poll-system run (single-item progress pattern).

## Current Implications

- deterministic step-wise progression
- predictable serialization point per poll system
- backlog can accumulate under high ingress until later ticks drain it

## Known Legacy Problem

The single-item poll pattern is not just a neutral throughput detail.
Legacy testing observed backlog causing visible holes and broader system lag under load.
Any future workflow rewrite should treat buffer draining, backpressure, sharding, and scheduling pressure as first-class
runtime concerns rather than preserving this behavior by default.

## Source Pointers

- `loo_cast_legacy/core_mod_api/src/backend/workflow/resources.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/core_type.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/core_function.rs`
