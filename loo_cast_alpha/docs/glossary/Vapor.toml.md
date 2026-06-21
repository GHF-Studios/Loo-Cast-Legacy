---
canonical_name: Vapor.toml
status: WIP-draft
aliases:
  - Vapor Manifest
---

Vapor.toml is the current pressure term for the [[Vapor Ecosystem]] manifest surface.
It is not Cargo.toml.
It is required for every Vapor artifact root.
Every folder level that groups capability declarations should have an explicit Vapor.toml describing that folder's
capability declaration set.
Vapor.toml may describe a folder with zero Rhai files, for example a pure grouping folder or Rust-backed capability
folder.
It may appear next to Rust source folders, Rhai declaration folders, pack roots, and generated artifact roots.
It is the place where manifest-style metadata lives when that data should not be embedded directly inside one
[[Rhai Asset]] file, and it is also the normal place for attachment/dependency metadata.

Current owner-answer-informed uses:

- capability/file-level dependencies
- folder-level composition, nesting, organization, and storage integration metadata
- visibility/publicness metadata
- packagepack, modpack, enginepack, and gamepack composition metadata
- target roles
- version requirements
- explicit conflicts
- compatibility hints
- Steam/Workshop publication metadata such as title, tags, visibility, changelog, preview image, support files, and
  dependency lists

Folder/composition shape pressure:

- Folder/project/pack placement metadata describes where a capability declaration folder sits in the physical or
  packaged composition structure.
- Placement metadata must not imply code logic, execution flow, or causality.
- There is no implicit default model for nested Vapor.toml files.
- Every Vapor.toml describes only its own folder or artifact root as a unit.
- Dependency objects should be explicit normal dependencies and may carry fields such as `id`, `path`, `version`,
  `kind`, `optional`, `reason`, and `features`, but exact schema is not locked.
- Conflict metadata should be explicit and may declare local or broader artifact/packagepack conflicts.
- Conflict declarations should generally be path+version style predicates, not deep content comparisons.
- `publishes` is not currently an accepted schema concept; publishing metadata exists, but the exact field shape is not
  locked.
  A section such as `steam.workshop` is plausible for Steam/Workshop publication metadata, but internal structure and
  ordering are not settled.

Boundary:
Rhai declarations remain foundational for authored capability declarations.
Vapor.toml exists because some metadata is manifest-shaped and should be validated by launcher/SDK tooling before a
concrete engine/game fixture launches.
Vapor.toml is analogous to Cargo.toml as build-system/package-system metadata, while [[Rhai Asset]] files declare
capabilities.
Sidecar `.meta` files remain disfavored.

Open pressure:
[[Vapor.lock]] is the current pressure term for resolved dependency/fingerprint state.
Nested Vapor.toml files need careful treatment because nesting describes capability declaration folders, not arbitrary
filesystem clutter.
Every folder should be explicit.
The exact field names for placement, dependencies, conflicts, and publication metadata remain unsettled.

#glossary
