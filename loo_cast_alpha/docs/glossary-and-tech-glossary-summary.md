# Glossary and Tech Glossary Summary

Source scope: the original 136 Markdown pages in `loo_cast_alpha/docs/glossary/`. Obsidian workspace metadata under `.obsidian/` is editor state and is not summarized here.

The glossary divides terms by tag: `#glossary` pages are concept vocabulary, while `#tech_glossary` pages are implementation-facing notes. `README.md` is the index for this split and carries both concerns.

## Mission-Level Synthesis

The central architecture is a capability-first Vapor ecosystem. A Capability is not just an API function or artifact label; it is the common graph primitive for identity, authority, projection, composition, validation, runtime execution faces, and diagnostics. Artifact and pack terms exist to describe source/build/distribution/composition containers, but the runtime truth is the resolved capability graph after Runtime Lock.

Vapor is currently Steam-exclusive. Phase 3 is a proof of SDK, launcher, Steam/Workshop flows, packagepack composition, capability/Rhai declaration loading, diagnostics, fingerprints, and hello-world-style engine/game fixtures. Phase 3 deliberately does not prove USF worldmodel, gameplay, rendering, save/load, or full simulation behavior.

The Spacetime Engine and Loo Cast are first-party instances inside Vapor. The Engine role is a coupled `core_engine` plus matching `core_mod`; the Game role is `base_mod`. USF is a public/API-facing Spacetime Engine subsystem, not a standalone Vapor product layer.

Rhai is the canonical declaration and callback authoring surface, but not the raw runtime authority surface. Scripts emit data-first Capability Declarations and sanctioned callbacks through projected `ctx` facades. Rust owns host registration, materialization, scheduling, heavy kernels, mutation authority, validation, and reconcile/commit/apply.

Runtime composition is closed after Runtime Lock. Design remains open between lock cycles, but post-lock graph mutation is forbidden by default. Runtime dynamism should be expressed through explicit capabilities, registries, or policies, not arbitrary structural mutation.

The workflow notes are legacy implementation evidence for Rust-side orchestration. They document typed workflow requests, domain-specific stage buffers, Bevy-visible stage systems, composite wrappers, timeout behavior, and known refactor hazards such as placeholder stage slots and generated `transmute` handoffs.

## Every Page

### Artifact Compatibility Envelope.md

Defines the rule boundary for when source, build, and redistributable development artifacts can be reused versus rebuilt. It treats source as the canonical sharable artifact class and allows cached redistributable implementation-library builds per compatible target bucket, while requiring source archival so artifacts can always be rebuilt without external repository availability.

### Artifact.md

Defines Artifact as a concrete file, folder, archive, build output, distribution object, or installed/downloaded object in the Vapor ecosystem. It explicitly excludes in-memory Capabilities and splits precision terms into Source Artifact, Build Artifact, Distributable Artifact, Workshop item, Packagepack, Enginepack, Gamepack, and Modpack.

### Asymmetric Failure Doctrine.md

States the failure posture: runtime integrity violations should fail visibly and quickly, while persistence-sensitive paths such as save/load need stronger recovery and corruption-avoidance policy. Startup invalidity should fail launch cleanly through launcher diagnostics rather than crashing the launcher, and persistence safety should rely on backups/autosaves plus hard failure when safety is uncertain.

### Build Artifact.md

Defines Build Artifact as built output from source that has not yet been assembled, packaged, or published as a final distributable. It is distinct from Distributable Artifact and from in-memory Capability objects.

### Callback Context Type.md

Defines the dedicated `ctx` type projected into a Callback Type. Each meaningful callback should have its own projected context, scoped to allowed capabilities/data/operations and distinct from both the function shape and signature metadata.

### Callback Scope Envelope.md

Defines the concrete capability/API projection envelope used for callback invocation contexts. It is a specific instance of the broader Capability Graph Scope Envelope.

### Callback Signature.md

Defines metadata that identifies a callback context and invocation shape without requiring the fully resolved capability graph. It is fingerprint/ID-like, supports compatibility checks and diagnostics, and remains separate from executable callback logic.

### Callback Type.md

Defines a typed callback function shape in Vapor/Rhai terms. It describes the kind of callback entrypoint, may encode firing policy such as single-fire or multi-fire, has a dedicated Callback Context Type, and is not the same as Capability Slot Type.

### Capability Bootstrap Fixed-Point Cycle.md

Defines the deterministic iterative startup process that materializes resolvable capability/API layers. First-order declarations are root-level and cannot depend on other capabilities; dependency cycles are invalid.

### Capability Contract.md

Defines the Capability Contract Family as the rules capabilities must declare and satisfy across compatibility, paths, projection, scaled channels, and runtime coordination. It separates provider dependencies from declaration `ctx` dependencies, requires Vapor-readable metadata before lock, and treats USF as a user of the capability model rather than its foundation.

### Capability Declaration.md

Defines the pre-lock authored payload for a Capability. It is data-first, shaped by a Capability Slot Type/Rust host contract, may include declared behavior callbacks, and is promoted into a Capability only after validation and Runtime Lock transition.

### Capability Dependency Layer Notes.md

Documents the key phase split: Rhai declaration and Rust runtime execution coexist but remain separate. It distinguishes mod/provider dependencies, declaration-time `ctx` path requirements, and post-lock runtime interactions; declaration contexts and callback contexts get separate effective access masks that hard-fail if violated.

### Capability Graph Diagnostics.md

Defines diagnostics produced from invalid or suspicious capability/slot graph states. It recommends classifying errors by graph primitive first, then projecting them into player, modpack-author, and developer views; Phase 3 should prove diagnostics for conflicts, cycles, missing providers, bad Rhai declarations, fingerprint mismatches, corrupted downloads, and visibility violations.

### Capability Graph Scope Envelope.md

Defines the hard maximum API/context scope allowed for a profile-bound script or callback. Runtime policy may narrow or re-open paths within this envelope, but can never widen access beyond it.

### Capability Path.md

Defines a stable addressing route inside a capability/API graph or projection facade. Paths are used for policy allow/deny decisions and should not imply dependency, causality, execution flow, or folder-nesting defaults by themselves.

### Capability Projection API.md

Defines script-safe or callback-safe contextual facades projected from the capability/API graph. These lightweight Rhai-facing handles include declaration `ctx` and callback `ctx`, and are intentionally not raw access to the unrestricted host graph.

### Capability Resolution Semantics.md

Defines resolution as layered, not singular: dependency graph bootstrap, materialization/merge into the host graph, and projection/access policy are separate. Required unresolved dependencies, cycles, invalid multiplicity, or denied required callback paths hard-fail; no implicit fallback owner appears.

### Capability Role and State Authority Notes.md

Records that the current input/output role taxonomy is underpowered. Possible future role surfaces include authority, reconciler, realizer, bridge, and mutator, but canonical mutation authority still needs pressure-testing across capability runtime, USF runtime, authority capabilities, and reconcile/commit/apply.

### Capability Role.md

Defines Capability Role as an unresolved term for how a capability participates in graph semantics. It warns that input/output wording is incomplete and that mutation authority belongs outside capability objects until the runtime authority pipeline is modeled explicitly.

### Capability Runtime.md

Defines the runtime orchestration layer for capability discovery, registration, validation, coordination, and execution routing. It builds and locks a resolved launched Engine/Game graph through staged discovery/validation, executes materialized capabilities through projected contexts and scaled channels, and keeps canonical mutation decisions in host reconcile/commit/apply paths.

### Capability Slot Type.md

Defines the projected/gated slot/context shape formerly described with profile/type-template wording. It describes what kind of capability surface can be attached, requested, validated, or exposed, while cardinality remains a separate slot policy and callbacks remain separate concepts.

### Capability-Centric Semantics.md

Defines the doctrine that project meaning and authority are modeled through capabilities first, while execution still uses lower-level runtime substrate primitives. Global utility surfaces such as math/logging can exist, but only as explicit bounded capability surfaces.

### Capability.md

Defines Capability as Vapor's core runtime/contract graph primitive spanning ability, authority, API exposure, composition, validation, and orchestration. It sets identity/visibility rules, staged graph construction, Rust/Rhai declaration and callback flow, host-owned execution authority, composite capability semantics, USF boundary, and Phase 3 requirements for stable paths, projection, validation, lock, and diagnostics.

### Chunk.md

Defines Chunk as the first-level USF spatial partition at a canonical Scale, fixed at `1000^3` scale-local units per scale. Chunks can act as address/cache/generation/simulation/realization boundaries, but significant world authority should live in Phenomena, with chunk state serving residual, local, cache, temporary, or metric-oriented roles unless explicitly promoted.

### Closed Runtime and Open Design.md

Defines the principle that runtime activation is deterministic and bounded once locked, while design and contracts can evolve between lock cycles. Structural changes should pass through explicit lifecycle transitions rather than hidden hot mutation.

### Complexity Gradient.md

Defines a continuous spectrum from automated low-complexity interaction to direct high-complexity system control. It supports the Player-to-Creator Path by exposing deeper mechanisms through wrappers over the same underlying systems rather than separate modes of reality.

### Composite Workflow Runtime Notes.md

Documents composite workflows generated by `composite_workflow!` as Rust-side wrappers coordinating workflow invocations and local orchestration inside Tokio tasks. It covers capture semantics, fallible-mode selection, context creation/cleanup, and immediate or deferred return handling.

### Contract Family.md

Defines a Contract Family as a coherent group of related contract definitions governing one semantic concern. Examples are capability, modding, and USF families, each with declaration/validation boundaries and runtime counterparts.

### Contract.md

Defines Contract as the umbrella over contract families across Vapor, Spacetime Engine, and Loo Cast. It is not one mechanism; each family must have supporting runtime/backends, and USF remains an internal Spacetime subsystem contract rather than a standalone Vapor product contract.

### Declaration Scope Envelope.md

Defines the concrete capability/API projection envelope applied to declaration entrypoint execution. It is a sibling concrete envelope to Callback Scope Envelope under the broader Capability Graph Scope Envelope.

### Distributable Artifact.md

Defines Distributable Artifact as the final packaged object produced by Vapor tooling for upload/distribution, with Steam Workshop as the Phase 3 public target. Such artifacts may reference dependencies instead of embedding everything, while local resolved forms may contain needed dependency contents.

### Dynamic Authority Resolution.md

Defines authority as lifecycle- and operation-relative rather than one global static fact. Runtime policy can narrow or re-open access within the Capability Graph Scope Envelope, while definition, runtime, and output/application authority remain distinct.

### Engine Mod.md

Defines an Engine Mod as a mod attached to an Engine fixture or Engine-facing extension point. Engine Mods attach under Enginepack, may affect Engine fingerprint/identity, and are required in Phase 3 to prove real Engine-side extension.

### Engine.md

Defines Engine as a mandatory Vapor product/composition role represented by a coupled `core_engine` plus matching `core_mod`. Non-Spacetime engines may exist and may ignore much of the recommended capability model after bootstrap, but the first-party docs should not use that breadth to make USF look like a product-level slot.

### Enginepack.md

Defines Enginepack as the user/modpack-author-facing object selecting one Engine fixture plus Engine-attached contributions. It must select a coupled `core_engine`/`core_mod` pair, contribute downstream compatibility/fingerprints, and prove default, alternative, and modded Engine fixtures in Phase 3.

### Entity Plane Split.md

Defines a modeling pattern where one conceptual entity is represented across backend and frontend planes. Backend owns interaction/state authority; frontend owns visible presentation, enabling projection tricks, scale-relative visibility, portals, and wrapping without collapsing authority boundaries.

### Entity Proxy.md

Defines a coupled proxy representation of another entity for behavior, visibility, or both. Proxy modes may be backend, frontend, or dual-plane and compose with observer-relative simulation and portal traversal.

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

### Loo Cast.md

Defines Loo Cast primarily as the first-party Game built on Spacetime Engine with USF as flagship simulation framework. Informally it may also name the product/project bundle granting access to the game, engine, and Vapor SDK/modding rails, but precise writing should distinguish those layers.

### Managed Ambiguity.md

Defines a practice of treating unresolved design edges as explicit tracked material while preserving hard invariants. It avoids both premature over-locking and unstructured drift.

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

Defines the in-memory integrated form of a loaded mod after registration. It resolves locked capabilities such as Scale Definitions, Metrics, Phenomena, and Scale Realizers into active behavior through Modding Runtime, Capability Runtime, and USF Runtime.

### Mod Structure.md

Defines the three-stage shape of a mod: authoring structure, artifact structure, and runtime representation. The stages separate source layout, build outputs, and in-memory integration under the Modding Contract.

### Modding Contract.md

Defines rules for mod declaration, lifecycle, dependencies, compatibility, replacement, composition, and integration. It scopes additive-only behavior carefully: core reserved layers are additive-only by default, but composition-time policies may support exclusive replacement, registries, optional providers, and integration apertures before lock.

### Modding Runtime.md

Defines the runtime layer for mod loading, dependency resolution, registration, validation, lifecycle execution, and Runtime Lock enforcement. It performs staged artifact discovery, projection, shallow and deep validation, and launcher-native failure before `core_engine` starts when composition is invalid.

### Modpack.md

Defines a user/modpack-author-facing composition object for selecting compatible mods and nested modpacks. Modpacks are typed by attachment target, preserve nested boundaries for organization/diagnostics/fingerprints, publish through Workshop, and participate in Packagepack resolution.

### Normal Workflow Runtime Notes.md

Defines a normal workflow as a typed `run_workflow_*` request handled by the workflow runtime, not a composite wrapper. It covers request relay, WorkflowInstance insertion, stage-buffer routing, stage polling, typed responses, and the current single-active-instance gate per module/workflow key.

### Observer-Relative Simulation.md

Defines view-conditioned detail resolution over the scale system. The active scale is the first-class change-authority scale, higher scales continue through summarized/scaled-time semantics, and lower detail is normally elided except through scoped sampling/simulation/inspection with upward significance overflow.

### Packagepack.md

Defines the complete launch composition object players launch and modpack authors edit. It selects exactly one Enginepack and exactly one compatible Gamepack, includes selected Modpacks/Engine Mods/Game Mods/Extension Mods, carries resolved fingerprint metadata, and is central to Phase 3 local and Workshop-backed lifecycle proof.

### Phase 3 Vapor Scenario Suite.md

Defines the scenario/integration-test suite for proving Vapor composition without USF/worldmodel/gameplay scope. It must cover local-only authoring, default and heavily modded packagepacks, extension and nested modpacks, alternative engines/games, offline installed launch, Steam upload/update/download/install flows, and invalid dependency/conflict/fingerprint/Rhai/Vapor.toml/download cases.

### Phenomenon.md

Defines Phenomenon as the primary USF carrier of significant world state, world generation, interactions, and detail materialization. Phenomena replace zones as ontology authority, own significance thresholds and materialization/re-aggregation logic, and use metrics/chunks/cross-scale context without making chunks the primary authority.

### Pillar Dependency Topology.md

Maps the role-level stack as Vapor Ecosystem -> Spacetime Engine -> Loo Cast. USF is a pivotal Spacetime module/framework part, not a separate product pillar; replacing the engine may leave only Vapor-level conventions in common.

### Player-to-Creator Path.md

Describes progression from player use to understanding and authorship across the Vapor product stack. It is the product expression of the project ethos, moving from optimizing gameplay loops toward direct code changes and serious mod development.

### Polycentric Pillars.md

Defines the architecture as intentionally multi-pillar rather than centered on one framework. USF, capability semantics, runtime substrate/orchestration, workflow, and scripting/reflection are co-significant and must remain coherent.

### Portal Traversal Semantics.md

Defines continuity rules for observers/entities moving across non-trivial spatial mappings such as portals or wrapping worlds. It coordinates Scale View, Observer-Relative Simulation, and Entity Proxy behavior to preserve perception and interaction coherence.

### Project Artifact Structure.md

Defines project-scope build and distribution artifact classes, separating runtime-deliverable artifacts from development/dependency-channel artifacts. It mirrors mod artifact distinctions without defining runtime composition.

### Project Authoring Structure.md

Defines the source-side repository/workspace organization across runtime code, contracts, declarations, tooling, and documentation. It treats `docs/glossary` as a canonical authoring surface and warns that current filesystem layout may be transitional.

### Project Ethos.md

Defines the project's high-level commitment to science, openness, empathy, curiosity, and empowerment. The Player-to-Creator Path is named as the main product manifestation.

### Project Runtime Representation.md

Defines the active in-memory project form after Runtime Lock, including mod graph, ownership mappings, materialized capabilities/channels, workflow state, and runtime substrate state. Structure is fixed at lock; runtime evolution changes state/intent within that structure.

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

Defines Rhai declaration files as canonical authored asset/capability declaration surfaces. One file should normally define one authored leaf Capability Declaration; Vapor.toml carries manifest metadata; generated media is output/cache rather than canonical source; Phase 3 must load/validate declarations and prove one focused callback path.

### Rhai Bridge Domains and Access Provider Notes.md

Documents bridge-domain and access-provider findings from legacy/quarantine code. Bridge modules mirror domains, reflection metadata and dispatch catalogs define surfacing, and `AccessCellProvider` patterns create scoped access windows that align with projection-based script safety but remain partly provisional.

### Rhai Capability.md

Defines Rhai Capability as a declaration-level, dynamic, human-readable API object exposed through projected `ctx` subgraphs. Scripts define declarations, parameters, policy logic, and callbacks, while Rust owns scheduling, heavy kernels, state authority, and host-side execution.

### Rhai Generic Dispatch Policy Notes.md

Documents how Rhai should handle generic-like behavior without runtime Rust monomorphization. It favors explicit dispatch registries/catalogs, deterministic signature/type IDs, panic-fast duplicate/missing registration, declaration-first semantics, one script per Capability Declaration, projected `ctx` graphs, and keeping facade/bridge layers thin over monomorphized Rust-safe surfaces.

### Rhai Reflection Macro Surface Notes.md

Documents the macro/reflection registration surface as a strong but non-exclusive architecture signal. `reflect_extern_*`, `#[reflect_*]`, and marker attributes generate inventory metadata, build a deterministic RuntimeBindingGraph, hard-fail duplicate/missing critical pairs, register modules, and support strict script alias preprocessing.

### Rhai Value Semantics and AccessCell Notes.md

Documents provisional but high-signal value/access semantics: Clone, Owned, Ref, Mut, Scoped* modes, AccessCell state transitions, explicit read/write lifecycle, bounded contention, and panic-fast stale/invalid access. It leaves open whether final Rhai bridging should stay thin or preserve a richer value-semantics model.

### Runtime Intent Reconcile Commit Apply Mapping Notes.md

Documents target runtime lifecycle semantics: after definition/freeze, ticks follow emit -> route -> batch -> reconcile -> evaluate -> commit/apply. It treats legacy runtime code and reflection macros as evidence, not final authority, and keeps capabilities as intent emitters/request relays rather than canonical state owners.

### Runtime Lock.md

Defines the boundary where validated launch composition becomes immutable runtime state before Engine/Game execution. It requires fixed ownership, fixed callback masks, fixed active mod graph, converged cycle-free bootstrap, and forbids post-lock graph mutation by default.

### Runtime Substrate.md

Defines the Spacetime Engine execution substrate for scale-layered simulation and capability-driven runtime behavior. ECS is the execution/data medium, while contracts/capabilities define semantic authority, and deterministic activation is enforced through Runtime Lock.

### SDK.md

Defines the SDK as the full creator-facing Vapor toolchain, not just a library. It must support scaffolding, validation, linting, packaging, fingerprinting, publishing, updating, migration, docs, launcher/CLI/Rust tooling surfaces, and Phase 3 public command families over shared `vapor_core`.

### Scale Contract Runtime Notes.md

Summarizes runtime assumptions for the Scale Contract: canonical -35..35 scale spine, one definition and realizer type per coordinate, explicit support per capability-scale pair, one effective realizer per active slice, and explicit cross-scale math conversion/policy boundaries.

### Scale Contract.md

Defines declaration/compatibility rules for Scale Definition, Scale Support, and Scale Realizer. Each of the 71 canonical scale coordinates needs one scale definition and one declared realizer type, and every capability-scale pair must explicitly declare supported/unsupported.

### Scale Definition.md

Defines what is semantically meaningful at one Scale coordinate. It lists recognized capability, metric, phenomenon, and realizer types for that coordinate under the Scale Contract.

### Scale Realizer Cardinality.md

Defines the activation invariant that each active Scale Slice must resolve to exactly one effective Scale Realizer. Missing or multiple realizers for a slice are invalid.

### Scale Realizer.md

Defines the semantic realization capability bound to one Scale Slice, formerly called a phenomena realizer. The term is useful but under pressure because Phenomena likely own much of the concrete detail materialization and re-aggregation logic.

### Scale Slice.md

Defines the runtime realization of one Scale coordinate inside the USF Instance Graph. Slices may be simulated in parallel and composed through capability outputs, and each carries one effective Scale Realizer binding.

### Scale Support.md

Defines the support state of a capability at a specific Scale coordinate. Each capability-scale pair has exactly one explicit state: supported or unsupported.

### Scale View.md

Defines observer-relative projection and traversal state over scale coordinates. It is not camera/rendering/chunk-streaming itself, and for pre-alpha assumes one primary observer while treating active scale as change-authority, higher scales as summarized/scaled-time, and lower scales as sampled/scoped.

### Scale.md

Defines Scale as the canonical semantic coordinate type in USF. It identifies where simulation meaning is anchored but does not itself declare support, runtime realization, or observer projection behavior.

### Scaled Capability Channel.md

Defines a scale-scoped execution face of a capability implementation. Channel availability follows Scale Support, execution binds to active Scale Slice context, and scripts reach relevant capability objects only through projected `ctx` subgraphs.

### Script Safety.md

Defines the invariant that scripts cannot access the unrestricted global capability/API graph. Scripts use projected facades, whitelist-oriented policy, sanctioned declarations/callbacks/hooks/messages, and host-backed capabilities rather than owning scheduling or heavy runtime kernels.

### Scripting Projection Meta-Layer.md

Defines the layer that maps declaration and callback contexts into projected capability/API facades. It keeps contexts co-equal rather than parent/child and treats reflection metadata plus host orchestration as key implementation mechanisms.

### Slot Graph Composition.md

Defines composition-time ownership and extension as parent-owned slots filled by capabilities under type/cardinality/policy rules. It explains how reserved Engine/Game pillars are mandatory but replaceable before lock, forbids cycles/self-slots, and recommends modeling runtime dynamism through explicit capabilities rather than post-lock slot mutation.

### Source Artifact.md

Defines Source Artifact as raw authoring-side source: files, folders, Rhai declarations, Rust source, Vapor.toml files, and other inputs before build/packaging.

### Spacetime Engine.md

Defines the first-party Engine product providing Runtime Substrate and Capability Runtime for default and modified experiences. It owns USF as a public/API-facing subsystem and is represented by the `core_engine` plus matching `core_mod` reserved-role pair.

### Stage Buffer Runtime Notes.md

Documents per-domain workflow stage queues: ECS, Render, Async, EcsWhile, and RenderWhile buffers. Entries hold module/workflow/stage identity plus stage object and optional data; poll systems currently process one entry per run, creating deterministic but potentially backlogged progression.

### Stage Sender Cache Runtime Notes.md

Documents runtime maps from workflow stage metadata to concrete buffer-message sender objects. The caches are built from `register_workflow_mods!` metadata and decouple orchestration systems from direct stage type/module paths.

### Steam Workshop.md

Defines Steam Workshop as the mandatory public Phase 3 distribution surface for Vapor mods, modpacks, and related artifacts. Vapor must support upload/update, private/unlisted tests, download/install/update, enable/disable/uninstall, dependency detection, fingerprint verification, prompted auto-subscribe, and offline installed behavior.

### Steam.md

Defines Steam as the exclusive external identity, ownership, authorization, and distribution substrate for Phase 3 Vapor. Public usage requires Steam identity/ownership rails, while local/offline authoring and already-installed offline play/test can remain possible when Steam is unavailable.

### USF Contract Runtime Boundary Notes.md

Clarifies USF's runtime boundary: `core_engine` is the composition/runtime host, USF is a public Spacetime module/framework part, not a standalone product layer, and replacing USF effectively means creating another Engine. It also phase-tags declaration seams separately from post-lock execution seams.

### USF Contract.md

Defines the USF Contract Family as public/API-facing simulation contract structure inside Spacetime Engine. It covers Scale, Scale Definition, Scale Support, Scale Realizer, and Scale Contract while staying separate from Vapor-level product contracts.

### USF Definition Lifecycle.md

Defines pre-runtime establishment and validation of singleton-like Capability Declarations through iterative fixed-point bootstrap. After validation and Runtime Lock, declarations become Capabilities, active USF runtime capabilities materialize, and definition mutation is no longer part of active runtime.

### USF Instance Graph.md

Defines the active structured set of runtime-materialized USF capabilities, organized as a 71-scale stack of Scale Slices. It enforces singleton Scale/Scale Realizer per occupied scale slot and many-per-scale Phenomenon/Metric collections with at least one required per scale.

### USF Instantiation Capability Slot Notes.md

Documents the declaration/profile model for USF instantiation scripts: profiles such as scale, metric, phenomenon, and phenomenon_realizer map to Capability Slot Types; one script emits one Capability Declaration; access is via include/exclude-filtered `ctx`; callbacks use separate masks; and runtime materializes capabilities after lock.

### USF Instantiation Scripts.md

Defines declaration-centric Rhai authoring for singleton-like USF Capability Declarations. Scripts run with profile-tailored `ctx`, emit structured data plus logic closures, separate declaration and callback scopes, and feed runtime materialization into the USF Instance Graph.

### USF Math Raw Model Foundation Notes.md

Treats the temporary USF math raw model as the highest-authority alpha-era math foundation. It emphasizes facade-first contracts, explicit OpMode/OpPolicy, mixed-representation unions, Core/Field/Bridge operations, explicit field mutability/lock states, shape/domain constraints, cross-scale math taxonomy, and panic-contract validation.

### USF Position Stack and Overflow Policy Notes.md

Summarizes reusable position-stack semantics: recursive GridVec scale chain plus optional SubgridXyz and UnitVec offsets, balanced local digits, normalized unit offsets with carry, explicit wrap/checked/strict overflow policies, and zoom transforms with root-boundary guards.

### USF Runtime Evolution Lifecycle Notes.md

Documents target runtime execution after Runtime Lock: active materialized capabilities execute through hierarchical `ctx` subgraphs, callbacks enforce resolved masks, and runtime progression follows intent/reconcile/commit/apply. It notes current alpha is spec-first and uses legacy code as evidence.

### USF Runtime Evolution Lifecycle.md

Defines post-lock runtime progression for USF. Materialized capabilities execute callback/closure logic through projected contexts, dynamic policy can narrow/re-open callback paths within the envelope, intents are reconciled/committed/applied, and ECS remains substrate rather than capability authority.

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

Defines Vapor.lock as the resolved dependency, fingerprint, and hash-state counterpart to Vapor.toml. It should be plain TOML/lock-style, may exist per artifact root, and Phase 3 must read/write lock state for composition, fingerprints, hashes, and generated build/publish output.

### Vapor.toml.md

Defines Vapor.toml as the required manifest surface for every Vapor artifact root and every folder grouping capability declarations. It carries dependencies, conflicts, visibility, target roles, version constraints, placement/storage metadata, pack composition, and Steam/Workshop publication metadata, while Rhai files declare capabilities.

### Workflow Execution Trace Notes.md

Marks itself as a compatibility pointer. Concrete workflow examples and traces are now maintained in Workflow Usage Patterns Legacy Notes.

### Workflow Framework Premise Notes.md

Documents the core premise that workflow is Rust-side orchestration for materialized runtime values, with stage execution remaining Bevy-system-visible. It treats ECS, Render, Async, EcsWhile, and RenderWhile as first-class domains, separates control-plane lifecycle from execution-plane logic, and drafts run identity/concurrency via `run_id` and `concurrency_key`.

### Workflow Framework.md

Defines the Workflow Framework as the Rust-side orchestration layer for staged runtime work in the Runtime Substrate. It coordinates requests, progression, completion/failure, domain stages, and materialized capability/runtime values rather than raw Rhai engine internals.

### Workflow Instance Runtime Notes.md

Documents `WorkflowInstance` as the runtime-owned active request container. Variants match signature families, store identity, request ID, state, callbacks, stage counts, timeout frames, and data buffers, with placeholder replacement patterns while callbacks/data are in flight.

### Workflow Invariant Ledger Notes.md

Lists refactor-sensitive workflow invariants: placeholder stage-slot lifecycle, author-maintained output/input `transmute` compatibility, one active run per module/workflow key, single-item poll progress, RenderWhile sharding intent, and signature-family routing.

### Workflow Runtime Structure Notes.md

Documents current legacy workflow layering and request flow: macro declaration, registration, WorkflowPlugin resources/systems, call-site helpers, request relay, WorkflowMap insertion, stage initialization, domain buffers, pollers, completion/failure handling, and render-world extraction for render stages.

### Workflow Stage Runtime Notes.md

Documents stage-family behavior and stage lifecycle mechanics. It covers Ecs/Render/Async/EcsWhile/RenderWhile, setup/run wait/done semantics, placeholder swaps, unsafe output-to-input transmute contracts, single-item throughput, and RenderWhile split/fuse sharding.

### Workflow Stage Type Runtime Notes.md

Defines `StageType` as the workflow-domain discriminator with values Ecs, Render, Async, EcsWhile, and RenderWhile. It is stored in WorkflowState and drives routing through buffers, poll systems, render extraction, and diagnostics.

### Workflow State Runtime Notes.md

Documents `WorkflowState` as the per-instance state machine with Requested and Processing states. Processing tracks current stage, stage type, initialized/completed flags, and final removal from WorkflowMap.

### Workflow Type Request and Timeout Notes.md

Documents typed workflow request/response families: None, E, O, OE, I, IE, IO, IOE. It explains request IDs, per-signature channels, response inbox matching, base timeout panic behavior, controlled IOE timeout retry/abort/panic handling, and active-run gating.

### Workflow Usage Patterns Legacy Notes.md

Collects concrete legacy workflow usage patterns: startup completion signal, chunk boundary EcsWhile orchestration, GPU setup across ECS/RenderWhile/ECS, and GPU generation across ECS, RenderWhile, Render, and EcsWhile. It is the canonical page for examples other workflow notes link to.

### Zone-Era Concepts.md

Preserves older zone/DPT/ZLM/ZTM ideas as signal-bearing but no longer primary ontology authority. The core correction is that Phenomena can own localized/distributed/region-like significance, while zone-like tools may remain intermediate classification or realization-selection mechanisms.

### steam-like-platform-contracts.md

Defines a far-future possible abstraction for non-Steam platforms with Steam-like identity, ownership, Workshop-style distribution, and publishing contracts. It is explicitly not Phase 3 scope and must not weaken the current Steam-exclusive implementation path.
