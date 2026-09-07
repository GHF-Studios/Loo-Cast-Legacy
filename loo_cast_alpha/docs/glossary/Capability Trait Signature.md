---
canonical_name: Capability Trait Signature
status: WIP-draft
aliases: []
---

A Capability Trait Signature is the metadata/fingerprint-like description of a [[Capability Trait]].
It identifies the trait surface that a [[Capability Type]] can implement and that a [[Capability Extension Slot]] can
require as a bound.

Likely contents:

- stable trait identity
- version or fingerprint material
- required trait callbacks
- required metadata/projection surfaces
- compatibility constraints for implementers
- diagnostics material for missing or incompatible implementations

Boundary:
A Capability Trait Signature is not executable behavior.
It describes the trait's shape so Vapor tooling can compare declarations, validate extension slots, generate lockfile
state, and explain compatibility failures.

See also:

- [[Capability Trait]]
- [[Capability Type Signature]]
- [[Capability Extension Slot]]
- [[Callback Signature]]

#glossary
