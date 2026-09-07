---
canonical_name: Callback Signature
status: WIP-draft
aliases:
  - callback signature
  - Capability Callback Signature
---

A Callback Signature is the metadata/identity/fingerprint-like description that identifies a callback context and
[[Capability Callback]] invocation shape without requiring the fully resolved capability graph.

Current owner-answer-informed framing:

- It should be able to identify the signature of a callback's [[Callback Context Type]].
- It is metadata-like: closer to IDs, links, and fingerprints than to executable callback logic.
- It helps compare, validate, and diagnose callback compatibility before or during graph resolution.
- It is the callback-side member of the broader signature family that also includes [[Capability Type Signature]],
  [[Capability Trait Signature]], and [[Capability Instance Signature]].

Boundary:
Callback Signature is one of the active callback-side terms.
The exact fingerprint/ID representation is not locked.

See also:

- [[Callback Type]]
- [[Callback Context Type]]
- [[Capability Callback]]
- [[Capability Type Signature]]
- [[Capability Trait Signature]]
- [[Capability Instance Signature]]
- [[Fingerprint]]
- [[Capability Declaration]]

#glossary
