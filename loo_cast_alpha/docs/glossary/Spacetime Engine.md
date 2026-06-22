---
canonical_name: Spacetime Engine
status: draft
aliases: []
---

The Spacetime Engine is the first-party [[Engine]] product that provides the [[Runtime Substrate]] for default and
heavily modified experiences.
It uses and extends Vapor-level [[Capability Runtime]] semantics within the boundaries of the [[Contract]].

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
`core_engine` and its matching `core_mod` are [[Reserved Built-In Mod Role]] names that together define the concrete
Engine being used.
`core_mod` is mandatory for a valid Vapor product instance, but it is replaceable by selecting a different valid
`core_engine`/`core_mod` pair.
It should not be treated as optional, freely removable, or mix-and-match replaceable independently of its engine pair.
The `base_mod`/Game layer depends on the `core_mod` surface and on whatever it exposes or re-exports from
`core_engine` and Vapor.

#glossary
