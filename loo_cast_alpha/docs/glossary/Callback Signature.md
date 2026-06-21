---
canonical_name: Callback Signature
status: WIP-draft
aliases:
  - callback signature
---

A Callback Signature is the metadata/identity/fingerprint-like description that identifies a callback context and
callback invocation shape without requiring the fully resolved capability graph.

Current owner-answer-informed framing:

- It should be able to identify the signature of a callback's [[Callback Context Type]].
- It is metadata-like: closer to IDs, links, and fingerprints than to executable callback logic.
- It helps compare, validate, and diagnose callback compatibility before or during graph resolution.

Boundary:
Callback Signature is one of the active callback-side terms.
The exact fingerprint/ID representation is not locked.

See also:

- [[Callback Type]]
- [[Callback Context Type]]
- [[Fingerprint]]
- [[Capability Declaration]]

#glossary
