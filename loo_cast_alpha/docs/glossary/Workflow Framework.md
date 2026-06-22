---
canonical_name: Workflow Framework
status: WIP-draft
aliases: []
---

The Workflow Framework is the Rust-side orchestration layer for staged runtime work in the [[Runtime Substrate]].
It is responsible for lifecycle coordination (request, progression, completion/failure handling) across workflow
domains,
while stage logic itself remains scheduler-visible Bevy systems.
It orchestrates materialized Rust-side capability/runtime values and explicit Rust contract surfaces, not raw Rhai
engine internals.

It treats `ECS`, `Render`, and `Async` as first-class workflow domains.
`EcsWhile` and `RenderWhile` are core iterative stage variants for non-async domains.

Status boundary:
The Workflow Framework remains a future-target Rust orchestration concept.
The implementation-facing workflow notes are legacy evidence and refactor pressure, not a commitment to preserve the
old macro/runtime surface wholesale.

Scope boundary:

- workflow orchestration is Rust-side
- Rhai-side capability/domain exposure is handled by script-profile and binding surfaces
- stage logic should stay scheduler-visible as normal Bevy systems where practical
- legacy exclusive control-plane dispatch, placeholder swaps, unsafe handoff, active-run gates, and single-item polling
  are refactor debt unless a later design explicitly re-justifies them

Terminology used across workflow runtime notes:

- workflow run: one runtime lifecycle unit for a typed workflow request
- workflow instance: a `WorkflowInstance` for a workflow run
- workflow state: a `WorkflowState` for a workflow run
- workflow domain discriminator: `StageType`, with values `Ecs`, `Render`, `Async`, `EcsWhile`, and `RenderWhile`

Implementation-facing notes:

- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
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

Current run identity/concurrency draft is maintained in those premise notes.

#glossary
