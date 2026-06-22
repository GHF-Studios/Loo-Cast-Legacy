# Workflow Invariant Ledger Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
- [Workflow Runtime Structure Notes](Workflow%20Runtime%20Structure%20Notes.md)
- [Workflow Type Request and Timeout Notes](Workflow%20Type%20Request%20and%20Timeout%20Notes.md)
- [Workflow Stage Runtime Notes](Workflow%20Stage%20Runtime%20Notes.md)
- [Stage Buffer Runtime Notes](Stage%20Buffer%20Runtime%20Notes.md)
- [Stage Sender Cache Runtime Notes](Stage%20Sender%20Cache%20Runtime%20Notes.md)
- [Workflow Instance Runtime Notes](Workflow%20Instance%20Runtime%20Notes.md)
- [Workflow State Runtime Notes](Workflow%20State%20Runtime%20Notes.md)

This ledger captures current behavior contracts that are easy to forget during refactor.
Status note: these are legacy implementation invariants.
Some should be preserved as semantics, while others are hazards or bottlenecks that future rewrites should eliminate or
make mechanically safe.

## Invariant 1: Stage Slot Placeholder Lifecycle

Current contract:

1. Active stage slot is replaced with placeholder while real stage object is in-flight.
2. Completion/failure handlers restore real stage object into its registry slot.
3. Slot must be re-filled before that stage slot is needed again.
4. Treat this as a refactor-sensitive invariant, not a desired public API shape.

## Invariant 2: Output->Input `transmute` Authoring Contract

Current contract:

1. Stage output/input boundary may use `unsafe transmute` in generated handlers.
2. Authoring must keep stage `N` `Output` and stage `N+1` `Input` representation-compatible.
3. Compatibility is currently a maintained convention, not runtime-verified proof.
4. Treat this as a legacy hazard; future wiring should encode or verify compatibility.

## Invariant 3: Active-Run Key Gate

Current contract:

1. The `WorkflowMap` holds at most one active workflow instance per `(module_name, workflow_name)`.
2. New requests for an already-active key retry on later frames.
3. Treat this as a coarse concurrency bottleneck, not settled admission policy.

## Invariant 4: Single-Item Poll Progress

Current contract:

1. Domain poll systems pop one stage-buffer entry per run.
2. Under high ingress, buffers may backlog and drain over later ticks.

## Invariant 5: RenderWhile Shard Intention

Current contract:

1. RenderWhile split/fuse sharding exists to reduce shared-state contention and preserve parallelism opportunities.
2. This does not guarantee full realized parallelism on its own; other orchestration bottlenecks still apply.
3. Treat it as a partial parallelism-preserving mechanism.

## Invariant 6: Signature Family Routing

Current contract:

1. Request/response typing is split by workflow signature families (`None`, `E`, `O`, `OE`, `I`, `IE`, `IO`, `IOE`).
2. Correct callback/result unpacking depends on signature-appropriate envelope routing.

## Source Pointers

- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/instance.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/resources.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/functions.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/stage.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/core_function.rs`
