---
canonical_name: Spacetime Engine
status: draft
aliases: []
---

The Spacetime Engine is the first-party [[Engine]] product that provides the [[Runtime Substrate]] for default and
heavily modified experiences.
It hosts composable system behavior through the [[Capability Runtime]] within the boundaries of the [[Contract]].

In the broader [[Vapor Ecosystem]] framing, the Spacetime Engine is an official engine/framework product rather than the
only possible engine product.
Other engines may exist inside Vapor-level conventions, even if they do not preserve Spacetime-specific systems or
expose the same modding affordances.

Current owner-answer-informed caveat:
The [[USF]] is a pivotal public/API-facing module/framework part of the Spacetime Engine.
It is not currently framed as a standalone product or a directly/exclusively replaceable Vapor-level [[Capability]].
USF is nevertheless public and API-facing; it is most of the practical Spacetime-level world/model API used by
[[Loo Cast]].

Current implementation framing:
`core_engine` and `core_mod` together define the concrete Engine being used.
`core_mod` is hard-required for its corresponding `core_engine` and should not be treated as freely removable or
mix-and-match replaceable.
The `base_mod`/Game layer depends on the `core_mod` surface and on whatever it exposes or re-exports from
`core_engine` and Vapor.

#glossary
