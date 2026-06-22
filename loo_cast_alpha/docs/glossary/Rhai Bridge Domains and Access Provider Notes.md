# Rhai Bridge Domains and Access Provider Notes

#tech_glossary

Status note:
This page is legacy/quarantine evidence, not settled target doctrine.
It preserves useful bridge and access-provider signals from `loo_cast_legacy` and
`TMP_rhai_semantic_reset_quarantine` for later reconciliation.

Related glossary terms:

- [Rhai Reflection Macro Surface Notes](Rhai%20Reflection%20Macro%20Surface%20Notes.md)
- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)
- [Rhai Value Semantics and AccessCell Notes](Rhai%20Value%20Semantics%20and%20AccessCell%20Notes.md)
- [Capability Projection API](Capability%20Projection%20API.md)
- [Script Safety](Script%20Safety.md)
- [Capability Runtime](Capability%20Runtime.md)

Research scope anchor:

1.
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/bridges/testing`
2.
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/bridges/domains`
3. Adjacent runtime/value-semantics code under the same quarantine slice.
4. Active legacy implementation signals under `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/*`.

High-signal findings from the bridge folders:

1. Bridge modules are domain-mirrored and hierarchical (`bevy::ecs`, `core_mod_api::*`, `std::*`, plus `testing::*`).
2. The `testing/shop/divisions/sex.rs` slice demonstrates end-to-end macro coverage in one file:
   trait, type, inherent impl, trait impl, constructor/method/item functions, and module-associated function.
3. Production bridge slices are facade registrations, while runtime behavior is delegated to runtime wrapper types and
   internals (`runtime/ecs/*`, `runtime/std/*`).
4. `world` and `commands` bridge surfaces use callback-style access (`FnPtr`) for scoped contextual execution.
5. Query/message/resource/bundle behavior uses explicit dispatch keys and compile-time catalogs, then resolves at
   runtime
   through registries.

Access-provider and scoped-borrow findings:

1. `AccessCellProvider` implementations act as method-keyed gate points for scoped access windows.
2. Provider calls are request-shaped (`Box<dyn Any>` payloads) and panic-fast on unsupported method names or payloads.
3. Scoped windows are explicitly opened and closed; lifecycle completion is mandatory before returning to scheduler
   flow.
4. Lifetime erasure/restoration patterns (via `transmute`) are used in provider implementations and therefore require
   strict usage discipline.

Semantic implication for glossary work:

1. Bridge code is not just binding syntax; it encodes an authority-gating pattern (
   `projection -> scoped access -> close`).
2. Reflection metadata and dispatch catalogs define what can be surfaced at all.
3. Scoped provider contracts define how that surface is safely and synchronously accessed at runtime.
4. This aligns with the current projection/safety model: raw global graph is not script-safe, contextual facades are.

Current ambiguity/gap markers:

1. Quarantine slice is strong signal but not canonical by itself.
2. Active legacy and quarantine differ in completeness and organization.
3. Profile/policy grammar (include/exclude precedence, diagnostics taxonomy) remains underdefined.
4. Dynamic profile nesting and capability-profile composition rules are still design-level WIP.

Source pointers:

-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/bridges/testing/shop/divisions/sex.rs`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/bridges/domains/bevy/ecs/world.rs`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/bridges/domains/bevy/ecs/catalog/sysparam_providers.rs`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/runtime/ecs/dispatch_policy.rs`
- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/meta/registry.rs`
- `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/engine/preprocess.rs`
