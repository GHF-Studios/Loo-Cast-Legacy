---
canonical_name: Artifact Compatibility Envelope
status: WIP-draft
aliases: [ ]
---

The Artifact Compatibility Envelope defines when development artifacts are reusable and when rebuilds are required.
Canonical sharable artifacts are source-first, especially the [[Mod Contract Source]],
the [[Mod Implementation Source]], and the [[Redistributable Mod Contract Source]], under SDK and toolchain
compatibility constraints.
Cached build artifacts of the [[Redistributable Mod Implementation Library]] may be reused per supported target
compatibility bucket to reduce unnecessary full dependency-graph recompiles in normal SDK-stable workflows.
Source archival remains mandatory so dynamic and static artifacts can always be rebuilt without relying on external
second-party repository availability.

#glossary
