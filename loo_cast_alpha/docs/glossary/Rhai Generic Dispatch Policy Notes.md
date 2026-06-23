# Rhai Generic Dispatch Policy Notes

#tech_glossary

Status note:
This page mixes active declaration-first direction with legacy/quarantine dispatch evidence.
The declaration model is the stronger target signal; concrete dispatch catalogs and prefixes remain provisional until
reconciled with current Vapor/Rhai implementation.

Related glossary terms:

- [USF Instantiation Scripts](USF%20Instantiation%20Scripts.md)
- [USF Instantiation Capability Slot Notes](USF%20Instantiation%20Capability%20Slot%20Notes.md)
- [USF Runtime](USF%20Runtime.md)
- [Capability Slot Type](Capability%20Slot%20Type.md)
- [Capability Declaration](Capability%20Declaration.md)
- [Rhai Capability](Rhai%20Capability.md)
- [Capability Runtime](Capability%20Runtime.md)
- [Runtime Substrate](Runtime%20Substrate.md)
- [Rhai Reflection Macro Surface Notes](Rhai%20Reflection%20Macro%20Surface%20Notes.md)
- [Rhai Bridge Domains and Access Provider Notes](Rhai%20Bridge%20Domains%20and%20Access%20Provider%20Notes.md)
- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)
- [Workflow Framework](Workflow%20Framework.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)

Quarantine extraction (`TMP_rhai_semantic_reset_quarantine`) with high signal but provisional authority:

1. Rhai cannot request new Rust monomorphizations at runtime.
2. Generic-like behavior likely needs an explicit dispatch registry pipeline.
3. Signature ID prefixes (`QUERY_SIG__`, `MESSAGE_SIG__`, `BUNDLE_SIG__`, `RESOURCE_SIG__`) are strong candidates, not
   final law.
4. Canonical Rust path-style type/trait IDs are likely useful for traceability and determinism.
5. Duplicate/invalid registration and resolver-miss handling should be fail-fast by default unless we later define
   softer recovery paths.

Active legacy reflection/registration slice (`loo_cast_legacy/core_mod_api`) with concrete runtime signal:

1. Runtime binding metadata is inventory-collected into one `RuntimeBindingGraph`.
2. Duplicate IDs and missing required pairings (for example trait without trait-object metadata) hard-fail.
3. Engine bootstrap registers top-level modules from that graph (`register_binding_graph`).
4. Reflection/registration uses custom proc-macros (`reflect_extern_*`) in `core_engine_macros`.
5. This gives a deterministic metadata-first registration surface even where runtime bridge coverage is incomplete.

Active quarantine dispatch-surface signal (`TMP_rhai_semantic_reset_quarantine`):

1. Query/message/bundle/resource dispatch catalogs are compile-time registered and key-resolved at runtime.
2. Dispatch policy enforces canonical signature prefixes and canonical path-style type/trait IDs.
3. Resolver misses include available-key diagnostics and hard-fail by default.
4. Generic-like query metadata is explicit and catalog-backed; runtime does not synthesize new monomorphizations.

High-signal documentation anchor for reflection shape and layering boundaries:
[Rhai Reflection Macro Surface Notes](Rhai%20Reflection%20Macro%20Surface%20Notes.md)

Terminology correction (legacy-to-current mapping):

1. "Rhai type" is useful intuition but too ambiguous for this stack.
2. Preferred term for raw declared graph output is [[Capability Declaration]] / [[Capability Node]] material.
3. Current active wording replaces old profile/type-template/slot-type pressure with [[Capability Type]],
   [[Capability Trait]], [[Capability Extension Slot]], and signature metadata.
4. The useful remaining idea behind capability type templates is Rust-side host validation/materialization authority plus
   explicit type/trait/callback signatures.

Declaration-first posture (legacy MVP slice, superseded by module/node typing):

1. Old script profiles should now be interpreted as type/trait/callback source classifications or projection policies,
   not as final vocabulary.
2. One script/file no longer implies one singleton-like Capability Declaration.
   A Rhai file is one typed [[Rhai Asset]] contributing to a [[Capability Module]] / [[Capability Node]].
3. Legacy profile/type wording usually maps to Capability Type, Capability Trait, Capability Extension Slot, or
   [[Callback Type]], depending on the exact meaning.
4. Rust-side host validation/materialization wiring constrains declarations; older notes may call this capability
   type-template authority.
5. Extension slots are graph extension/dependency points with trait bounds, not script-produced objects.
6. Script execution yields data/declaration material, not raw Rust type objects.
7. Capability type/trait/callback signatures define the allowed script API graph topology: atomic capability nodes plus
   composite/category nodes.
8. Access is declared as include/exclude path declarations over that graph, so very specific capability-object subgraphs
   can be exposed.
9. Domains that are nonsensical, non-implementable for that type/trait/callback projection surface, or dangerous are
   intentionally omitted.
10. Capabilities exposed to scripts are [[Rhai Capability]] objects, identified by human-readable string IDs, and access
   to them is granted or denied by declaration/projection policy.
11. `ctx` is object-based and dynamic; domains/subdomains can open/close over time according to runtime policy and
   declaration context.
12. Runtime executes declaration entrypoints with type/trait/callback-tailored `ctx` subgraphs to produce capability
    node declaration material.
13. Callback closures declared by those entrypoints execute with callback-scoped effective `ctx` masks resolved by
    allow/deny policy, not implicit carry-over from declaration-entrypoint `ctx`.
14. A Rust materialization pass consumes those declarations and produces runtime capability machinery.
15. One Rhai file now leans toward one typed declaration or data asset, not one authored leaf capability node.
    Richer syntax, fields, parameters, and local construction code may exist inside the file, but public graph nodes
    must come from explicit module/node/type/trait/callback declarations.
16. Capability semantics are intentionally split:
    declaration-level [[Rhai Capability]] API surfaces and runtime-side Rust implementation/execution surfaces under
    [[Capability Runtime]] in the [[Runtime Substrate]].
17. This keeps scripts declaration/object-descriptor first while runtime behavior remains Rust-side.
18. Dependency semantics should remain layered: mod/provider resolution, declaration `ctx` path requirements, and
    post-lock runtime interaction must not be conflated.

Legacy dispatch extraction (still useful, but secondary to declaration semantics):

1. Reflect/declare operation metadata.
2. Register known signatures/catalog entries.
3. Resolve through deterministic keys/paths.
4. Execute through resolver/provider paths without bypass.

Open design space (rephrased around declaration/profile model):

1. How profile -> capability-type mapping is encoded and validated at load time.
2. How the one-script/one-capability-declaration invariant is enforced in tooling/runtime (error shape, diagnostics,
   migration path).
3. How atomic/composite API graph nodes are authored and versioned per profile.
4. How include/exclude path grammar, precedence, and conflict resolution are specified.
5. How API graph-domain allow/deny surfaces are reviewed and evolved per profile.
6. How capability-object grants/denials are declared, composed, audited, and dynamically opened/closed per profile.
7. How declaration entrypoints + `ctx` capability-object subgraphs map into capability declarations and then into
   runtime [[Capability Instance]]s without leaking unrelated domains.
8. How fail-fast vs softer failure policy is scoped per profile and environment.
9. Which registry/dispatch details remain global and which should become profile-local.
10. How declaration-seam events are shaped relative to execution-seam events without phase leakage.
11. How much of the legacy inventory+macro reflection stack should remain canonical vs being replaced by a smaller
    profile-first registry surface.
12. How Rhai-side capability usage differs from Rust-side capability kernel usage, including what crosses the bridge and
    what stays in host-side kernels.
13. How startup/logging/output callbacks should prove callback behavior in Phase 3 without prematurely locking hook
    terminology.

Raw-model alignment (math + scripting contract posture):

- script entrypoints are declaration-profile entrypoints first
- facade/bridge surfaces are the monomorphized Rhai-safe contract surface used by declaration entrypoints, `ctx`
  capability-object
  subgraphs, and capability objects
- the bridge layer should stay thin: translate between Rhai-friendly shapes and contract-facing typed surfaces
- workflow orchestration should consume materialized Rust-side artifacts and should not directly drive raw Rhai engine
  internals
- avoid exposing raw generic math trait surfaces directly to scripts
- preserve explicit kind/repr/policy selection semantics (`OpMode` + `OpPolicy`)

Legacy source pointers:

-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/documents/markdown_summary/rhai_generic_binding_policy.md`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/runtime/ecs/dispatch_policy.rs`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/documents/markdown_summary/rhai_bridge_playbook.md`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/catalog.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/scalar/shared.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/vector/shared.rs`
- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/meta/registry.rs`
- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/bind/engine_ext.rs`
- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/engine/preprocess.rs`
- `loo_cast_legacy/core_engine_macros/src/rhai_binding/reflection/mod.rs`

Rustdoc anchors:

- `crates/core_mod/src/spec/rhai.rs`
