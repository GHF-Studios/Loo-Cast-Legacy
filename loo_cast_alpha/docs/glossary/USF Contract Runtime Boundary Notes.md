# USF Contract Runtime Boundary Notes

#tech_glossary

Related glossary terms:

- [USF Contract](USF%20Contract.md)
- [USF Runtime](USF%20Runtime.md)
- [Capability Runtime](Capability%20Runtime.md)
- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)
- [Modding Runtime](Modding%20Runtime.md)
- [Contract Family](Contract%20Family.md)

Boundary notes:

1. `core_engine` is treated as composition/runtime host boundary.
2. USF is treated as a pivotal public/API-facing Spacetime Engine module/framework part, not a standalone product layer.
3. Capability + modding runtime composition is sibling-runtime coupling, not contract redefinition.
4. USF is not currently modeled as a directly/exclusively replaceable Vapor-level component.
5. Replacing USF means forking/modifying the Spacetime Engine enough that the result is effectively another Engine.
6. Seam analysis is phase-tagged: declaration seams (script load/validate/lock path) and execution seams
   (post-lock runtime evolution path).

This mirrors the ownership split direction from legacy notes:

- `core_engine`: composition/runtime host
- `core_mod_api` / `base_mod_api`: runtime and capability surfaces

Legacy source pointers:

- `loo_cast_legacy/core_mod_api/src/lib.rs`
- `loo_cast_legacy/base_mod_api/src/lib.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_engine_capability_stack_raw_model/new_insights.md`

Rustdoc anchors:

- `crates/core_engine/src/spec/mod.rs`
- `crates/core_mod/src/spec/mod.rs`
