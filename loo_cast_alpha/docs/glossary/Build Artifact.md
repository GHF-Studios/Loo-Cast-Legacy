---
canonical_name: Build Artifact
status: WIP-draft
aliases: []
---

A Build Artifact is built source output that is not yet assembled, packaged, or published as a final distributable.

Boundary:
Build artifacts are produced by SDK/build tooling from [[Source Artifact]]s.
They are not the same as [[Distributable Artifact]]s and are not in-memory [[Capability]] objects.
A linked executable, shared object, or dynamic library can be a build artifact before the surrounding runtime library
payloads, manifests, dependency contents, and publication metadata are assembled into a distributable form.

#glossary
