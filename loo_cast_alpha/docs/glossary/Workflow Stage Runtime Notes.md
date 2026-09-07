# Workflow Stage Runtime Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
- [Workflow Runtime Structure Notes](Workflow%20Runtime%20Structure%20Notes.md)
- [Composite Workflow Runtime Notes](Composite%20Workflow%20Runtime%20Notes.md)
- [Workflow Type Request and Timeout Notes](Workflow%20Type%20Request%20and%20Timeout%20Notes.md)
- [Workflow Stage Type Runtime Notes](Workflow%20Stage%20Type%20Runtime%20Notes.md)
- [Stage Buffer Runtime Notes](Stage%20Buffer%20Runtime%20Notes.md)
- [Stage Sender Cache Runtime Notes](Stage%20Sender%20Cache%20Runtime%20Notes.md)
- [Workflow Invariant Ledger Notes](Workflow%20Invariant%20Ledger%20Notes.md)

This note documents current workflow-domain stage behavior and stage-lifecycle mechanics.
Status note: this is legacy implementation signal.
The useful target pressure is scheduler-visible stage execution across workflow domains; placeholder slots,
`unsafe transmute` handoff, single-item polling, and coarse run gates are hazards or bottlenecks unless a future design
re-justifies them.

## Stage Families (Current)

- `Ecs`
- `Render`
- `Async`
- `EcsWhile`
- `RenderWhile`

`EcsWhile` and `RenderWhile` use setup+run semantics and emit `Wait` or `Done` outcomes.

`StageType` is the runtime enum discriminator for these families and is used in workflow state tracking and message
routing.

## Stage Object Lifecycle and Placeholder Invariant

Current behavior in `workflow_initialization_system` and `workflow_completion_handling_system`:

1. Active stage object is moved out of registry slot via `std::mem::replace(..., placeholder)`.
2. The real stage object is pushed into stage buffers and eventually returned through completion/failure messages.
3. Completion/failure handlers write the returned stage object back into its registry slot.
4. For intermediate-stage completion, next stage is moved out the same way (placeholder swap), then queued.

Documented invariant in current model:

- Placeholder exists only while a real stage object is in-flight.
- Stage slots are expected to be re-filled by returned real stage objects before the slot is needed again.
- This is a refactor-sensitive invariant, not an ideal API shape; future rewrites should prevent placeholder access by
  construction where possible.

## Output->Input Handoff via `unsafe transmute` (Legacy Hazard)

Macro-generated stage response handlers currently use `unsafe { std::mem::transmute(output) }` for stage-to-stage
output/input handoff.

Current intended soundness contract in this codebase:

1. Stage `N` `Output` and stage `N+1` `Input` are intentionally authored as representation-compatible shapes.
2. Field order/types/validity semantics are preserved across that boundary.
3. This invariant is maintained by workflow authoring patterns (often wrapper structs such as `inner` forwarding).

Current limitation:

- The macro wiring does not mechanically prove representation compatibility; the contract is author-maintained.
- This is legacy hazard evidence. A future implementation should encode or prove stage-boundary compatibility rather
  than relying on author-maintained layout assumptions.

## Throughput Behavior (Current)

Each workflow-domain stage poll system currently processes one buffered stage item per system run:

- pop one entry from `StageBuffer`
- run setup/run handler
- return

Current effect:

- deterministic single-item progress per stage-poll system tick
- backlog can accumulate in buffers under heavy ingress
- while-stages repeatedly requeue through wait-handling and consume frame slices over time

## RenderWhile State Extract Behavior (Current)

`RenderWhile` uses a split/fuse shard path:

1. split main `RenderWhileWorkflowStateExtract` entry into a local shard before polling
2. poll render-while stage buffer using shard visibility
3. fuse shard back into extracted state resource after polling

Legacy design rationale (documented intent):

- avoid holding/mutating the full extracted state map during per-stage render-while polling
- keep stage-local iteration state isolated while the stage poll system runs
- reduce contention on shared extract state so render-side scheduling has a better chance to stay parallel-friendly

Current caveat:

- this sharding is a partial parallelism-preserving mechanism, not full parallelism by itself
- overall realized parallelism is still bounded by other workflow runtime bottlenecks documented in this note

## Source Pointers

- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/stage.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/stage.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/core_function.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/core_type.rs`
