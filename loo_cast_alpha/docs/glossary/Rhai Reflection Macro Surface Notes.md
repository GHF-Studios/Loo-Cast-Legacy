# Rhai Reflection Macro Surface Notes

#tech_glossary

Status note:
This page is a high-signal legacy implementation note, not final target doctrine.
The macro and registration model worked in the legacy tree and remains useful evidence for future Vapor/Rhai
reflection design.

Related glossary terms:

- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)
- [Rhai Value Semantics and AccessCell Notes](Rhai%20Value%20Semantics%20and%20AccessCell%20Notes.md)
- [Rhai Bridge Domains and Access Provider Notes](Rhai%20Bridge%20Domains%20and%20Access%20Provider%20Notes.md)
- [USF Instantiation Scripts](USF%20Instantiation%20Scripts.md)
- [Capability Runtime](Capability%20Runtime.md)
- [Workflow Framework](Workflow%20Framework.md)

Intent-first anchor:

The reflection macro surface is a high-signal, low-noise working framework surface for Rhai-facing API registration.
It is not the single central pillar; it is one reliable architecture signal among many, and should be read alongside
other working framework surfaces such as the [Workflow Framework](Workflow%20Framework.md).

Core macro families:

1. Extern declarative family (`reflect_extern_*!(...)`):
   explicit metadata declaration for modules, functions, traits, types, and generic metadata/instantiations.
2. Attribute family (`#[reflect_*]`):
   local AST-driven reflection from Rust items (`type`, `trait`, inherent impl, trait impl, function forms).
3. Marker attributes in impl bodies (`#[reflect_constructor_function]`, `#[reflect_method_function]`,
   `#[reflect_item_associated_function]`):
   consumed by impl-level reflection macros; pass-through by themselves for ergonomic local annotation.

Registration pipeline shape:

1. Macros generate metadata marker/static entries and submit them through `inventory`.
2. Runtime builds one `RuntimeBindingGraph` from collected metadata.
3. Duplicate IDs or missing required pairings hard-fail during graph build.
4. Engine bootstrap registers top-level modules from that graph.
5. Script preprocess validates alias/use behavior against known global symbols from that same reflected graph.

This creates one deterministic naming/registration universe across declaration and execution seams.

Concrete invariants observed in active legacy code (`loo_cast_legacy/core_mod_api` + `core_engine_macros`):

1. `RuntimeBindingGraph::build()` collects all metadata families (modules, traits, types, impls, functions, generic
   definitions, generic instantiations) via `inventory` and hard-fails on duplicate IDs.
2. Trait registration is a required pair: trait metadata without trait-object metadata hard-fails graph build.
3. Engine bootstrap registers top-level modules from the graph and recursively binds submodules, type-binding modules,
   impl function surfaces, and module/item/constructor/method entries.
4. Script preprocess enforces strict `use <path> as <alias>;` syntax, validates aliases against reserved keywords and
   known global symbol names, and rewrites alias tokens to canonical paths.
5. Macro-declared generic metadata (`reflect_extern_generic_definition` and
   `reflect_extern_generic_instantiation`) is explicit metadata registration, not runtime monomorphization creation.

Important semantic framing:

1. Macro reflection defines registration topology and binding metadata shape.
2. Script profiles and capability roots define what scripts are allowed to use.
3. Runtime orchestration (`intent -> reconcile -> commit -> apply`) defines how emitted behavior is processed.
4. These are related but distinct layers; docs should not collapse them.
5. Reflection-macro docs should be treated as one strong signal surface among multiple pillars, not as hierarchy apex.

Implication for glossary-premise work:

1. Reflection macros and binding-graph build are the deterministic naming/registration substrate.
2. Profile/policy gating semantics should be layered above that substrate, not fused with it.
3. Panic-fast behavior at metadata-build and preprocess boundaries is already established by code-level signal.

Example posture:

1. Active concise example surface:
   `loo_cast_legacy/core_mod/src/script_channels/*` (`ctx::math::scalar` + `ctx::math::vector`).
2. Rich, broader examples:
   `loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/...`.
3. These examples are architecture-shape signals and reference exemplars; they are not required to look like
   final production ergonomics to be useful.
4. Runtime/content wiring around examples can drift over time; macro-surface semantics and registration topology are
   the durable high-signal layer to preserve.

Legacy source pointers:

- `loo_cast_legacy/core_engine_macros/src/lib.rs`
- `loo_cast_legacy/core_engine_macros/src/rhai_binding/reflection/mod.rs`
- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/meta/registry.rs`
- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/bind/engine_ext.rs`
- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/engine/preprocess.rs`
- `loo_cast_legacy/core_mod/src/script_channels/mod.rs`
- `loo_cast_legacy/core_mod/src/script_channels/math/mod.rs`
- `loo_cast_legacy/core_mod/src/script_channels/math/scalar.rs`
- `loo_cast_legacy/core_mod/src/script_channels/math/vector.rs`
- `loo_cast_legacy/documents/markdown_summary/rhai_macro_surface.md`
