# Workflow State Runtime Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
- [Workflow Instance Runtime Notes](Workflow%20Instance%20Runtime%20Notes.md)
- [Workflow Stage Type Runtime Notes](Workflow%20Stage%20Type%20Runtime%20Notes.md)
- [Workflow Stage Runtime Notes](Workflow%20Stage%20Runtime%20Notes.md)
- [Workflow Invariant Ledger Notes](Workflow%20Invariant%20Ledger%20Notes.md)

`WorkflowState` is the per-workflow-instance lifecycle state machine.
Status note: this is legacy implementation signal.
The requested/processing lifecycle shape is useful evidence, but the exact flag structure is not target doctrine.

## Current State Variants

1. `Requested`
2. `Processing { current_stage, current_stage_type, stage_initialized, stage_completed }`

## Current Transition Shape

1. The request relay inserts a workflow instance as `Requested`.
2. Request system transitions to `Processing` at stage 0.
3. Setup/wait/completion systems toggle `stage_initialized`/`stage_completed` as stage lifecycle advances.
4. Final completion/failure removes the workflow instance from the `WorkflowMap`.

## Current Meaning of Flags

- `stage_initialized`:
    - current stage setup path has run (or stage has reached initialized state).
- `stage_completed`:
    - current stage terminal completion was observed by completion handling.

## Source Pointers

- `loo_cast_legacy/core_mod_api/src/backend/workflow/types.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/instance.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
