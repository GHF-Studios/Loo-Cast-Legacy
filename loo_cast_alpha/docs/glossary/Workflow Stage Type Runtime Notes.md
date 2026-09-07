# Workflow Stage Type Runtime Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
- [Workflow Stage Runtime Notes](Workflow%20Stage%20Runtime%20Notes.md)
- [Workflow State Runtime Notes](Workflow%20State%20Runtime%20Notes.md)
- [Workflow Runtime Structure Notes](Workflow%20Runtime%20Structure%20Notes.md)
- [Stage Buffer Runtime Notes](Stage%20Buffer%20Runtime%20Notes.md)
- [Stage Sender Cache Runtime Notes](Stage%20Sender%20Cache%20Runtime%20Notes.md)
- [Workflow Invariant Ledger Notes](Workflow%20Invariant%20Ledger%20Notes.md)

`StageType` is the runtime discriminator for workflow domains at stage level.
Status note: this is legacy implementation signal.
The first-class domain distinction is useful target vocabulary, while exact enum naming remains implementation detail.

## Current StageType Values

- `Ecs`
- `Render`
- `Async`
- `EcsWhile`
- `RenderWhile`

## Current Usage

1. Stored in `WorkflowState::Processing.current_stage_type`.
2. Used by runtime orchestration to route stage objects/messages through workflow-domain buffers and poll systems.
3. Used in render-while extraction paths and diagnostics/state inspection.

## Source Pointers

- `loo_cast_legacy/core_mod_api/src/backend/workflow/stage.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/types.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/resources.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
