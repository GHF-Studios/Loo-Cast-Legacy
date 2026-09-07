# Workflow Instance Runtime Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
- [Workflow Runtime Structure Notes](Workflow%20Runtime%20Structure%20Notes.md)
- [Normal Workflow Runtime Notes](Normal%20Workflow%20Runtime%20Notes.md)
- [Workflow State Runtime Notes](Workflow%20State%20Runtime%20Notes.md)
- [Workflow Type Request and Timeout Notes](Workflow%20Type%20Request%20and%20Timeout%20Notes.md)
- [Workflow Invariant Ledger Notes](Workflow%20Invariant%20Ledger%20Notes.md)

`WorkflowInstance` is the runtime-owned container for an active typed workflow request.
Status note: this is legacy implementation signal.
The run-container idea may survive, but placeholder replacement patterns should be treated as refactor-sensitive
invariants.

## Current Variant Families

- `None`
- `E`
- `O`
- `OE`
- `I`
- `IE`
- `IO`
- `IOE`

Each variant stores module/workflow identity, `request_id`, `WorkflowState`, a callback, stage count, timeout frames,
and, where needed, a data buffer.

## Current Responsibilities

1. Carry run-scoped state across stage progression.
2. Hold callback closure until terminal completion/failure handling.
3. Hold data buffer for signatures that need input/output transport.
4. Track workflow state transitions (`Requested` -> `Processing` lifecycle).

## Current Placeholder Behavior

- `take_data_buffer` and `take_callback` use placeholder replacement patterns while values are in-flight.
- Correctness depends on handlers restoring valid values before later access paths require them.
- Future rewrites should prefer state shapes that make invalid placeholder access unrepresentable where practical.

## Source Pointers

- `loo_cast_legacy/core_mod_api/src/backend/workflow/instance.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/resources.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
