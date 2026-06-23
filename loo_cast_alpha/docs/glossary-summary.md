# Glossary Summary

Status: canonical split page-by-page review artifact for `#glossary` pages.
Individual glossary pages remain the detailed doctrine/framing surfaces; this summary is the canonical navigation and review layer for detecting mismatches to reconcile.
Dangling Obsidian links are allowed and can be useful future-concept pressure.

Source scope: current Markdown pages in `loo_cast_alpha/docs/glossary/`. Obsidian workspace metadata under `.obsidian/` is editor state and is not summarized here.

## Mission-Level Synthesis

The central architecture is a capability-first Vapor ecosystem. A Capability is not just an API function or artifact label; it is the common graph primitive for identity, authority, projection, composition, validation, runtime execution faces, and diagnostics. Artifact and pack terms exist to describe source/build/distribution/composition containers, but the runtime truth is the resolved capability graph after Runtime Lock.

Vapor is currently Steam-exclusive. Phase 3 is a proof of SDK, launcher, Steam/Workshop flows, packagepack composition, capability/Rhai declaration loading, diagnostics, fingerprints, and hello-world-style engine/game fixtures. Phase 3 deliberately does not prove USF worldmodel, gameplay, rendering, save/load, or full simulation behavior.

The Spacetime Engine and Loo Cast are first-party instances inside Vapor. The Engine role is a coupled `core_engine` plus matching `core_mod`; the Game role is `base_mod`. USF is a public/API-facing Spacetime Engine subsystem, not a standalone Vapor product layer.

The Capability Framework is the canonical Vapor framework; "scripting framework" is only an informal alias. Rhai is the canonical typed authoring surface for data assets, capability declarations, and sanctioned callbacks, but not the raw runtime authority surface. Data assets include config, localization, constants, tuning values, authored tables, and similar structured payloads. Declarations may carry data and metadata, but declaration-level logic crosses the framework boundary only as callbacks. Rust Host Contracts, Scriptable Rust Surfaces, the Rust Surface Graph, Capability Kernels, and Kernel Artifacts provide native backing where needed.

Runtime composition is closed after Runtime Lock. Design remains open between lock cycles, but post-lock graph mutation is forbidden by default. Runtime dynamism should be expressed through explicit capabilities, registries, runtime substrate policy, or kernel-backed native surfaces, not arbitrary structural mutation.

Native kernel loading is WIP doctrine: Capability Modules may import module-scoped Kernel Artifacts, likely platform-specific dynamic libraries. The required plugin boundary is a narrow Rust-native registration entrypoint into vapor_core, after which Vapor-owned Rust registry contracts take over. Vapor.lock and the Vapor Toolchain Envelope must prove compatibility; if they cannot, loading fails fast, and C ABI entrypoints are not accepted for Vapor kernel loading.

The workflow notes are legacy implementation evidence for Rust-side orchestration. They document typed workflow requests, domain-specific stage buffers, Bevy-visible stage systems, composite wrappers, timeout behavior, and known refactor hazards such as placeholder stage slots and generated `transmute` handoffs.


## Every #glossary Page

### Artifact Compatibility Envelope.md

Defines the rule boundary for when source, build, and redistributable development artifacts can be reused versus rebuilt. It treats source as the canonical sharable artifact class and allows cached redistributable implementation-library builds per compatible target bucket, while requiring source archival so artifacts can always be rebuilt without external repository availability.

### Artifact.md

Defines Artifact as a concrete file, folder, archive, build output, distribution object, or installed/downloaded object in the Vapor ecosystem. It explicitly excludes in-memory Capabilities and splits precision terms into Source Artifact, Build Artifact, Distributable Artifact, Workshop item, Packagepack, Enginepack, Gamepack, and Modpack.

### Asymmetric Failure Doctrine.md

States the failure posture: startup/config invalidity should block launch with structured diagnostics where possible, while runtime invariant violations may still panic visibly. Persistence-sensitive paths such as save/load need stronger corruption-avoidance, backup/autosave, atomic or replace-safe write patterns, and hard failure when the safe path is no longer trustworthy.

### Build Artifact.md

Defines Build Artifact as built output from source that has not yet been assembled, packaged, or published as a final distributable. It is distinct from Distributable Artifact and from in-memory Capability objects. This is akin to a fully linked shared object or binary but without the dynamically linked libraries nearby/packaged.

### Callback Context Type.md

Defines the dedicated `ctx` type projected into a Callback Type / Capability Callback. Each callback type should have its own projected context, and trait callbacks should be shaped by the declaring Capability Trait, implementing Capability Type, and resolved projection policy.

### Callback Scope Envelope.md

Defines the concrete capability/API projection envelope used for callback invocation contexts. It is a specific instance of the broader Capability Graph Scope Envelope.

### Callback Signature.md

Defines metadata that identifies a Capability Callback context and invocation shape without requiring the fully resolved capability graph. It is the callback-side member of the broader signature family beside Capability Type Signature, Capability Trait Signature, and Capability Instance Signature.

### Callback Type.md

Defines the type of a Capability Callback. It describes scheduled/mediated callback entrypoints, may encode firing policy such as single-fire or multi-fire, has a dedicated Callback Context Type, and is not arbitrary callable logic.

### Capability Bootstrap Fixed-Point Cycle.md

Defines the deterministic iterative startup process that materializes resolvable capability/API layers. First-order declarations are root-level and cannot depend on other capabilities; dependency cycles are invalid.

### Capability Callback.md

Defines a capability-relevant scheduled/mediated logic entrypoint. Capability Callbacks have Callback Types, Callback Signatures, projected Callback Context Types, and may be declared by Capability Traits and implemented by Capability Types; arbitrary functions are not capability concepts by default.

### Capability Contract.md

Defines the Capability Contract Family as the overloaded umbrella for capability metadata, declaration, projection, and runtime/instance rules. It now splits old slot/template pressure across Capability Type, Capability Trait, Capability Extension Slot, and the signature family while preserving provider-vs-declaration dependency separation.

### Capability Declaration.md

Defines the raw authored form of a Capability Node before validation/materialization into a Capability Instance. It can be spread across a Capability Module/Node source subtree with typed files for Capability Types, Capability Traits, and Capability Callbacks plus Vapor.toml metadata. Declared material is data-first, and declaration-level behavior crosses the capability boundary only as sanctioned callbacks.

### Capability Graph Diagnostics.md

Defines diagnostics produced from invalid or suspicious capability, trait, callback, signature, or extension-slot graph states. It recommends classifying errors by graph primitive first, then projecting them into player, modpack-author, and developer views.

### Capability Graph Scope Envelope.md

Defines the hard maximum API/context scope allowed for a profile-bound script or callback. Runtime policy may narrow or re-open paths within this envelope, but can never widen access beyond it.

### Capability Path.md

Defines a stable addressing route inside a capability/API graph or projection facade. Paths are used for policy allow/deny decisions and should not imply dependency, causality, execution flow, or folder-nesting defaults by themselves.

### Capability Projection API.md

Defines script-safe or callback-safe contextual facades projected from the capability/API graph. These lightweight Rhai-facing handles include declaration `ctx` and callback `ctx`, are not raw access to the unrestricted host graph, and absorb the old Scripting Projection Meta-Layer concept because scripting projection is capability projection.

### Capability Resolution Semantics.md

Defines resolution as layered, not singular: dependency graph bootstrap, materialization/merge into the host graph, and projection/access policy are separate. Required unresolved dependencies, cycles, invalid multiplicity, or denied required callback paths hard-fail; no implicit fallback owner appears.

### Capability Role.md

Defines Capability Role as an unresolved term for how a capability participates in graph semantics. It warns that input/output wording is incomplete and that mutation authority belongs outside capability objects until the runtime authority pipeline is modeled explicitly.

### Capability Runtime.md

Defines the runtime orchestration layer for capability discovery, registration, validation, coordination, and execution routing. It builds and locks one resolved launched Engine/Game runtime graph as the main authority source, while metadata registries, lockfile/fingerprint material, and projections remain supporting views; canonical mutation decisions stay in host reconcile/commit/apply paths.

### Capability Extension Slot.md

Defines an explicit capability graph extension/dependency point. It accepts candidate Capability Instances when their Capability Type Signatures satisfy required Capability Trait bounds and additional slot policy such as cardinality, ordering, optionality, replacement, or conflicts.

### Capability Framework.md

Defines the Vapor-owned framework for capability graph declarations, data assets, sanctioned callbacks, traits, extension slots, projection, runtime binding, host contracts, scriptable Rust surfaces, kernels, and runtime behavior. `Scripting Framework` is an informal alias only.

### Capability Kernel.md

Defines module-scoped native implementation backing for Capability Modules. A kernel may expose many Rust functions, types, constructors, adapters, systems, callbacks, or registries, and should not be treated as one tiny executable unit per capability.

### Capability Instance.md

Defines a concrete validated/materialized occurrence of a Capability in a specific graph environment. Use it for staged or runtime graph objects; use Capability for the broad model/type/contract concept and Capability Declaration for authored pre-materialization payloads.

### Capability Instance Signature.md

Defines metadata/fingerprint-like description of a concrete declared, staged, or runtime capability graph node. It can include graph identity, path, source/module path, type signatures, implemented traits, callbacks, child relationships, and diagnostics material.

### Capability Module.md

Defines a typed source-layout unit for authoring capability graph material. A module is usually a folder with Vapor.toml, generated Vapor.lock, reserved typed subfolders, and optional module-scoped Capability Kernel imports through Kernel Artifacts or built-in Rust Surface Graph entries.

### Capability Node.md

Defines the declared/source-side graph node that can later become a Capability Instance. Capability Declaration is the raw declared form of a Capability Node, usually represented by a module-shaped source subtree rather than one Rhai file.

### Capability Slot Type.md

Legacy bridge term now superseded by Capability Extension Slot plus Capability Trait bounds and signature metadata. Old usages may map to extension slots, traits, callback types, projection scopes, or host-contract wiring depending on context.

### Capability Trait Signature.md

Defines metadata/fingerprint-like description of a Capability Trait. It identifies required trait callbacks, projection surfaces, compatibility constraints, and diagnostics material for implementers and extension-slot validation.

### Capability Trait.md

Defines a dynamic, Rhai-compatible, Rust-trait-like capability-side interface/marker/contract that Capability Types can implement. Traits can declare trait callbacks, and Capability Extension Slots use trait bounds to validate accepted candidates.

### Capability Type Signature.md

Defines metadata/fingerprint-like description of a Capability Type, including identity, implemented traits, callbacks, projection/context surfaces, compatibility metadata, and fingerprint material.

### Capability Type.md

Defines the declared kind/category of capability that can later produce or classify Capability Instances. It is defined by typed source files inside Capability Modules/Nodes, can implement Capability Traits, and replaces old template-style thinking.

### Capability-Centric Semantics.md

Defines the doctrine that project meaning and authority are modeled through capabilities first, while execution still uses lower-level runtime substrate primitives. Global utility surfaces such as math/logging can exist, but only as explicit bounded capability surfaces.

### Capability.md

Defines Capability as Vapor's intentionally broad runtime/contract graph primitive spanning ability, authority, API exposure, metadata, composition, validation, and orchestration. The active model splits source/type semantics into Capability Modules, Nodes, Types, Traits, Extension Slots, Callbacks, Signatures, Instances, Host Contracts, Scriptable Rust Surfaces, and Kernels.

### Chunk.md

Defines Chunk as the first-level USF spatial partition at a canonical Scale, fixed at `1000^3` scale-local units per scale. Chunks can act as address/cache/generation/simulation/realization boundaries, but significant world authority should live in Phenomena, with chunk state serving residual, local, cache, temporary, or metric-oriented roles unless explicitly promoted.

### Closed Runtime and Open Design.md

Defines a bridge phrase for the combination of Runtime Lock and Managed Ambiguity. It should not carry much standalone doctrine: concrete runtime closure rules belong in Runtime Lock, while unresolved design tracking belongs in Managed Ambiguity.

### Complexity Gradient.md

Defines a continuous spectrum from automated low-complexity interaction to direct high-complexity system control. It supports the Player-to-Creator Path by exposing deeper mechanisms through wrappers over the same underlying systems rather than separate modes of reality.

### Contract Family.md

Defines a Contract Family as a coherent group of related contract definitions governing one semantic concern. Examples are capability, modding, and USF families, each with declaration/validation boundaries and runtime counterparts.

### Contract.md

Defines Contract as the umbrella over contract families across Vapor, Spacetime Engine, and Loo Cast. It is not one mechanism; each family must have supporting runtime/backends, and USF remains an internal Spacetime subsystem contract rather than a standalone Vapor product contract.

### Declaration Scope Envelope.md

Defines the concrete capability/API projection envelope applied to declaration entrypoint execution. It is a sibling concrete envelope to Callback Scope Envelope under the broader Capability Graph Scope Envelope.

### Distributable Artifact.md

Defines Distributable Artifact as the final packaged object produced by Vapor tooling for upload/distribution, with Steam Workshop as the Phase 3 public target. `Authoring Artifact` is preserved as a strong alias for the author-facing packaged form assembled around linked executables/shared objects, runtime libraries, payloads, manifests, dependency contents, and publication metadata.

### Dynamic Authority Resolution.md

Defines authority as lifecycle- and operation-relative rather than one global static fact. Runtime policy can narrow or re-open access within the Capability Graph Scope Envelope, while definition, runtime, callback, reconcile/commit/apply, and persistence authority remain distinct.

### Engine Mod.md

Defines an Engine Mod as a mod attached to an Engine fixture or Engine-facing extension point. Engine Mods attach under Enginepack, may affect Engine fingerprint/identity, and are required in Phase 3 to prove real Engine-side extension.

### Engine.md

Defines Engine as a mandatory Vapor product/composition role represented by a coupled `core_engine` plus matching `core_mod`. Non-Spacetime engines may exist and may ignore much of the recommended capability model after bootstrap. Open pressure: the docs should separate broad non-Spacetime engine allowance from USF product-slot framing.

### Enginepack.md

Defines Enginepack as the user/modpack-author-facing object selecting one Engine fixture plus Engine-attached contributions. It must select a coupled `core_engine`/`core_mod` pair, contribute downstream compatibility/fingerprints, and prove default, alternative, and modded Engine fixtures in Phase 3.

### Entity Plane Split.md

Defines a modeling pattern where one conceptual entity is represented across backend and frontend planes. Backend owns interaction/state authority; frontend owns visible presentation, enabling projection tricks, scale-relative visibility, portals, and wrapping without collapsing authority boundaries. It also helps keep traditional f32/f64-based libraries such as Rapier usable behind projection boundaries.

### Entity Proxy.md

Defines a coupled proxy representation of another entity for behavior, visibility, or both. Proxy modes may be backend, frontend, or dual-plane and compose with observer-relative simulation and portal traversal. It also supports world-wrapping implementations, though portal traversal may carry the stronger semantic framing.

### Execution-Reconciliation Dual Core.md

Defines the runtime core as two irreducible functions: execution emits candidate intents/outcomes, while reconcile/commit/apply decides authoritative progression. Capabilities can emit intents and bind operations, but canonical state progression remains host-side.

### Extension Mod.md

Defines Extension Mod as a mod attached to another mod-like artifact rather than directly to Engine or Game. Attachment, dependencies, conflicts, visibility, and placement must be explicit Vapor.toml metadata, not implicit folder/code behavior.

### Fingerprint.md

Defines Fingerprint as a deterministic identity/compatibility summary richer than an ID or hash. It may combine manifest structure, hashes, versions, dependency shape, public capability paths, Steam IDs, and Rhai identities for compatibility, diagnostics, caching, publishing, verification, and reproducibility.

### Game Mod.md

Defines a Game Mod as a mod attached to a Game/`base_mod` fixture or Game-facing extension point. Game Mods attach under Gamepack, may affect Game fingerprint/identity, and are part of Phase 3's proof of real Game-side extension.

### Game.md

Defines Game as the mandatory playable/content pillar of a Vapor product instance, represented by singleton `base_mod`. It is required but replaceable as a Game-level concept, must declare Engine/core_mod constraints, and is mediated through Gamepack.

### Gamepack.md

Defines Gamepack as the user/modpack-author-facing object selecting one Game/`base_mod` plus Game-attached contributions. It depends on the selected Enginepack, contributes identity/fingerprint data, and does not make USF a product slot.

### Global Capability API Graph.md

Defines the unrestricted host-authoritative capability/API graph produced by Rust bootstrap, reflection registration, and module loading. It is not script-safe; scripts must use projected facades.

### Global Capability Surface.md

Defines deliberately global utility capability/API surfaces such as math and logging. These are globally scoped by contract but do not grant unrestricted domain-state authority and remain bounded by scope envelopes and runtime policy.

### Kernel Artifact.md

Defines packaged native implementation artifacts that provide Capability Kernels, likely platform-specific dynamic libraries for Phase 3 pressure. The required boundary is a narrow Rust-native registration entrypoint into vapor_core under a locked Vapor Toolchain Envelope; incompatible artifacts fail fast, and C ABI entrypoints are not accepted.

### Loo Cast.md

Defines Loo Cast primarily as the first-party Game built on Spacetime Engine with USF as flagship simulation framework. Informally it may also name the product/project bundle granting access to the game, engine, and Vapor SDK/modding rails, but precise writing should distinguish those layers.

### Managed Ambiguity.md

Defines a practice of treating unresolved design edges as explicit tracked material while preserving hard invariants. It should produce named open questions, ledger entries, TODOs, or phase-bound decisions; it is not permission to leave contradictions invisible.

### Metric.md

Defines Metric as a scale-bound USF capability/data/function surface for measured, derived, or computed state. Metrics include primitive fields and composite computed functions, are public within their owning scale context, inform Phenomenon significance/materialization logic, and may replace some older zone-era data-layer uses.

### Mod Artifact Structure.md

Defines the packaged build-output side of a mod, separating build, distributable, and source/development artifacts. Runtime-deliverable implementation artifacts are redistributable libraries; contract and implementation sources remain development artifacts governed by compatibility envelopes.

### Mod Authoring Structure.md

Defines the source-side layout for mods. Required authored parts include implementation-crate source and contract-crate source even if no-op, with Rhai declarations as canonical authored data/asset definitions and generated media treated as outputs/caches rather than canonical source.

### Mod Contract Source.md

Defines the mod's contract-crate source side. It is a required authored counterpart to implementation source and has a redistributable source packaging form.

### Mod Implementation Source.md

Defines the mod's implementation-crate source side. It is a required authored counterpart to contract source and builds into the redistributable runtime implementation library.

### Mod Runtime Representation.md

Defines the in-memory integrated form of a loaded mod after registration. It resolves locked capabilities such as Scale Definitions, Metrics, Phenomena, and Scale Realizers into active behavior through Modding Runtime, Capability Runtime, and USF Runtime. Open pressure: Modding and Capability are distinct but closely related, and glossary pages that imply a hard separation should be audited.

### Mod Structure.md

Defines the three-stage shape of a mod: authoring structure, artifact structure, and runtime representation. The stages separate source layout, build outputs, and in-memory integration under the Modding Contract.

### Modding Contract.md

Defines rules for mod declaration, lifecycle, dependencies, compatibility, replacement, composition, and integration. It scopes additive-only behavior carefully: core reserved layers are additive-only by default, but composition-time policies may support exclusive replacement, registries, optional providers, and integration apertures before lock. Optional-provider and integration-aperture wording remains underexplored and should not be treated as stable doctrine yet.

### Modding Runtime.md

Defines the runtime layer for mod loading, dependency resolution, registration, validation, lifecycle execution, and Runtime Lock enforcement. It performs staged artifact discovery, projection, shallow and deep validation, and launcher-native failure before `core_engine` starts when composition is invalid.

### Modpack.md

Defines a user/modpack-author-facing composition object for selecting compatible mods and nested modpacks. Modpacks are typed by attachment target, preserve nested boundaries for organization/diagnostics/fingerprints, publish through Workshop, and participate in Packagepack resolution.

### Observer-Relative Simulation.md

Defines view-conditioned detail resolution over the scale system. The active scale is the first-class change-authority scale, higher scales continue through summarized/scaled-time semantics, and lower detail is normally elided except through scoped sampling/simulation/inspection with upward significance overflow. Open pressure: internal implications remain underexplored, including higher-scale significance flow and how larger-scale changes affect lower scales.

### Packagepack.md

Defines the complete launch composition object players launch and modpack authors edit. It selects exactly one Enginepack and exactly one compatible Gamepack, includes selected Modpacks/Engine Mods/Game Mods/Extension Mods, carries resolved fingerprint metadata, and is central to Phase 3 local and Workshop-backed lifecycle proof.

### Phase 3 Vapor Testing Suite.md

Defines the Phase 3 Vapor Testing Suite for proving Vapor composition without USF/worldmodel/gameplay scope. It combines automated validation tests, local/manual scenarios, and manually verified Steam/Workshop flows; it must cover local-only authoring, default and heavily modded packagepacks, extension and nested modpacks, alternative engines/games, offline installed launch, Steam upload/update/download/install flows, and invalid dependency/conflict/fingerprint/Rhai/Vapor.toml/download cases.

### Phenomenon.md

Defines Phenomenon as the primary USF carrier of significant world state, world generation, interactions, and detail materialization. Phenomena replace zones as ontology authority, own significance thresholds and materialization/re-aggregation logic, and use metrics/chunks/cross-scale context without making chunks the primary authority.

### Pillar Dependency Topology.md

Maps the role-level stack as Vapor Ecosystem -> Spacetime Engine -> Loo Cast. USF is a pivotal Spacetime module/framework part, not a separate product pillar; replacing the engine may leave only Vapor-level conventions in common.

### Player-to-Creator Path.md

Describes progression from player use to understanding and authorship across the Vapor product stack. It is the product expression of the project ethos, moving from optimizing gameplay loops toward direct code changes and serious mod development.

### Polycentric Pillars.md

Defines the architecture as intentionally multi-pillar rather than centered on one framework. USF, capability semantics, runtime substrate/orchestration, workflow, and scripting/reflection are co-significant and must remain coherent.

### Portal Traversal Semantics.md

Defines continuity rules for observers/entities moving across non-trivial spatial mappings such as portals or wrapping worlds. It coordinates Scale View, Observer-Relative Simulation, and Entity Proxy behavior to preserve perception and interaction coherence. Entity Plane Split is likely relevant and should be cross-linked or audited with this page.

### Project Artifact Structure.md

Defines project-scope build and distribution artifact classes, separating runtime-deliverable artifacts from development/dependency-channel artifacts. It mirrors mod artifact distinctions without defining runtime composition. This entry remains unstable and needs audit.

### Project Authoring Structure.md

Defines the source-side repository/workspace organization across runtime code, contracts, declarations, tooling, and documentation. It treats `docs/glossary` as a canonical authoring surface and warns that current filesystem layout may be transitional. Open pressure: the expected future split into separate Vapor, Spacetime Engine, and Loo Cast repositories/projects has broad implications for glossary topology and repository setup.

### Project Ethos.md

Defines the project's high-level commitment to science, openness, empathy, curiosity, and empowerment. The Player-to-Creator Path is named as the main product manifestation. The ethos also emphasizes inspiring people to engage with real systemic complexity and deep interconnection instead of simplified false models.

### Project Runtime Representation.md

Defines the active in-memory project form after Runtime Lock, including mod graph, ownership mappings, Capability Instances/channels, workflow state, and runtime substrate state. Startup graph structure is fixed at lock; runtime evolution changes state/intent within that structure through explicit capability, registry, kernel, or substrate policy.

### Project Structure.md

Defines the project-level three-stage shape: authoring structure, artifact structure, and runtime representation. It aligns project organization with Mod Structure and SDK boundaries.

### README.md

Indexes the glossary directory and explains the tag split between `#glossary` and `#tech_glossary`. It lists the current implementation-facing notes, especially Phase 3/Vapor planning, Rhai, USF, runtime, and workflow note pages.

### Redistributable Mod Contract Source.md

Defines the packaged distribution form of Mod Contract Source for build-time/development dependency workflows. It may include source plus docs, tests, or examples and can also appear inside Steam-distributed composites.

### Redistributable Mod Implementation Library.md

Defines the runtime-deliverable implementation library artifact set for a mod. It includes identity/manifest metadata and platform-specific dynamic libraries for supported targets, distinct from source-side artifacts.

### Reserved Built-In Mod Role.md

Defines `core_engine`, `core_mod`, and `base_mod` as reserved structural role names and literal required Rust crate names. They are mandatory but replaceable through valid Engine/Game selection; `core_engine` and `core_mod` are coupled for Phase 3 and cannot be mixed independently.

### Rhai Asset.md

Defines Rhai Asset as the umbrella for typed Rhai source files contributing capability-framework material. Rhai data assets include config, localization, constants, tuning values, authored tables, and similar structured payloads. Rhai declaration assets may carry data, metadata, graph meaning, type/trait/callback declarations, and sanctioned callbacks; one Rhai file no longer implies one Capability Declaration.

### Rhai Capability.md

Defines Rhai Capability as a declaration-level, dynamic, human-readable API object exposed through projected `ctx` subgraphs. Scripts define data, declarations, parameters, policy data/rules, and callbacks; callbacks are the sanctioned declaration-level logic surface, while Rust-owned host/runtime systems own native execution, state authority, scheduling, and projection of native capabilities into Rhai contexts.

### Runtime Lock.md

Defines the boundary where validated launch composition becomes an immutable startup graph core before Engine/Game execution. It requires fixed ownership, fixed callback masks, fixed active mod graph, converged cycle-free bootstrap, and forbids post-lock graph mutation by default while still allowing explicitly governed runtime substrate state.

### Runtime Substrate.md

Defines the Spacetime Engine execution substrate for scale-layered simulation and capability-driven runtime behavior. ECS is the execution/data medium, contracts/capabilities define semantic authority, and the substrate owns the mutable inner-inner runtime graph: state, registries, queues, resources, IO handles, and nondeterministic effects allowed after Runtime Lock.

### Rust Host Contract.md

Defines the Rust-defined contract surface that validates, projects, materializes, schedules, binds, or executes capability-side type, trait, callback, extension-slot, data, or native surfaces.

### Rust Surface Graph.md

Defines the Rust-native registry/graph of scriptable host functionality known to Vapor. It is expected to be coordinated by vapor_core and projected into the capability/Rhai side through Rust Host Contracts.

### SDK.md

Defines the SDK as the full creator-facing Vapor toolchain, not just a library. It must support scaffolding, validation, linting, packaging, publishing, migration, docs, launcher/CLI/Rust tooling surfaces, and eventually the Vapor Toolchain Envelope for native Kernel Artifact builds, including vendored canonical toolchains or Rust meta-toolchain launcher behavior if needed.

### Scale Contract.md

Defines declaration/compatibility rules for Scale Definition, Scale Support, and Scale Realizer. Each of the 71 canonical scale coordinates needs one scale definition and one declared realizer type, and every capability-scale pair must explicitly declare supported/unsupported.

### Scale Definition.md

Defines what is semantically meaningful at one Scale coordinate. It lists recognized capability, metric, phenomenon, and realizer types for that coordinate under the Scale Contract.

### Scale Realizer Cardinality.md

Defines the activation invariant that each active Scale Slice must resolve to exactly one effective Scale Realizer. Missing or multiple realizers for a slice are invalid.

### Scale Realizer.md

Defines the semantic realization capability bound to one Scale Slice, formerly called a phenomena realizer. The term is useful but under pressure because Phenomena likely own much of the concrete detail materialization and re-aggregation logic.

### Scale Slice.md

Defines the runtime realization of one Scale coordinate inside the USF Instance Graph. Slices may be simulated in parallel and composed through capability outputs, and each carries one effective Scale Realizer binding. It is evidence for considering simple capability input/output roles, but not enough to settle that split.

### Scale Support.md

Defines the support state of a capability at a specific Scale coordinate. Each capability-scale pair has exactly one explicit state: supported or unsupported. Open pressure: support declarations may need a more general framing than capability-scale pairs, because capabilities can be syntax around deeper semantics.

### Scale View.md

Defines observer-relative projection and traversal state over scale coordinates. It is not camera/rendering/chunk-streaming itself but rather a substrate used by those things down the road, and for pre-alpha assumes one primary observer while treating active scale as change-authority, higher scales as summarized/scaled-time, and lower scales as sampled/scoped.

### Scale.md

Defines Scale as the canonical semantic coordinate type in USF. It identifies where simulation meaning is anchored but does not itself declare support, runtime realization, or observer projection behavior.

### Scaled Capability Channel.md

Defines a USF-compatible, scale-scoped execution face of a capability implementation. It is not part of the general definition of Capability; it is one way for a capability to participate in USF scale semantics through Scale Support and active Scale Slice context.

### Script Safety.md

Defines the invariant that scripts cannot access the unrestricted global capability/API graph. Scripts use projected facades, whitelist-oriented policy, data/declaration assets, sanctioned callbacks/hooks/messages, and host-backed capabilities rather than owning scheduling, native integration, IO, simulation kernels, or authoritative mutation.

### Scripting Projection Meta-Layer.md

Legacy bridge name now merged into Capability Projection API. The preserved signal is that declaration `ctx` and callback `ctx` are distinct projected contexts, and reflection metadata plus host orchestration remain useful implementation mechanisms.

### Scriptable Rust Surface.md

Defines selected Rust-native functionality made visible to the Capability Framework through explicit registration metadata and host contracts. Owned types may implement Scriptable directly; external crates normally need wrappers, adapters, or descriptors.

### Slot Graph Composition.md

Defines composition-time ownership and extension as parent-owned Capability Extension Slots filled by candidates whose Capability Type Signatures satisfy required Capability Trait bounds and slot policy. It explains mandatory Engine/Game pillars, forbids cycles/self-slots, and recommends modeling runtime dynamism through explicit capabilities rather than post-lock slot mutation.

### Source Artifact.md

Defines Source Artifact as raw authoring-side source: files, folders, Rhai declarations, Rust source, Vapor.toml files, and other inputs before build/packaging.

### Spacetime Engine.md

Defines the first-party Engine product providing Runtime Substrate and Capability Runtime for default and modified experiences. It owns USF as a public/API-facing subsystem and is represented by the `core_engine` plus matching `core_mod` reserved-role pair. Open pressure: Spacetime Engine should extend/use Capability Runtime, not own the concept.

### Steam Workshop.md

Defines Steam Workshop as the mandatory public Phase 3 distribution surface for Vapor mods, modpacks, and related artifacts. Vapor must support upload/update, private/unlisted tests, download/install/update, enable/disable/uninstall, dependency detection, fingerprint verification, prompted auto-subscribe, and offline installed behavior.

### Steam.md

Defines Steam as the exclusive external identity, ownership, authorization, and distribution substrate for Phase 3 Vapor. Public usage requires Steam identity/ownership rails, while local/offline authoring and already-installed offline play/test can remain possible when Steam is unavailable.

### USF Contract.md

Defines the USF Contract Family as public/API-facing simulation contract structure inside Spacetime Engine. It covers Scale, Scale Definition, Scale Support, Scale Realizer, and Scale Contract while staying separate from Vapor-level product contracts.

### USF Definition Lifecycle.md

Defines pre-runtime establishment and validation of singleton-like Capability Declarations through iterative fixed-point bootstrap. After validation and Runtime Lock, declarations become Capabilities, active USF runtime capabilities materialize, and definition mutation is no longer part of active runtime, at least for the immutable startup-constructed core.

### USF Instance Graph.md

Defines the active structured set of runtime-materialized USF capabilities, organized as a 71-scale stack of Scale Slices. It enforces singleton Scale/Scale Realizer per occupied scale slot and many-per-scale Phenomenon/Metric collections with at least one required per scale.

### USF Instantiation Scripts.md

Defines declaration-centric Rhai authoring for USF-specific Capability Declaration / Capability Node material. Scripts run with type/trait/callback-tailored `ctx`, emit structured data plus sanctioned callback closures, separate declaration and callback scopes, and feed runtime materialization into the USF Instance Graph.

### USF Runtime Evolution Lifecycle.md

Defines post-lock runtime progression for USF. Materialized capabilities execute sanctioned callback logic through projected contexts, dynamic policy can narrow/re-open callback paths within the envelope, intents are reconciled/committed/applied, and ECS remains substrate rather than capability authority.

### USF Runtime.md

Defines the Spacetime Engine runtime implementation of USF Contract behavior. It invokes declaration entrypoints during activation, materializes capabilities from lock-established declarations, executes runtime behavior and Rhai callbacks through resolved masks, and composes with Capability Runtime, Modding Runtime, and Workflow Framework.

### USF.md

Defines USF as the flagship first-party public/API-facing simulation subsystem inside Spacetime Engine. It is not a standalone Vapor product or directly replaceable capability, though its APIs are public and it is most of the practical Spacetime world/model surface used by Loo Cast.

### Vapor Crate Topology.md

Documents current crate/workspace direction for Vapor: likely public/open-source early, split into crates such as `vapor_core`, `vapor_sdk`, `vapor_launcher`, `vapor_steam`, and optionally `vapor_macros`. Phase 3 requires the core/sdk/launcher/steam split unless renamed by owner approval.

### Vapor Ecosystem.md

Defines Vapor as the Steam-exclusive ecosystem/product layer over engines, games, mods, packs, SDK, launcher, capability substrate, Rhai authoring, validation, identity, and Workshop distribution. It is broader than Loo Cast but currently available through the Loo Cast product bundle; Phase 3 proves tooling/composition/distribution, not USF/gameplay systems.

### Vapor Launcher.md

Defines the launcher as the user/modpack-author/developer UI for browsing, composing, validating, installing, launching, and diagnosing Vapor content. It requires Player, Modpack Author, and Developer modes, separate launcher/runtime logs, install ledger, Steam/Workshop actions, and real Packagepack launch in Phase 3.

### Vapor Product Instance Stack.md

Defines a concrete selected launch/composition stack. It must resolve `core_engine`, matching `core_mod`, and `base_mod`; those roles are required but replaceable through valid Engine/Game selection, with Packagepack mediating Enginepack, Gamepack, and Modpack contributions.

### Vapor Product Stack.md

Defines the high-level product/composition relationship: Vapor Ecosystem provides SDK/launcher/distribution/capability/Rhai substrate; Engine and Game are generic roles; a launch selects one of each; Packagepack brings Enginepack, Gamepack, Modpacks, and mods together.

### Vapor.lock.md

Defines Vapor.lock as the resolved dependency, fingerprint, hash-state, and future kernel/toolchain compatibility counterpart to Vapor.toml. It should eventually record Kernel Artifact identity, platform target, toolchain envelope, exported registration metadata, native dependency fingerprints, and compatibility proof.

### Vapor.toml.md

Defines Vapor.toml as the required manifest surface for every Vapor artifact root and Capability Module/Node source folder. It classifies typed Rhai files, declares kernel imports/native surface requirements, and carries dependencies, conflicts, visibility, target roles, placement/storage metadata, pack composition, and Steam/Workshop publication metadata.

### Vapor Toolchain Envelope.md

Defines the locked Rust/cargo/platform/toolchain compatibility envelope needed for native Kernel Artifact builds and runtime loading. It must make Rust-native registration entrypoints compatible enough to use; if not, loading fails fast rather than proving only a trivial C ABI import boundary.

### Workflow Framework.md

Defines the Workflow Framework as the Rust-side orchestration layer for staged runtime work in the Runtime Substrate. It coordinates requests, progression, completion/failure, domain stages, and Capability Instance/runtime values rather than raw Rhai engine internals.

### Zone-Era Concepts.md

Preserves older zone/DPT/ZLM/ZTM ideas as signal-bearing but no longer primary ontology authority. The core correction is that Phenomena can own localized/distributed/region-like significance, while zone-like tools may remain intermediate classification or realization-selection mechanisms.

### steam-like-platform-contracts.md

Defines a far-future possible abstraction for non-Steam platforms with Steam-like identity, ownership, Workshop-style distribution, and publishing contracts. It is explicitly not Phase 3 scope and must not weaken the current Steam-exclusive implementation path.
