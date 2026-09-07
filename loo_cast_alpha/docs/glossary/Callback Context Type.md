---
canonical_name: Callback Context Type
status: WIP-draft
aliases:
  - callback ctx type
  - callback context
---

A Callback Context Type is the dedicated `ctx` type projected into one [[Callback Type]] /
[[Capability Callback]].
It describes the callback's scoped view of capabilities, data, inputs, and allowed operations.

Current owner-answer-informed framing:

- Every meaningful Callback Type should have a custom dedicated Callback Context Type.
- The callback context should be projected, not raw unrestricted graph access.
- The context type is distinct from the callback function shape and from the callback's stable signature identity.
- Trait callbacks should receive context types shaped by the declaring [[Capability Trait]], implementing
  [[Capability Type]], and resolved projection policy.

Boundary:
Callback Context Type is one of the active callback-side terms.
It should be linked to [[Capability Projection API]] and scope-envelope rules when those rules matter.

See also:

- [[Callback Type]]
- [[Callback Signature]]
- [[Capability Callback]]
- [[Capability Projection API]]
- [[Callback Scope Envelope]]
- [[Script Safety]]

#glossary
