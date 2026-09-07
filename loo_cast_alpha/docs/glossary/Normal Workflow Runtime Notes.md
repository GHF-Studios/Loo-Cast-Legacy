# Normal Workflow Runtime Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
- [Composite Workflow Runtime Notes](Composite%20Workflow%20Runtime%20Notes.md)
- [Workflow Type Request and Timeout Notes](Workflow%20Type%20Request%20and%20Timeout%20Notes.md)
- [Workflow Runtime Structure Notes](Workflow%20Runtime%20Structure%20Notes.md)
- [Workflow Instance Runtime Notes](Workflow%20Instance%20Runtime%20Notes.md)
- [Workflow State Runtime Notes](Workflow%20State%20Runtime%20Notes.md)
- [Workflow Stage Runtime Notes](Workflow%20Stage%20Runtime%20Notes.md)
- [Workflow Invariant Ledger Notes](Workflow%20Invariant%20Ledger%20Notes.md)

In this note cluster, a normal workflow is a typed workflow request handled by the workflow runtime
(`run_workflow_*` families), not a composite wrapper.
Status note: this distinction remains useful target vocabulary, while the exact legacy request/runtime implementation
is evidence rather than final doctrine.

## Current Shape

1. Caller sends typed request via `run_workflow_*` helper.
2. The request relay system inserts a `WorkflowInstance` into the `WorkflowMap`.
3. Request/initialization systems route the first stage into a workflow-domain stage buffer.
4. Stage poll systems progress stage lifecycle.
5. Completion/failure handlers return typed response envelope to caller callback.

## Current Run Identity

- Keyed by `(module_name, workflow_name, request_id)` for response matching.
- Active-run gating currently enforces a single active workflow instance per `(module_name, workflow_name)` in
  the `WorkflowMap`.
- This active-run key gate is a coarse concurrency bottleneck and should be treated as refactor pressure, not the final
  concurrency policy.

## Relation To Composite Workflows

- Composite workflows can invoke one or multiple normal workflows.
- Workflow runs are the execution units; composite workflows are orchestration wrappers.

## Source Pointers

- `loo_cast_legacy/core_mod_api/src/backend/workflow/functions.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/request.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/response.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/resources.rs`
