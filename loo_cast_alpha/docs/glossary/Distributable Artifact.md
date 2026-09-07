---
canonical_name: Distributable Artifact
status: WIP-draft
aliases:
  - Authoring Artifact
  - Workshop Artifact
  - Redistributable Artifact
---

A Distributable Artifact is the final packaged uploadable/distributable object produced by Vapor tooling.
In Phase 3, the primary public distribution target is [[Steam Workshop]].
`Authoring Artifact` is preserved as a strong alias because the object may still be author-facing even after linked
executables, shared objects, dynamic libraries, runtime libraries, payloads, manifests, dependency contents, and
publication metadata have been assembled around the build output.

Boundary:
A Distributable Artifact may reference dependencies rather than embedding every dependency literally.
Downloaded and grouped local artifact forms may contain the resolved dependency contents needed for local use.

See also:

- [[Artifact]]
- [[Source Artifact]]
- [[Build Artifact]]
- [[Steam Workshop]]

#glossary
