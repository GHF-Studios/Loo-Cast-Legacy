---
canonical_name: Runtime Lock
status: WIP-draft
aliases: []
---

The Runtime Lock is the boundary where validated composition becomes immutable runtime state.
In the first-party stack, this boundary is established before `core_engine` enters locked runtime execution.
By this boundary, ownership resolution inside the [[Capability Graph Scope Envelope]] must already be finalized.
At this boundary, callback access policy inputs must already be resolved into effective callback `ctx` path masks for
runtime invocation.
Bootstrap fixed-point resolution must already have converged to a valid cycle-free state before lock.
After lock, the active mod graph and resolved ownership mappings are fixed, and runtime load or unload mutation is not
allowed/possible.
Deterministic behavior here is grounded in pre-runtime validation and fixed load ordering through
the [[Modding Runtime]] and the [[Slot Graph Composition]].

Current owner-answer-informed clarification:
Runtime Lock is both a finish state and something whose establishment may be treated as a lifecycle event.
The lock state is reached after staged graph construction succeeds: artifact discovery, user/modpack projection, shallow
metadata pre-validation, dependency/capability expansion, deep validation, then lock.
Post-lock graph mutation is forbidden by default.
If runtime dynamism is required, it should be modeled through explicit dynamic capability/registry policies rather than
arbitrary graph mutation.

Batch 005 follow-up clarification:
Runtime Lock currently applies to launchable Engine/Game runtime composition.
Launcher and SDK authoring should instead remain a more static-only, read-only, hardcoded capability environment for
now, not a dynamically Rhai-extended runtime composition.

Phase 3 lock-candidate anchor:
Runtime Lock Phase 3 behavior is anchored by
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W03 and the Runtime Flow.
In Phase 3, no selected Engine/Game fixture should run until the launchable composition reaches Runtime Lock.

See also:

- [[Capability Bootstrap Fixed-Point Cycle]]
- [[Asymmetric Failure Doctrine]]

#glossary
