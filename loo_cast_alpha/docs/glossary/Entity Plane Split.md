---
canonical_name: Entity Plane Split
status: WIP-draft
aliases: [ ]
---

Entity Plane Split is a modeling pattern where one conceptual entity is represented across backend and frontend planes.
The backend plane owns interaction/state authority while the frontend plane owns visible presentation authority.
This split is intended to support scale-relative visibility, projection tricks, and portal/wrapping mechanics without
collapsing semantic authority boundaries.
It also gives the runtime room to use traditional floating-point local subsystems, such as physics or rendering
libraries, behind a scale-aware semantic authority boundary.

See also:

- [[Entity Proxy]]
- [[Observer-Relative Simulation]]
- [[Portal Traversal Semantics]]

#glossary
