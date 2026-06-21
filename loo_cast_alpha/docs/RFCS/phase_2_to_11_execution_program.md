# Phase 2 to 11 Execution Program (Working Draft)

Status: Active draft for roadmap consolidation toward `0.5.0` alpha.

Authority caveat:
This file is a roadmap crystallization surface, not current doctrine by itself.
Historical `Locked` and `Provisional-Locked` labels in this file must be treated as historical unless reconfirmed by
current owner direction or the latest glossary.
If this file conflicts with the latest glossary or owner-answer logs, treat this file as stale.

Current owner-answer-informed phase framing (2026-06-17):

- Phase 2: Vapor + USF Planning and Spec Crystallization.
- Phase 3: Vapor Launcher and Product-Stack Proof.
- Phase 4: Product-Stack + USF Prototype/MVP.

This supersedes older Phase 3 wording that framed the phase as a USF runtime spine restoration.
Phase 3 coding must prove Vapor launcher/product-stack/modpack/capability semantics with real Steam/Workshop
integration, while excluding USF/worldmodel implementation.
Phase 4 is the first USF-inclusive product-stack prototype/MVP phase.

Purpose:

- Hold one coherent execution program for Phases `2..11`.
- Preserve viewpoint-style infodumps while turning locked points into actionable phase work.
- Keep interlocks explicit instead of pretending phases are fully independent.

Source inputs:

- V1 input docs:
  - `draft_phase_buckets.md`
  - `alpha_doctrine_draft.md`

---

## Program Constraints (Historical Lock Snapshot)

1. Strict phase numbering is `2..11`.
2. `Rxx` units are micro-RFC style decision/task units.
3. `Rxx` units map `1:1` to GitHub issues (sub-issues allowed for large work).
4. Milestones remain the active phase authority surface during execution.

---

## Viewpoint Model

- V1: Current worldview/program baseline (already captured in source inputs above).
- V2: Modding + capability contracts (captured below in this file).
- V3: USF/USF math/foundational USF + USF-aware capability integration (deferred for now).

---

## V2 Snapshot (Modding + Capability Contracts)

### Terminology (Provisional-Locked)

2026-06-17 note:
This terminology snapshot is historical where it uses unqualified `package` or `composite package`.
Current active vocabulary prefers [[Packagepack]], [[Enginepack]], [[Gamepack]], [[Modpack]], Steam package,
[[Source Artifact]], [[Build Artifact]], or [[Distributable Artifact]] depending on the meaning.

- `mod`:
  - exactly one Rust crate
  - exactly one `PluginGroup`
  - can contain whatever internals/assets are needed for its contract
- `package`:
  - compiled, packaged mod artifact including assets
- `modpack`:
  - composition of multiple mod/plugin groups
- `composite package`:
  - compiled/packaged modpack artifact, explicitly acknowledging recursive/nested composition

### Conflict Model (Locked)

- Conflict model is `mod-wide explicit conflicts + scoped invariant enforcement safety net`.
- Explicit conflicts are author-declared incompatibilities.
- Invariant enforcement still validates safety boundaries even when explicit conflict declarations are incomplete.

### Versioning Direction (Provisional)

- SemVer-aligned direction.
- Strong lockstep tendency for entrypoint/first-party/private statically-linked surfaces (detail schema still open).

### Failure Policy (Locked for Pre-Alpha Drafting)

- Hard-fail by default for invalid states (`no soft allow` path in current pre-alpha planning posture).

### Capability Provider Model (Provisional-Locked)

- Provider cardinality is `declared per capability`.
- Provider-need ambiguity is hard-invalid:
  - if a capability requires singleton and there are `0` or `>1` eligible providers in scope, resolution must hard-fail
  - no warning-and-continue behavior
- Multi-provider ordering policy allows both:
  - graph/topology-derived order
  - explicit priority metadata
  - exact merge/precedence semantics remain capability-specific

### Capability Taxonomy (V2 Provisional-Locked)

- `authority/*`:
  - default shape is singleton provider-ownership capability
  - marked as "slightly weird" and requires one targeted refinement pass before final lock
- `registry/*`:
  - multi-provider ordered set capability class
- `hook/*`:
  - multi-provider ordered execution capability class
- `service/*`:
  - singleton-by-default service capability class
  - no pre-alpha tie-break path for singleton-critical ownership conflicts

### Scoped Capability Context Model (V2 Provisional-Locked)

- Capabilities must be localizable by explicit scope keys.
- `scale-layer` is a first-class scope axis (RenderLayers-like concept at capability-contract level).
- Cardinality/resolution rules are evaluated per scope key (for example singleton-per-scale rather than singleton-global).
- Composite capabilities are first-class and may orchestrate many primitive/composite capabilities under one higher-level
  contract.
- Rendering is treated as a canonical composite-capability example (not the only one):
  - scoped sub-results may be computed per scale/context
  - sub-results may be composed into one final output surface
  - this pattern is intended as a general capability-composition direction, not rendering-only doctrine

### Alpha Security/Packaging Simplification (Locked)

- No package hash/fingerprint requirement is mandated for alpha baseline.

Current correction:
This lock snapshot is historical for active Phase 3 Vapor planning.
Current owner-answer-informed pressure requires [[Fingerprint]] and [[Vapor.lock]]-style resolution/fingerprint data for
Phase 3 packagepack validation, diagnostics, publishing, caching, and reproducibility.

### Runtime Provider-Ownership Policy (Locked for Pre-Alpha)

- Pre-alpha policy: provider ownership is load-time locked.
- Runtime provider-ownership reassignment is not required for pre-alpha closure.
- Runtime reassignment is explicitly an alpha/post-alpha exploration surface.

### `authority/*` Refinement Draft (V2 Working)

Intent:

- Define singleton-critical provider ownership semantics in a scoped, deterministic, and fail-fast way.

Core terms:

- `authority scope key`:
  - the concrete scope tuple where ownership is evaluated
  - includes capability id + declared scope axes (for example `scale-layer`, plus optional context keys)
- `authority claim`:
  - a provider's declaration that it can own a given `authority/*` capability for a given scope selector
- `effective owner`:
  - the single resolved provider for one capability at one authority scope key

Pre-alpha resolution algorithm (draft):

1. Build candidate provider set from active mod graph after dependency/conflict filtering.
2. Match providers whose authority claims apply to the current authority scope key.
3. If no candidate exists, hard-fail (missing required owner).
4. If exactly one candidate exists, select it.
5. If multiple candidates exist, treat as automatic incompatibility and hard-fail validation/load.
6. Lock resolved effective owner for that scope key at load-time for pre-alpha runtime.

Draft policy boundaries:

- Ownership is evaluated per scope key, not globally.
- User-layer basic mode does not perform authority-owner overrides in pre-alpha.
- Pre-alpha does not require tie-break/owner-selection strategy surfaces for singleton-critical authority capabilities.

### Pre-Alpha Manifest Contract Shape (V2 Working, Non-Field-Level)

Intent:

- Define the minimum semantic declaration surface needed for deterministic load validation and capability resolution.
- Avoid low-level schema bikeshedding until contract shape is stable.

Required semantic sections (draft):

1. Identity + version:
   - string-based mod identity
   - SemVer-aligned mod version
2. Dependency declaration:
   - required/optional dependencies with cargo-like version constraints
3. Capability authority graph declaration:
   - capabilities exposed by this mod
   - capability slots/requirements this mod expects to be filled
   - scope-localized singleton/multi-provider requirements per capability as needed
   - capability graph must resolve to concrete 1:1 ownership where singleton-critical
4. Incompatibility behavior (capability-driven first):
   - primary incompatibility detection is implied by unresolved/invalid capability graph state
   - explicit mod-wide conflict declarations are optional in pre-alpha and non-primary
5. Packaging/runtime metadata:
   - declared asset/runtime resource mapping needed for package/composite-package load
   - asset authority resolution follows the same capability-graph ownership model
   - no pre-alpha hash/fingerprint requirement

Current correction:
This section uses older `package` / `composite package` / manifest vocabulary.
Active planning should translate it through [[Packagepack]], [[Enginepack]], [[Gamepack]], [[Modpack]], [[Vapor.toml]],
[[Vapor.lock]], and [[Fingerprint]] before implementation.

Validation posture:

- Manifest validation is pre-runtime and fail-fast.
- Any missing required semantic section or semantically invalid section blocks launch flow.

### Slot-Graph Composition Model (V2 Working)

- `core_engine` exposes a root slot.
- One `core_mod` occupies that root slot.
- Current correction: `core_mod` is mandatory, but it is replaceable through selection of another valid coupled
  `core_engine`/`core_mod` pair.
  It is not optional, freely removable, or independently mix-and-match replaceable as a gameplay mod.
  `core_engine`, `core_mod`, and `base_mod` are reserved built-in mod roles in a Vapor product instance stack.
- A mod may expose additional slots and may require other slots to be filled.
- Slot filling is recursive/nested through the mod capability graph.
- Load is valid only if required singleton slots resolve to exactly one owner per scope key and graph invariants hold.
- Any unresolved required slot or multiply-owned singleton-critical slot causes hard-fail during validation/load.
- This model is intended to allow intricate cross-mod compositions that still remain machine-resolvable and fail-fast safe.

---

## Resolution Domains (Split by Concern)

Resolution policy is not one global rule. It is split into explicit domains:

1. Dependency/version resolution.
2. Incompatibility resolution (capability-driven first, explicit conflicts optional).
3. Capability-provider resolution.
4. User composition resolution (launcher/config layer).

---

## Pipeline Model (Working Structure)

This is currently one pipeline baseline plus additional pipeline surfaces to define:

1. Mod load/runtime pipeline (baseline accepted):
   - discover -> parse manifest -> validate -> resolve -> init -> runtime -> shutdown
2. Mod composition/modpack authoring pipeline (needs explicit contract pass).
3. Mod developer pipeline (SDK/tooling author workflow, needs explicit contract pass).
4. Root developer pipeline (partially addressed by Phases 0/1; no new deep pass yet).

---

## Phase Interlock Policy (Program Rule)

To model real interleaving without losing ownership:

- Every `Rxx` gets one `home phase`.
- `Rxx` may declare cross-phase prerequisites/dependencies.
- Phase closure requires:
  - all home-phase `Rxx` meeting exit criteria
  - all declared prerequisites for those `Rxx` being satisfied

This keeps accountability clear while acknowledging non-linear dependency reality.

---

## Draft Phase Intent Map (2..11)

- Phase 2: Vapor + USF Planning and Spec Crystallization.
- Phase 3: Vapor Launcher and Product-Stack Proof.
- Phase 4: Product-Stack + USF Prototype/MVP.
- Phase 5: implement USF semantics contracts.
- Phase 6: plan USF math contracts.
- Phase 7: implement USF math contracts.
- Phase 8: plan broad capability platform needed before gameplay.
- Phase 9: implement capability platform baseline.
- Phase 10: plan gameplay slice target.
- Phase 11: implement gameplay slice + human validation proof.

---

## Open Decisions Queue (V2 Next Pass)

1. Reconcile this file with glossary corrections from `question_batch_001.txt` through `question_batch_004a.txt`.
2. Replace old unqualified package/manifest language with Packagepack/Enginepack/Gamepack/Modpack, [[Vapor.toml]],
   [[Vapor.lock]], [[Fingerprint]], and [[Rhai Asset]] vocabulary.
3. Draft the Phase 3 scope ceiling and execution spec around [[Phase 3 Vapor Scenario Suite]] rather than a single
   canonical command or USF runtime spine.
4. Draft the Phase 4 USF-inclusive execution spec before Phase 3 coding begins.
5. Capability/slot semantics pass: decide how authority claims, exclusive slots, variadic slots, ordered registries,
   optional providers, and integration apertures relate.
6. Phase completion proof definitions for Phase 3 and Phase 4.

---

## Editing Protocol for This File

- Append new infodumps as `Vn` sections (do not overwrite previous viewpoint data).
- Promote locked points into concrete `Rxx` items.
- Keep unresolveds explicit and phase-routed.
