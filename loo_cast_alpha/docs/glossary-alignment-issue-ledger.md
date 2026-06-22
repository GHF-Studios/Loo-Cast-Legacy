# Glossary Alignment Issue Ledger

Status: first-pass audit from `glossary-and-tech-glossary-summary.md` owner edits plus cross-document scans.

Purpose: identify the documentation changes implied by the owner corrections before rewriting glossary/RFC/source docs. This is a ledger, not final doctrine.

## Legend

- `direct`: explicitly requested or strongly implied by owner edits.
- `inferred`: found by cross-reading the glossary, tech glossary, RFCs, and current summary.
- `question`: needs owner confirmation before rewrite.
- `spillover`: affects non-glossary docs too.

## Global Structure

### GL-001: Preserve The Modified Summary As Owner-Intent Input

Type: direct.

Affected files:

- `loo_cast_alpha/docs/glossary-and-tech-glossary-summary.md`

Issue:
The summary now contains owner corrections inline. It should be treated as a temporary source-of-truth input, not polished documentation.

Likely action:
Keep it unchanged for now or archive it as an input record before replacing it with generated/embedded summaries.

### GL-002: Add Per-Page `Summary` Sections For Obsidian Embeds

Type: direct/inferred.

Affected files:

- all glossary and tech-glossary pages
- future summary index pages

Issue:
Obsidian supports embedding a note or a heading with `![[Note#Heading]]`. The glossary can use this by adding stable `## Summary` sections to pages and building summary indexes from embeds such as `![[Capability#Summary]]`.

Likely action:
Add a short `## Summary` section to each glossary page. Then create separate index files for normal glossary and tech glossary that embed those summaries rather than duplicating text.

### GL-003: Harden The `#glossary` vs `#tech_glossary` Split

Type: direct/inferred.

Affected files:

- `loo_cast_alpha/docs/glossary/README.md`
- all pages with implementation-facing content
- all pages with concept-level content

Issue:
The current split is useful but soft. Several pages tagged `#glossary` are planning or implementation-facing (`Vapor Crate Topology`, `Phase 3 Vapor Scenario Suite`, possibly `SDK`, `Vapor Launcher`, `Vapor.toml`, `Vapor.lock`). The README also includes tag text in prose, which makes naive tag scanning count it as both glossary and tech glossary.

Likely action:
Define classification criteria:

- `#glossary`: stable conceptual vocabulary and product/domain semantics.
- `#tech_glossary`: implementation notes, legacy/quarantine evidence, code/runtime behavior notes, phase/test/runbook notes.
- optional future tags: `#legacy_signal`, `#quarantine_signal`, `#phase_plan`, `#owner_input`.

### GL-004: Split Summary Indexes Into Normal And Tech Glossary

Type: direct.

Affected files:

- `loo_cast_alpha/docs/glossary/README.md`
- new `Glossary Summary.md`
- new `Tech Glossary Summary.md`

Issue:
The owner wants persistent TOC-like summaries for normal glossary and tech glossary. The current single summary is useful as an audit artifact, but it mixes conceptual and implementation-facing pages.

Likely action:
After `## Summary` sections exist, create two Obsidian-friendly index pages using embeds and explicit source ordering.

### GL-005: Keep Alphabetical Review Mode

Type: direct.

Affected files:

- summary/index pages

Issue:
The owner explicitly found alphabetical ordering useful for holistic review because it avoids rabbit-hole ordering.

Likely action:
Keep alphabetic per-page order in the review summaries, even if other topic maps are later added.

### GL-006: Add A Cross-Concept Coherence Map

Type: direct/inferred.

Affected files:

- new ledger/map document, possibly generated from links later

Issue:
The owner wants a “double loop” coherence pass: each concept checked against relevant other concepts. A fully exhaustive all-pairs review is too large to do manually in one edit pass, but the docs can support it with a graph/coherence map.

Likely action:
Create a topology map grouping concepts by capability, Vapor/product stack, Rhai, USF, workflow, artifacts, and phase planning. Then audit cross-group contradictions.

## Capability / Modding / Runtime Layering

### CAP-001: Capability Runtime Is Vapor-Level, Not Hosted By Spacetime Engine

Type: direct/spillover.

Affected files:

- `glossary/Capability Runtime.md`
- `glossary/Spacetime Engine.md`
- `glossary/Runtime Substrate.md`
- `glossary/Runtime Substrate.md`
- `glossary/USF Runtime.md`
- `docs/ARCHITECTURE.md`

Issue:
Current wording says the concrete Capability Runtime lives inside or is hosted by Spacetime Engine. Owner correction: Spacetime Engine should extend/utilize the capability runtime; Capability semantics supersede Engine/USF and help implement them.

Likely action:
Reframe Capability Runtime as Vapor-defined infrastructure with concrete runtime embeddings/adapters in launched Engine/Game compositions. Spacetime Engine is a first-party user/extension point, not the owner of the concept.

### CAP-002: Scaled Capability Channel Is Outdated Or Misframed

Type: direct.

Affected files:

- `glossary/Scaled Capability Channel.md`
- `glossary/Capability Contract.md`
- `glossary/Capability Runtime.md`
- `glossary/Scale Contract.md`
- `glossary/Scale Support.md`

Issue:
Current wording implies scale-scoped channels are intrinsic to capabilities or that capability infrastructure depends on USF. Owner correction: scale-scoped capability channels are a USF-compatible implementation pattern, not the general capability model.

Likely action:
Rename or reframe as `Scale-Scoped Capability Channel` or `USF-Scaled Capability Channel`, with an explicit boundary: general capabilities are Vapor-level and may or may not be scale-scoped.

### CAP-003: Capability Contract Is Overburdened By Legacy Concepts

Type: direct/question.

Affected files:

- `glossary/Capability Contract.md`
- `glossary/Contract.md`
- `glossary/Contract Family.md`
- `glossary/Capability.md`
- `glossary/Capability Runtime.md`

Issue:
Owner says the summary is mostly good but the concept feels broadly contaminated by legacy framing. The page mixes contract family, scaled channels, Rhai `ctx`, dependency layering, metadata requirements, and USF boundaries.

Likely action:
Do a focused rewrite or split:

- Capability Contract: durable rules a capability declaration/implementation must satisfy.
- Capability Runtime: runtime execution/orchestration.
- Capability Projection: script/callback facades.
- Capability Metadata: discovery/validation/diagnostic substrate.

### CAP-004: Modding And Capability Are Closer Than Docs Suggest

Type: direct/inferred.

Affected files:

- `glossary/Mod Runtime Representation.md`
- `glossary/Modding Contract.md`
- `glossary/Modding Runtime.md`
- `glossary/Capability.md`
- `glossary/Capability Runtime.md`

Issue:
Owner notes that Modding and Capability are not identical, but are more tightly coupled than many pages imply. Current docs sometimes present them as sibling families instead of deeply interdependent views over composition/runtime capability graph semantics.

Likely action:
Clarify that modding is one major product/composition use of the capability model. Modding runtime resolves mods/artifacts into capability graph contributions rather than standing beside capability semantics as an unrelated layer.

### CAP-005: Some Capabilities Directly Call Rust Kernels

Type: direct.

Affected files:

- `glossary/Runtime Intent Reconcile Commit Apply Mapping Notes.md`
- `glossary/Capability.md`
- `glossary/Capability Runtime.md`
- `glossary/Execution-Reconciliation Dual Core.md`
- `glossary/Rhai Capability.md`

Issue:
Current summary overstates “capabilities as intent emitters/request relays.” Owner correction: some capabilities are intent emitters, while others directly call into Rust kernel layers. Canonical state progression still needs reconcile/commit/apply discipline where relevant.

Likely action:
Replace blanket wording with capability classes:

- pure projected/query capabilities
- request/intent-emitting capabilities
- direct Rust-kernel binding capabilities
- authority/reconciler capabilities, if later formalized

### CAP-006: Hardcoded Native Capabilities Must Project Into Rhai Contexts

Type: direct.

Affected files:

- `glossary/Rhai Capability.md`
- `glossary/Rhai Asset.md`
- `glossary/Capability.md`
- `glossary/Capability Projection API.md`

Issue:
Owner correction: Rust-defined/hardcoded capabilities can be projected into Rhai `ctx` and used like any other context-visible capability. Rhai-defined capabilities are not the only projected things.

Likely action:
Make native-vs-Rhai capability origin explicit and separate origin from projection/use.

### CAP-007: Callback Context Type Needs Stronger Compile-Time/Metadata Framing

Type: direct.

Affected files:

- `glossary/Callback Context Type.md`
- `glossary/Callback Signature.md`
- `glossary/Callback Type.md`
- `glossary/Rhai Capability.md`
- `RFCS/phase_3_vapor_execution_spec.md`

Issue:
Owner correction: callback context metadata should mirror Rust function shape/signature enough to pull in or reject the required dependency/`ctx` capability graph projection at callback validation time.

Likely action:
Clarify callback context type as host-defined metadata plus projected context, not just a prose description of available data.

### CAP-008: Capability Role Taxonomy Remains Open, But Input/Output Has Evidence

Type: direct/question.

Affected files:

- `glossary/Capability Role.md`
- `glossary/Capability Role and State Authority Notes.md`
- `glossary/Scale Slice.md`

Issue:
Owner notes `Scale Slice` may support a simpler input/output split, but not enough to decide. Existing docs say input/output is underpowered.

Likely action:
Track both:

- input/output as useful directional vocabulary in some runtime/scale contexts
- broader authority/reconciler/realizer/bridge/mutator as unresolved role pressure

### CAP-009: Optional Providers And Integration Apertures Are Underdefined

Type: direct.

Affected files:

- `glossary/Modding Contract.md`
- `glossary/Slot Graph Composition.md`
- `RFCS/phase_2_to_11_execution_program.md`

Issue:
Owner is uncomfortable with `optional providers` and `integration apertures` because they are nearly unexplored. Current docs use them as if they are understood pressure terms.

Likely action:
Demote these terms to explicitly unresolved vocabulary or create small placeholder pages that say they are not yet formal concepts.

### CAP-010: Slot Graph Composition Needs Static-Core / Dynamic-Substrate Boundary

Type: direct.

Affected files:

- `glossary/Slot Graph Composition.md`
- `glossary/Runtime Lock.md`
- `glossary/Capability Runtime.md`

Issue:
Owner correction: slots are the static composition mechanism for the immutable startup-generated graph core/root/host. Dynamic mutable substrate layered on top does not follow Slot Graph Composition directly.

Likely action:
Add an explicit static graph core vs dynamic substrate section.

## Vapor / Product / Phase Planning

### VAP-001: Phase 3 Scenario Suite Should Become Phase 3 Vapor Testing Suite

Type: direct/spillover.

Affected files:

- `glossary/Phase 3 Vapor Scenario Suite.md`
- `glossary/README.md`
- `glossary/Packagepack.md`
- `glossary/Capability Graph Diagnostics.md`
- `RFCS/phase_3_vapor_execution_spec.md`
- `RFCS/phase_2_to_11_execution_program.md`
- `docs/NOW.md`

Issue:
Owner prefers `Phase 3 Vapor Testing Suite`, not `Scenario Suite` or `Test Suite`, because the suite includes manual verification flows that cannot be automatically run/verified. Existing docs repeatedly say scenario/integration-test suite.

Likely action:
Rename page and references, or keep old page as alias with canonical name changed. Replace `integration-test suite` wording with `testing suite` and distinguish automated validation fixtures from manually verified Steam/Workshop flows.

### VAP-002: Phase 3 Automated Testing Scope Is Overstated

Type: direct/spillover.

Affected files:

- `RFCS/phase_3_vapor_execution_spec.md`
- `glossary/Phase 3 Vapor Scenario Suite.md`
- `docs/NOW.md`
- `question_batch_005.txt` as source evidence

Issue:
Owner answer in batch 005 says Steam flows are manually verified, not integration-tested for most things. Current docs still use integration-test language too heavily.

Likely action:
Formalize three lanes:

- automated unit/pure-validation tests
- local manual scenario runs
- Steam/Workshop manual verification checklist

### VAP-003: Project Authoring Structure Must Reflect Upcoming Multi-Repo Split

Type: direct/spillover/question.

Affected files:

- `glossary/Project Authoring Structure.md`
- `glossary/Project Structure.md`
- `glossary/Vapor Crate Topology.md`
- `docs/NOW.md`
- `docs/ARCHITECTURE.md`

Issue:
Owner now expects a future Vapor-based multi-repo/multi-project layout splitting Loo Cast, Spacetime Engine, and Vapor. Current `NOW.md` says “Keep one repo,” which conflicts unless scoped as current-only.

Likely action:
Clarify current mono-repo as temporary crystallization workspace and add a future migration note for the Great Split into Vapor, Spacetime Engine, and Loo Cast repositories/projects.

### VAP-004: SDK Needs LSP Support Mentioned

Type: direct.

Affected files:

- `glossary/SDK.md`
- `RFCS/phase_3_vapor_execution_spec.md` if Phase 3 scope includes it

Issue:
Owner added LSP support “if/where applicable” to SDK expectations.

Likely action:
Add LSP/editor support as a desirable SDK surface, likely not a hard Phase 3 gate unless owner confirms.

### VAP-005: Build Artifact Needs Dynamic-Library Packaging Boundary

Type: direct.

Affected files:

- `glossary/Build Artifact.md`
- `glossary/Distributable Artifact.md`
- `glossary/Redistributable Mod Implementation Library.md`
- `glossary/Mod Artifact Structure.md`

Issue:
Owner correction: Build Artifact is akin to a linked shared object or binary before nearby/packaged dynamic library payloads are assembled.

Likely action:
Add concrete examples and clarify the boundary between built outputs and packaged distributable/runtime-deliverable library sets.

### VAP-006: Engine Page Has A Weird USF Boundary Coupling

Type: direct.

Affected files:

- `glossary/Engine.md`
- `glossary/Spacetime Engine.md`
- `glossary/USF.md`
- `glossary/Pillar Dependency Topology.md`

Issue:
Owner flags the Engine summary framing as awkward because non-Spacetime engines and first-party USF product-slot caution are related but not the same sentence-level concern.

Likely action:
Separate:

- generic Engine role and non-Spacetime engines
- first-party Spacetime/USF boundary

### VAP-007: Project Ethos Needs Inspiration / Systemic Reality

Type: direct.

Affected files:

- `glossary/Project Ethos.md`
- `glossary/Player-to-Creator Path.md`
- maybe `docs/README.md`

Issue:
Owner says inspiration is a major ethos component: inspiring people to look at the real systemic complexity and interconnection of reality rather than simplified lies.

Likely action:
Rewrite Project Ethos from a thin list into a stronger statement including inspiration, systemic complexity, interconnection, curiosity, and empowerment.

### VAP-008: Project Artifact Structure Feels Wobbly

Type: direct/question.

Affected files:

- `glossary/Project Artifact Structure.md`
- `glossary/Artifact.md`
- `glossary/Mod Artifact Structure.md`

Issue:
Owner is unsure about this concept. It may be too abstract or duplicative of Artifact and Mod Artifact Structure.

Likely action:
Either tighten it as project-scope artifact taxonomy or fold it into Artifact/Project Structure if it does not carry unique value.

### VAP-009: Vapor Public/Open Source / Multi-Project Implications Need Propagation

Type: inferred/spillover.

Affected files:

- `glossary/Vapor Ecosystem.md`
- `glossary/Vapor Crate Topology.md`
- `glossary/Project Authoring Structure.md`
- `docs/ARCHITECTURE.md`
- `docs/NOW.md`

Issue:
The future repo split changes wording around product ownership, public crate topology, proprietary first-party code, SDK availability, and project structure.

Likely action:
Create one authoritative repo/product topology note and link from affected pages.

## Rhai / Scripting / Legacy Signal

### RHAI-001: Legacy/Quarantine Notes Need Explicit Labels

Type: direct.

Affected files:

- `glossary/Rhai Bridge Domains and Access Provider Notes.md`
- `glossary/Rhai Reflection Macro Surface Notes.md`
- `glossary/Rhai Generic Dispatch Policy Notes.md`
- `glossary/Rhai Value Semantics and AccessCell Notes.md`
- workflow runtime notes, possibly

Issue:
Owner wants useful legacy/quarantine code clearly labeled as such. Current pages sometimes say high-signal/provisional, but not as a consistent status model.

Likely action:
Add a standard status block to all tech notes:

- active target doctrine
- legacy implementation signal
- quarantine/provisional signal
- obsolete/stale

### RHAI-002: Rhai Asset Phase 3 Proof Is Understated

Type: direct.

Affected files:

- `glossary/Rhai Asset.md`
- `RFCS/phase_3_vapor_execution_spec.md`

Issue:
Owner correction changes “prove one focused callback path” to “prove the whole Phase 3 stack working fully with capability stuff.” The focused callback proof may still be part of this, but the page should not understate Rhai's role in Phase 3.

Likely action:
Reword Phase 3 anchor: Rhai Asset must participate in end-to-end packagepack/capability/fingerprint/launcher/Steam proof, while callback taxonomy remains intentionally limited.

### RHAI-003: Scripting Projection Meta-Layer May Need Rename Or Deletion

Type: direct/question.

Affected files:

- `glossary/Scripting Projection Meta-Layer.md`
- `glossary/Global Capability API Graph.md`
- `glossary/Capability Projection API.md`
- `glossary/Rhai Capability.md`

Issue:
Owner finds the current concept unclear, silly, and possibly misnamed. It may overlap heavily with Capability Projection API.

Likely action:
Investigate whether this page should be:

- folded into Capability Projection API
- renamed to `Projection Context Mapping`
- kept only as a tech note
- deleted

### RHAI-004: Script Safety Needs Engine-Author Boundary

Type: direct.

Affected files:

- `glossary/Script Safety.md`
- `glossary/Steam.md`
- `glossary/Capability Graph Diagnostics.md`
- `RFCS/phase_3_vapor_execution_spec.md`

Issue:
Current script safety can sound like Vapor prevents malicious software generally. Owner correction: if someone implements an engine, Vapor cannot make malicious engine code impossible. The boundary should be script/projection safety and validation, not hostile-code sandboxing.

Likely action:
Clarify:

- Rhai scripts get constrained projected contexts.
- Workshop/downloaded content is validated for integrity/compatibility.
- Vapor Phase 3 is not a hostile-code sandbox, especially for native engine/mod binaries.

### RHAI-005: Rhai Generic Dispatch Page Is Correct But Dense

Type: direct.

Affected files:

- `glossary/Rhai Generic Dispatch Policy Notes.md`

Issue:
Owner says the concept is a mouthful but not wrong.

Likely action:
Keep substance, add a better `## Summary` and perhaps split declaration-first model from dispatch-catalog model.

### RHAI-006: Rhai Instantiation Script Language Feels Wobbly

Type: direct/question.

Affected files:

- `glossary/USF Instantiation Scripts.md`
- `glossary/USF Instantiation Capability Slot Notes.md`
- `glossary/Rhai Asset.md`
- `glossary/Rhai Capability.md`

Issue:
Owner flags the USF instantiation script framing as somewhat uncertain. It may be mixing general Rhai declaration substrate with USF-specific declaration profiles.

Likely action:
Separate generic Rhai declaration semantics from USF-specific script profiles/slot types.

## USF / Scale / Simulation

### USF-001: USF Math Raw Model Notes Are Outdated

Type: direct.

Affected files:

- `glossary/USF Math Raw Model Foundation Notes.md`
- `glossary/Scale Contract Runtime Notes.md`
- `glossary/USF Position Stack and Overflow Policy Notes.md`
- `glossary/Rhai Generic Dispatch Policy Notes.md`

Issue:
Owner moved away from a custom math implementation toward existing crates and `num_traits`, while retaining many mathematical semantics and constraints.

Likely action:
Rewrite page as historical raw-model semantics, not active implementation foundation. Preserve semantic ideas: explicit conversion boundaries, operation policy, determinism, panic contracts, shape/domain constraints. Replace custom-math-authority claims with “prefer established crates where possible.”

### USF-002: Scale Support Should Not Require Explicit Unsupported Entries For Every Pair

Type: direct.

Affected files:

- `glossary/Scale Support.md`
- `glossary/Scale Contract.md`
- `glossary/Scale Contract Runtime Notes.md`
- `glossary/Capability Contract.md`

Issue:
Current docs require each capability-scale pair to declare `supported` or `unsupported`. Owner doubts this due to combinatorial explosion. Explicit support declarations are desirable, but explicit non-support blacklists are likely not.

Likely action:
Change model toward positive support declarations plus default absence/unsupported semantics, unless a specific contract needs explicit denial.

### USF-003: Scale Support May Be More General Than Capability Support

Type: direct/question.

Affected files:

- `glossary/Scale Support.md`
- `glossary/Scale Contract.md`

Issue:
Owner notes support may apply to more than capabilities because capabilities may be more syntax/structure than semantics in some contexts.

Likely action:
Generalize Scale Support to scale-coordinate support declarations for capability-backed or contract-backed semantic surfaces.

### USF-004: Observer-Relative Simulation Needs Internal Consistency Pass

Type: direct/question.

Affected files:

- `glossary/Observer-Relative Simulation.md`
- `glossary/Scale View.md`
- `glossary/USF Runtime Evolution Lifecycle.md`
- `glossary/Phenomenon.md`
- `glossary/Metric.md`

Issue:
Owner says higher scales also use significance flow and current docs do not discuss how larger-scale changes affect lower scales. This may or may not belong directly in Observer-Relative Simulation.

Likely action:
Add an open-pressure section or create a separate `Cross-Scale Significance Flow` concept.

### USF-005: Definition Lock Applies To Immutable Startup Core, Not All Runtime Substrate

Type: direct.

Affected files:

- `glossary/USF Definition Lifecycle.md`
- `glossary/Runtime Lock.md`
- `glossary/Slot Graph Composition.md`
- `glossary/USF Runtime Evolution Lifecycle.md`

Issue:
Owner correction: definition mutation is absent from the immutable startup-constructed core, but dynamic substrate can still exist on top.

Likely action:
Add “immutable startup core vs dynamic runtime substrate” language consistently.

### USF-006: USF Instance Graph Multiplicity May Need Recheck

Type: inferred/question.

Affected files:

- `glossary/USF Instance Graph.md`
- `glossary/Scale Support.md`
- `glossary/Scale Contract.md`

Issue:
Current page says at least one Phenomenon and Metric per scale. Given support-declaration changes and USF uncertainty, this hard invariant may need owner reconfirmation.

Likely action:
Flag for question batch before rewriting.

## Entity / Spatial Semantics

### ENT-001: Entity Plane Split Enables Traditional f32/f64 Engines

Type: direct.

Affected files:

- `glossary/Entity Plane Split.md`
- `glossary/Spacetime Engine.md`
- maybe future physics/rendering notes

Issue:
Owner notes that entity plane split helps integrate traditional f32/f64 technologies such as Rapier.

Likely action:
Add this as an implementation motivation without overcommitting to Rapier specifically.

### ENT-002: Portal Traversal Should Include Entity Plane Split

Type: direct.

Affected files:

- `glossary/Portal Traversal Semantics.md`
- `glossary/Entity Plane Split.md`
- `glossary/Entity Proxy.md`

Issue:
Current portal page links Scale View, Observer-Relative Simulation, and Entity Proxy, but owner expects Entity Plane Split to be relevant too.

Likely action:
Add Entity Plane Split to portal semantics dependencies.

### ENT-003: Entity Proxy / World Wrapping Semantics Need Separation

Type: direct/question.

Affected files:

- `glossary/Entity Proxy.md`
- `glossary/Portal Traversal Semantics.md`

Issue:
Owner notes proxies simplify world wrapping mechanically, but semantically that may belong more to portal traversal.

Likely action:
Clarify proxy as a mechanism, portal traversal as semantic continuity model.

## Workflow / Legacy Runtime Notes

### WF-001: Delete Workflow Execution Trace Notes

Type: direct/spillover.

Affected files:

- `glossary/Workflow Execution Trace Notes.md`
- `glossary/README.md`
- all workflow note backlinks

Issue:
Owner says this page/concept should be removed as weird and outdated. It is currently only a compatibility pointer to Workflow Usage Patterns Legacy Notes.

Likely action:
Delete the page and remove backlinks from workflow notes and README, or replace with an alias redirect only if Obsidian link stability matters.

### WF-002: Stage Buffer Backlog Was A Real Runtime Problem

Type: direct.

Affected files:

- `glossary/Stage Buffer Runtime Notes.md`
- `glossary/Workflow Stage Runtime Notes.md`
- `glossary/Workflow Invariant Ledger Notes.md`
- maybe future workflow issue/RFC

Issue:
Owner confirms backlog caused visual holes and lagged the whole system. Current docs describe backlog neutrally; they should identify it as a legacy limitation/risk.

Likely action:
Add a “Known Legacy Problem” section and create a future implementation issue for throughput/backpressure/sharding.

### WF-003: Workflow Notes Need Legacy Status Consistency

Type: inferred.

Affected files:

- all `Workflow * Runtime Notes.md`
- `Stage Buffer Runtime Notes.md`
- `Stage Sender Cache Runtime Notes.md`

Issue:
Workflow pages document legacy behavior but some summaries read as active architecture. They need a consistent status banner: legacy implementation signal, not target doctrine unless promoted.

Likely action:
Standardize tech-note headers.

## General Concept Cleanup

### GEN-001: Closed Runtime And Open Design May Be Too Broad To Keep

Type: direct/question.

Affected files:

- `glossary/Closed Runtime and Open Design.md`
- `glossary/Runtime Lock.md`
- `glossary/Managed Ambiguity.md`
- `glossary/Project Runtime Representation.md`

Issue:
Owner says the concept feels over-generic, under-specified, and likely not redeemable in current form.

Likely action:
Either delete/fold into Runtime Lock and Managed Ambiguity, or rewrite narrowly around “immutable activation boundary, evolvable design between activations.”

### GEN-002: Capability Contract And Closed Runtime Are “Feels Wrong” Investigation Items

Type: direct/question.

Affected files:

- `glossary/Capability Contract.md`
- `glossary/Closed Runtime and Open Design.md`

Issue:
Both pages triggered broad owner discomfort, not just wording corrections. They should not be patched lightly.

Likely action:
Ask focused question batch before rewriting.

### GEN-003: Determinism Should Be “Deterministic-By-Default”

Type: direct.

Affected files:

- `glossary/Runtime Substrate.md`
- `glossary/Runtime Lock.md`
- `glossary/Capability Runtime.md`

Issue:
Owner edited summary to `deterministic(-by-default)`. Absolute determinism may overstate runtime reality.

Likely action:
Use “deterministic-by-default activation/composition” and distinguish deterministic lock/validation from nondeterministic or externally influenced runtime behavior.

### GEN-004: Summary Text May Be Ahead Of Source Pages

Type: direct/inferred.

Affected files:

- many glossary pages

Issue:
Owner noticed some summaries are more coherent than source pages. This likely means summary language should be promoted back into source pages after correction.

Likely action:
During rewrite, compare each source page against its summary and promote concise improved framing into `## Summary`.

### GEN-005: Need Questions Before Final Rewrite Of Fuzzy Concepts

Type: direct.

Affected files:

- owner-question batch document to be created

Issue:
Several concepts are uncertain enough that rewriting now would encode guesses: Capability Contract, Closed Runtime and Open Design, Scripting Projection Meta-Layer, USF Instantiation Scripts, Scale Support generalization, optional providers/integration apertures.

Likely action:
Prepare a focused question batch before editing those pages.

## Immediate Edit Candidates

These are low-risk enough to edit after ledger review:

1. Add legacy/quarantine status labels to Rhai bridge/reflection/value-semantics notes.
2. Add Entity Plane Split link to Portal Traversal Semantics.
3. Add f32/f64/traditional physics motivation to Entity Plane Split.
4. Add LSP support as SDK pressure.
5. Add Build Artifact examples.
6. Mark Stage Buffer backlog as legacy risk.
7. Delete or de-reference Workflow Execution Trace Notes.
8. Reword Spacetime Engine / Capability Runtime ownership.
9. Reword Scaled Capability Channel as USF-specific capability implementation pattern.
10. Rename Phase 3 Vapor Scenario Suite to Phase 3 Vapor Testing Suite after owner confirms filename/link strategy.

## Question Batch Seeds

1. Should `Closed Runtime and Open Design` be deleted, folded into Runtime Lock, or rewritten narrowly?
2. Should `Capability Contract` be split into multiple pages, or rewritten as one narrower page?
3. Should `Scripting Projection Meta-Layer` be deleted/folded into Capability Projection API, renamed, or kept as a tech note?
4. Should `Scale Support` mean positive support declarations only, with absence meaning unsupported?
5. Does Scale Support apply only to capabilities, or to any scale-aware semantic surface?
6. Should `Phase 3 Vapor Scenario Suite.md` be renamed on disk, or should it keep the filename with a new canonical name/alias?
7. Is the future repo split now strong enough to update `NOW.md`, or should `NOW.md` continue saying “keep one repo” as current operational policy?
8. Should `USF Instance Graph` still require at least one Phenomenon and Metric per scale?
9. Should `USF Instantiation Scripts` be USF-specific only, with generic Rhai declaration semantics moved elsewhere?
10. Should workflow legacy notes remain in glossary indefinitely, or move to a legacy implementation evidence folder?
