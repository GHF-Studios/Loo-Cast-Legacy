# Workflow Type Request and Timeout Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
- [Workflow Runtime Structure Notes](Workflow%20Runtime%20Structure%20Notes.md)
- [Composite Workflow Runtime Notes](Composite%20Workflow%20Runtime%20Notes.md)
- [Normal Workflow Runtime Notes](Normal%20Workflow%20Runtime%20Notes.md)
- [Workflow Instance Runtime Notes](Workflow%20Instance%20Runtime%20Notes.md)
- [Workflow State Runtime Notes](Workflow%20State%20Runtime%20Notes.md)
- [Workflow Stage Runtime Notes](Workflow%20Stage%20Runtime%20Notes.md)
- [Stage Sender Cache Runtime Notes](Stage%20Sender%20Cache%20Runtime%20Notes.md)
- [Workflow Invariant Ledger Notes](Workflow%20Invariant%20Ledger%20Notes.md)

This note documents the current typed workflow request/response and timeout-control behavior.
Status note: this is legacy implementation signal.
The typed request/response split and controlled timeout path are useful evidence; the exact helper names and default
panic behavior are not target doctrine.

## Typed Workflow Request vs Composite Workflow (Current)

- In the current runtime, a normal workflow is a typed workflow request handled by `run_workflow_*` and the stage
  runtime.
- Composite workflows are orchestration wrappers that can invoke one or more normal workflows plus local orchestration
  logic.

## Current Workflow Type Surface

The runtime uses signature families for typed workflow contracts:

- `WorkflowType` with a `None` signature
- `WorkflowTypeE`
- `WorkflowTypeO`
- `WorkflowTypeOE`
- `WorkflowTypeI`
- `WorkflowTypeIE`
- `WorkflowTypeIO`
- `WorkflowTypeIOE`

Each family maps to typed request/response envelopes and per-signature channels.

## Current Request/Response Path

1. `run_workflow_*` creates a `request_id` and sends a typed request envelope.
2. `workflow_request_*_relay_system` converts requests into `WorkflowInstance` values in the `WorkflowMap`.
3. On completion/failure, callback wiring emits typed response envelopes back to request callers.
4. If unrelated responses arrive first, they are parked in `RESPONSE_INBOX` until matched by `WorkflowID`.

## Current Timeout Behavior

Default timeout behavior and controlled timeout behavior are materially different:

- Base `run_workflow_*` helpers:
    - use `RealTime` or `VirtualTime` timeout mode
    - timeout publishes a workflow-timeout signal and then panics
- Controlled path `run_workflow_ioe_with_timeout_control`:
    - emits timeout signal with incrementing `timeout_count`
    - delegates decision to caller callback (`Retry`, `Abort`, `Panic`)
    - can return the control error `TimeoutAborted` instead of panicking

## Current Concurrency Gate Shape

`workflow_request_*_relay_system` enforces a single active workflow instance per `(module_name, workflow_name)` key in
the `WorkflowMap`.
If a workflow instance is already active for that key, insertion retries on later frames.
This is a coarse active-run key gate and should be treated as a concurrency bottleneck, not settled admission policy.

## Source Pointers

- `loo_cast_legacy/core_mod_api/src/backend/workflow/traits.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/request.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/response.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/functions.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
