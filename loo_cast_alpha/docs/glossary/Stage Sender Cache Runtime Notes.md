# Stage Sender Cache Runtime Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
- [Stage Buffer Runtime Notes](Stage%20Buffer%20Runtime%20Notes.md)
- [Workflow Stage Type Runtime Notes](Workflow%20Stage%20Type%20Runtime%20Notes.md)
- [Workflow Runtime Structure Notes](Workflow%20Runtime%20Structure%20Notes.md)
- [Workflow Type Request and Timeout Notes](Workflow%20Type%20Request%20and%20Timeout%20Notes.md)
- [Workflow Invariant Ledger Notes](Workflow%20Invariant%20Ledger%20Notes.md)

Stage sender caches are runtime maps that resolve workflow stage metadata to concrete buffer-message sender objects.
Status note: this page is legacy mechanism signal.
The dynamic metadata-to-sender routing idea may remain useful, but the exact cache families and boxed sender surface are
not target doctrine.

## Current Cache Families

- `EcsStageSenderCache`
- `RenderStageSenderCache`
- `AsyncStageSenderCache`
- `EcsWhileStageSenderCache`
- `RenderWhileStageSenderCache`

## Current Key and Value Shape

- key: `(&'static str module_name, &'static str workflow_name, &'static str stage_name)`
- value: boxed dynamic sender trait object for that workflow domain

## Current Build Path

1. `register_workflow_mods!` generates workflow metadata with per-stage sender handles.
2. `build_stage_sender_caches()` scans metadata and populates domain caches.
3. send-to-buffer systems use caches to resolve sender by module/workflow/stage.

## Current Role

- decouple orchestration systems from direct stage type/module code paths
- provide dynamic dispatch from workflow metadata into concrete workflow-domain stage buffer channels

## Source Pointers

- `loo_cast_legacy/core_engine_macros/src/register_workflow_mods/mod.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/resources.rs`
