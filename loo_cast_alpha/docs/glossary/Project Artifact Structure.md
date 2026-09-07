---
canonical_name: Project Artifact Structure
status: WIP-draft
aliases: []
---

The Project Artifact Structure defines build and distribution artifact classes at project scope.
It distinguishes runtime-deliverable artifacts from development and dependency-channel artifacts.
Runtime-deliverable artifacts include executable/runtime bundles and mod runtime packages.
For mods, development and dependency classes center on the [[Mod Contract Source]], the [[Mod Implementation Source]],
and the [[Redistributable Mod Contract Source]], while runtime delivery centers on
the [[Redistributable Mod Implementation Library]].
Compatibility and reuse boundaries for cached build artifacts are defined by the [[Artifact Compatibility Envelope]].
This note describes artifact classes only and does not define runtime composition or in-memory integration behavior.

#glossary
