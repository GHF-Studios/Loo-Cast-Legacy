# Workflow Runtime Structure Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
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
- [Runtime Substrate](Runtime%20Substrate.md)

This note documents current legacy workflow-runtime structure and behavior.
It is documentation-first, not a redesign proposal.
Treat it as legacy implementation signal: useful mechanics and vocabulary may survive, but the exact old macro/runtime
surface is not target doctrine.

## Runtime Layer Map (Current)

1. Macro declaration layer via `define_workflow_mod_OLD`:
    - Generates stage modules, stage poll systems, and typed workflow entry functions.
2. Workflow-module registration layer via `register_workflow_mods!`:
    - Builds workflow metadata and stage-sender cache wiring.
3. Workflow runtime plugin layer via `WorkflowPlugin`:
    - Owns channels, buffers, request/response resources, and orchestration systems.
4. Call-site orchestration layer:
    - Uses `composite_workflow!` + `run_workflow_*` helpers.

## Request -> Execution -> Completion Flow (Current)

1. A caller invokes `run_workflow_*` (or `run_workflow_*_with_timeout_control`).
2. A typed request is sent to a per-signature request channel.
3. `workflow_request_*_relay_system` inserts a `WorkflowInstance` into the `WorkflowMap`.
4. `workflow_request_system` transitions the workflow state from `Requested` to `Processing`, sets stage 0, and emits
   `StageInitializationMessage`.
5. `workflow_initialization_system` moves the stage object out of the registry slot (temporary placeholder swap), then
   pushes the real stage into a workflow-domain stage buffer.
6. Domain receive systems move stage messages into local stage buffers.
7. Domain poll systems execute stage setup/run logic and emit setup/wait/completion/failure messages.
8. Wait/completion/failure relay/handling systems update workflow state, move to next stage, or finalize
   callback/response.

## Domain/Schedule Split (Current)

- `Ecs` / `EcsWhile` / `Async` pollers run in main app `Update`.
- `Render` / `RenderWhile` pollers run in RenderApp `Render` schedule.
- `Render` workflow-domain stage buffers are extracted from `MainWorld` into render-world resources before render
  polling.

## Legacy Usage Location

Canonical usage examples are maintained in
[Workflow Usage Patterns Legacy Notes](Workflow%20Usage%20Patterns%20Legacy%20Notes.md).

## Source Pointers

- `loo_cast_legacy/core_mod_api/src/backend/workflow/mod.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/functions.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/stage.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/mod.rs`
- `loo_cast_legacy/core_engine_macros/src/register_workflow_mods/mod.rs`
- `loo_cast_legacy/core_mod_api/src/backend/core/systems.rs`
- `loo_cast_legacy/core_mod_api/src/backend/chunk/systems.rs`
- `loo_cast_legacy/base_mod_api/src/backend/gpu/workflows.rs`
