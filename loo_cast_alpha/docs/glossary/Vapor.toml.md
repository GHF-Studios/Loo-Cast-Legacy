---
canonical_name: Vapor.toml
status: WIP-draft
aliases:
  - Vapor Manifest
---

Vapor.toml is the current pressure term for the [[Vapor Ecosystem]] manifest surface.
It is not Cargo.toml.
It is required for every Vapor artifact root.
Every [[Capability Module]] / [[Capability Node]] source folder should have an explicit Vapor.toml describing that
folder's typed declaration set.
Vapor.toml may describe a folder with zero Rhai files, for example a pure grouping folder or Rust-backed capability
folder.
It may appear next to Rust source folders, Rhai declaration folders, pack roots, and generated artifact roots.
It is the place where manifest-style metadata lives when that data should not be embedded directly inside one
[[Rhai Asset]] file, and it is also the normal place for attachment/dependency metadata.
It is also the source of truth for classifying Rhai files as [[Capability Type]], [[Capability Trait]], or
[[Capability Callback]] declaration assets.
It should also declare [[Capability Kernel]] imports and [[Kernel Artifact]] references when a module needs native
implementation backing.

Current owner-answer-informed uses:

- capability/file-level dependencies
- folder-level composition, nesting, organization, and storage integration metadata
- typed source-file classification for capability types, traits, callbacks, and child capability nodes
- kernel imports, native surface requirements, and module-level kernel artifact references
- visibility/publicness metadata
- packagepack, modpack, enginepack, and gamepack composition metadata
- target roles
- version requirements
- explicit conflicts
- compatibility hints
- Steam/Workshop publication metadata such as title, tags, visibility, changelog, preview image, support files, and
  dependency lists

Folder/composition shape pressure:

- Folder/project/pack placement metadata describes where a capability module or node folder sits in the physical or
  packaged composition structure.
- Placement metadata may imply typed source/declaration containment edges when the folder is part of the recognized
  capability module/node layout.
- Placement metadata must not silently imply execution flow, inheritance, callback invocation, or arbitrary causality.
- Reserved typed folders are expected for source organization, especially `types`, `traits`, `callbacks`, and
  `capabilities`.
- The `capabilities` folder is the recursive child-capability part of the source tree.
- Subfolder Vapor.toml files are scoped metadata fragments unless the manifest explicitly declares a nested capability
  module/node boundary.
- Every Vapor.toml describes only its own folder or artifact root as a unit, but all such metadata participates in the
  larger capability graph.
- Dependency objects should be explicit normal dependencies and may carry fields such as `id`, `path`, `version`,
  `kind`, `optional`, `reason`, and `features`, but exact schema is not locked.
- Conflict metadata should be explicit and may declare local or broader artifact/packagepack conflicts.
- Conflict declarations should generally be path+version style predicates, not deep content comparisons.
- `publishes` is not currently an accepted schema concept; publishing metadata exists, but the exact field shape is not
  locked.
  A section such as `steam.workshop` is plausible for Steam/Workshop publication metadata, but internal structure and
  ordering are not settled.

Boundary:
Rhai declarations remain foundational for authored capability declaration material.
Vapor.toml exists because some metadata is manifest-shaped and should be validated by launcher/SDK tooling before a
concrete engine/game fixture launches.
Vapor.toml is analogous to Cargo.toml as build-system/package-system metadata, while [[Rhai Asset]] files declare
capability types, traits, callbacks, and other typed capability-node material.
Kernel declarations in Vapor.toml should name required native backing without exposing raw private kernel internals as
public capability API.
Sidecar `.meta` files remain disfavored.

Open pressure:
[[Vapor.lock]] is the current pressure term for resolved dependency/fingerprint state.
Nested Vapor.toml files need careful treatment because nesting describes typed capability module/node metadata, not
arbitrary filesystem clutter.
Every meaningful folder should be explicit.
The exact field names for source classification, kernel imports, placement, dependencies, conflicts, and publication
metadata remain unsettled.

Phase 3 lock-candidate anchor:
The Phase 3 manifest scope is anchored by [Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md),
especially P3-W02.
Phase 3 must implement a real Vapor.toml parser/validator, explicit dependency/conflict objects, explicit
folder/composition/storage metadata, version constraints, and Steam/Workshop metadata sufficient for publish/install
flows.
Exact field names are not locked here.

#glossary
