# Draft Phase Buckets (2-11, Expandable)

Status: Working draft only. Nothing here is final, locked, or committed as roadmap truth.
Consolidation note: this V1 infodump source is now consolidated under `phase_2_to_11_execution_program.md`.
Current note (2026-06-17): Phase naming/scope in this file is stale wherever it conflicts with
`question_batch_004a.txt`, `NOW.md`, or current glossary pages.
Current framing is Phase 2: Vapor + USF Planning and Spec Crystallization; Phase 3: Vapor Launcher and Product-Stack
Proof; Phase 4: Product-Stack + USF Prototype/MVP.

Rules:

1. Buckets are numbered from `2` upward. Current draft uses `2..11` and may expand if needed.
2. Bucket names are optional and always `DRAFT`, never final.
3. Items enter a bucket only after basic ambiguity is resolved in discussion.
4. No cross-bucket overlap, ever: each item must exist in exactly one bucket.
5. Final naming/ordering/lock-in happens later, after infodump + refinement.

---

## Intake Queue (Unsorted)

Use this for raw points before we decide a bucket.

- [ ] (empty)

### Active Discussion IDs (Reference Index, Bucketed Below)

`R03` Runtime spine restoration

User position:
Process is already strong, runtime substance is thin. Restore core crates/mods/binaries soon, and avoid polishing too
early.

Integrated commentary:
Treat this as "substance before shine": rebuild executable/runtime capability first, then optimize/refine later. This is
the bridge from workflow maturity to product maturity.

Unresolved:
What is the minimum runtime slice that counts as "spine restored" for alpha progression?

`R04` Mod API definition lock

User position:
High-level modding decisions must be explicitly defined/locked because consequences are broad and long-range.

Integrated commentary:
Agree. Need a deliberate decision surface for mod identity/capability contracts before deep implementation scaling.

Unresolved:
Which decisions are hard-locked now vs provisional-locked with revisit triggers?

`R05` Composition + capability/plugin framing

User position:
Engine-level capabilities may be modeled as mod-like capability sets, likely attached to Bevy `Plugin`/`PluginGroup`
structures; business/presentation split is foundational.

Integrated commentary:
Agree. This is not side content; this is core architecture direction. The split is especially important for
projection-heavy world logic.

Unresolved:
How strict is the split at alpha (hard invariant vs strong guideline)?

`R06` End-state-first planning

User position:
Define what `0.5.0` must be able to do, then work backwards. Avoid detail-first drift.

Integrated commentary:
Agree strongly. This is the ambiguity-killer and should drive roadmap structure.

Unresolved:
What is the concise, testable stable-alpha capability definition?

`R07` USF/math/authority framing

User position:
Current framing is too uncertain/ambiguous; avoid simplistic two-way framing and derive from best-case target
requirements.

Integrated commentary:
Agree. Start from required end-state semantics, then choose decomposition (not the other way around).

Unresolved:
Which USF/math/authority capabilities are alpha-critical vs explicitly deferred?

`R08` Event/message seam preference

User position:
Module boundaries should primarily hinge on events/messages rather than direct coupling.

Integrated commentary:
Agree. Use seams as default coupling strategy unless direct calls are clearly justified.

Unresolved:
What explicit exceptions to seam-first are allowed at alpha?

`R09` Real mod identity + packaging semantics

User position:
Current setup does not yet define "what a mod is" rigorously for real third-party mod lifecycle/discovery/loading.

Integrated commentary:
Agree. This is a major identity gap; composition credibility depends on closing it.

Unresolved:
What minimum mod identity contract is required for alpha (manifest, load path, dependency semantics, runtime lifecycle)?

`R10` Human runtime oversight

User position:
Important, but lower near-term priority while foundational systems are still being shaped.

Integrated commentary:
Agree on priority. Keep as planned validation layer, but do not let it drive early architecture sequencing.

Unresolved:
What minimum human-reviewed runtime evidence format should be prepared now (for later use)?

`R11` Legacy extraction policy (meta-policy)

User position:
Yes, important, but should be policy documentation, not another main feature point in this pass.

Integrated commentary:
Agree. Track as policy/RFC discipline that constrains how restoration is done.

Unresolved:
Where should this policy live as source of truth (RFC-only vs WORKFLOWS/DECISIONS references)?

`R12` Stable-alpha definition

User position:
Core keystone. Need concrete definition of working stable alpha to resolve many downstream ambiguities.

Integrated commentary:
Agree strongly. This should anchor phase bucketing and later lock-in.

Unresolved:
What exact capability checklist defines "working stable alpha" independent of implementation style?

`R13` Alpha 0.5.0 startup path (launcher + default modpack)

User position:
`0.5.0` means a user can start via launcher, get a shipped default/built-in modpack, and have it selected on first startup.

Integrated commentary:
This gives a concrete user-visible baseline and should become part of the stable-alpha acceptance proof.

Decision (2026-05-09):
Hard fail/abort startup if default modpack is invalid or missing, but do not crash the launcher process itself. Report
the failure cleanly and stop launch flow. Launcher crash is only acceptable if launcher logic itself panics.

`R14` Core mod as first-party but not hardcoded

Status:
Superseded by 2026-06-16 owner answers in `question_batch_002.txt`.

Old user position:
`core_mod` should provide fundamental capabilities but still remain a normal mod artifact (deselectable, not untouchable
engine magic).

Superseded decision (2026-05-09):
This R14 entry previously treated `core_mod` as a normal mod artifact that could be deselected/replaced, with framework
bootstrap no-op/exit behavior if no effective core mod was present.
That premise is now wrong.

Current correction:
`core_mod` is mandatory for a valid Vapor product instance, but it is replaceable by selecting another valid coupled
`core_engine`/`core_mod` pair.
It is not optional, freely removable, or independently mix-and-match replaceable.
`core_engine`, `core_mod`, and `base_mod` are non-negotiable core architecture pillars of a Vapor product instance
stack.
They are reserved built-in mod roles, not ordinary extension mod names.
The `base_mod` layer is also mandatory, but replaceable as the Game-level concept.

`R15` Per-scale singleton and non-empty set invariants

User position:
For each scale, key definitions must be singleton or non-empty by contract (e.g., one scale definition, one realizer, at least one
phenomenon, at least one metric).

Integrated commentary:
Agree. This is the main anti-chaos invariant and a core alternative to Java/Minecraft-style loose mixin override behavior.

Decision (2026-05-09):
Fail fast. Missing required per-scale singleton/non-empty invariants must abort startup because representational
completeness is broken and panic-risk becomes structurally possible.

`R16` Rhai role: declarative definition surface, not performance-critical implementation core

User position:
Native Rust should stay the default for performance-critical logic; Rhai is valuable as a dynamic/declarative surface for definitions,
capability wiring, and state-derived generation intent.

Integrated commentary:
Agree. This gives a practical split: Rust owns hard kernels, scripts own shape/configuration/orchestration intent.

Decision (2026-05-09):
Rhai is definition/description-first. Do not use Rhai as the normal execution/update logic layer. Procedural/behavioral
script logic can exist as a rare edge-case mechanism, not as the default architecture path.

`R17` USF authority + zoom traversal semantics

User position:
USF is the authority model and core complexity center; ECS/Bevy is substrate, not semantic owner. Scale traversal/zoom is a
first-class mechanic with strict cross-scale semantics.

Integrated commentary:
Agree. This should be treated as an explicit contract surface, not an emergent side effect of implementation details.

Unresolved:
What is the minimum alpha traversal contract (camera/player semantics + cross-scale continuity guarantees) that must be proven live?

`R18` Digit-stack positional math recovery path

User position:
Digit-stack/carry-style cross-scale positional math is foundational and should not be lost; it needs deliberate review and revival.

Integrated commentary:
Current repository state indicates this logic is strongest in quarantine/temp paths, not in active runtime. Recovery plan is required.

Unresolved:
Do we promote quarantine math directly (with cleanup), or re-spec from first principles and re-implement against tests?

`R19` Product identity: mod graph is the product surface

User position:
Implied by the full discussion: engine internals are substrate; the meaningful product surface is the mod graph and its
contracts (including first-party mods).

Integrated commentary:
Strong architectural anchor. Prevents hardcoded engine-special cases and keeps first-party/third-party treatment aligned.

Unresolved:
Which parts are truly engine-private forever vs required to be mod-surface contracts by alpha?

`R20` Load-time ownership authority for per-scale definitions

User position:
Implied by singleton/conflict requirements: ownership of scale definitions must be deterministic and conflict-rejecting.

Integrated commentary:
This is the largest remaining contract gap: "who is allowed to define what, at which scale, and how conflicts are
resolved or rejected."

Unresolved:
What exact ownership/precedence/conflict algorithm should loader validation enforce at startup?

`R21` Macro/workflow sequencing discipline

User position:
Implied by roadmap concerns: avoid letting generated machinery hide unresolved domain contracts.

Integrated commentary:
Domain contracts first, macro abstraction second. Use macro power to encode settled decisions, not to decide semantics by
accident.

Unresolved:
Which workflow/macro surfaces are allowed in early restoration before domain contracts are locked?

`R22` Launcher + modding framework baseline timing

User position:
Launcher plus modding framework/setup is not optional later work; it must be an explicit early implementation target.

Integrated commentary:
Agree. This is a prerequisite for meaningful alpha execution and must not be deferred beyond initial contract/kernel
phase work.

Decision (2026-05-09, revised):
Treat launcher + mod framework bootstrap as Phase-3-scoped implementation work (after Phase-2 planning lock).

`R23` Alpha requires actual basic gameplay technical-demo slice

User position:
Alpha must not be only "framework with zero content." It needs a basic but comprehensive gameplay slice (broad and/or
deep), even if complex interactions and cross-scale mechanics remain limited.

Integrated commentary:
Agree. This is the substance bar that turns framework correctness into product credibility.

Unresolved:
What exact minimum gameplay demo slice is required for alpha proof (interaction set, loop, and completion criteria)?

`R24` Plan->implement cycle policy across domains

User position:
Roadmap should follow domain-local cycles: plan first, then implement; repeat per major concern.

Integrated commentary:
Agree. This reduces premature execution and keeps dependency order explicit.

Decision (2026-05-09):
Use alternating plan->implement waves as default sequencing policy.

`R25` USF semantics implementation wave (separate from USF math)

User position:
USF semantics and "USF math" are related but distinct concerns; they should not be collapsed into one bucket/wave.

Integrated commentary:
Agree. Semantics implementation should be its own wave after semantics planning, before dedicated math planning/impl.

Unresolved:
What is the minimum "USF semantics implemented" proof before entering the dedicated USF math wave?

`R26` USF math planning wave (separate from USF semantics planning)

User position:
USF math needs separate planning treatment with its own contracts and invariants.

Integrated commentary:
Agree. Prevents semantic-model concerns from obscuring math-contract decisions.

Unresolved:
Which math contracts are alpha-critical vs explicitly postponed?

`R27` USF math implementation wave

User position:
After planning USF math, implementation should be a distinct follow-up wave.

Integrated commentary:
Agree. This supports testability and avoids mixed concern churn.

Unresolved:
What implementation strategy is preferred for initial recovery: direct promotion from quarantine vs re-spec+rebuild?

`R28` Capability platform planning gate before gameplay

User position:
Gameplay work should wait until a broad baseline capability set is planned (models/textures/sound/transform/VFX/IO/etc.).

Integrated commentary:
Agree. This is the structural prerequisite for non-trivial gameplay content.

Unresolved:
What minimum capability inventory must be planned before gameplay planning starts?

`R29` Capability platform implementation wave

User position:
Capability planning must be followed by a dedicated implementation wave before gameplay implementation.

Integrated commentary:
Agree. This keeps gameplay from becoming capability-prototyping by accident.

Unresolved:
What completion criteria mark capability implementation as "enough to start gameplay implementation"?

`R30` Gameplay implementation wave with human-validated proof

User position:
After gameplay planning, implementation should culminate in a concrete playable proof slice.

Integrated commentary:
Agree. This is where framework and capability work convert into user-facing substance.

Unresolved:
What exact playable proof artifact and review protocol define completion?

`R31` Three-layer interaction model (root/mod/user)

User position:
Loo Cast should expose three encapsulated interaction layers: Root-Level Development, Mod-Level Development, and
User-Level Deployment. Mod-level may be embedded inside root-level workflows, and user-level deployment is required for
real testing feedback loops.

Integrated commentary:
Agree. Treat these as three products with distinct UX and constraints, not a single blurred workflow.

Decision (2026-05-09):
Adopt three-layer model as core architecture contract.

`R32` Role lattice and permission boundaries

User position:
Role hierarchy is strict: rooter ⊃ modder ⊃ user. Root development is maintainer-only; mod development is open to anyone
who installs SDK requirements; users can compose/play/publish modpacks, and (re-)configure mods.

Integrated commentary:
Agree. This clarifies governance and expected power per layer.

Decision (2026-05-09):
Adopt role lattice as canonical operational model.

`R33` SDK scope: API + tools + workflow + automation

User position:
SDK must include tooling and workflow support, not only API docs; optional IDE integrations/plugins are in scope as
extensions. SDK should support both external modders and internal development loops.

Integrated commentary:
Agree. SDK is a product surface, not only a library.

Decision (2026-05-09):
Define SDK as full modding product (contracts + tooling + docs + automation), with IDE adapters as optional layer.

`R34` Layer-specific primary app experiences

User position:
Each layer should have a proper encapsulated experience with a primary app: root-level via chosen IDE, mod-level via IDE
with tighter loop automation, user-level via launcher and game/engine binary (launcher as composition/config/modding tool
for non-root users; game binary as runtime execution surface).

Integrated commentary:
Agree. This prevents UX ambiguity and clarifies ownership of workflow responsibilities, especially the user-layer
launcher/runtime split.

Decision (2026-05-09):
Adopt per-layer primary app model as guiding UX principle, with explicit user-layer dual-app contract (launcher + game
binary).

`R35` Onboarding simplicity mandate

User position:
Despite deep complexity, setup and first-use for modders/users should stay as simple as possible with clear starter paths.

Integrated commentary:
Agree. "Fast to start, deep when needed" should be explicit policy.

Decision (2026-05-09):
Treat onboarding simplicity as non-optional design constraint for SDK + launcher flows.

`R36` Modpack authoring as user-layer capability

User position:
Not every user is a modder, but users may still compose/publish modpacks. Modpack development sits between pure play and
full mod coding and must be supported explicitly.

Integrated commentary:
Agree. This is a first-class persona, not an edge case.

Decision (2026-05-09):
Support modpack authorship/publishing as user-layer capability in launcher/deployment surfaces.

`R37` Steam-centered SDK/deployment distribution channel

User position:
Modding and deployment experience should be deliverable via Steam-facing channels (with guides/links for extra third-party
dependencies where required).

Integrated commentary:
Agree. Distribution/integration strategy is part of product design, not post-hoc ops.

Unresolved:
What exact split is shipped directly vs documented as external prerequisite in first public SDK/deployment iteration?

`R38` Phase 3 implementation wave for layerized workflows

User position:
After Phase-2 planning lock, Phase 3 should implement concrete layer workflows and tooling surfaces (not just framework
internals in isolation).

Integrated commentary:
Agree. Implementation must validate the three-layer model with real developer/user loops.

Decision (2026-05-09):
Phase-3 minimal done-criteria:
1. Real modding loop exists end-to-end (mod(s) composed by user, resolved, and run through launcher->game path).
2. Resulting app can be minimal (even glorified hello-world behavior), including simple output/log proof in launcher
   console path.
3. Mod development path is operational through SDK/tooling workflow, with optional IDE integration support.
4. User can compose and (re-)configure mod setup through launcher-level UX.
5. First concrete capability proof is logging: capability must be provided and consumed through the real capability/mod
   resolution path (no hardcoded bypasses).

Cross-ID unresolveds (shared):

- Minimum stable-alpha proof surface for composition: one scenario vs multiple scenario classes?
- Which legacy towers are mandatory for alpha identity vs explicitly post-alpha?
- Which invariants must fail-fast at load time vs warn-and-continue during development mode?
- What is the minimum "first-run onboarding path" for user, modpack author, and mod coder personas?

---

## Bucket 2

Draft Name (Optional): `DRAFT: Phase 2 Planning Baseline (Alpha + Mod Framework Contracts)`

Items:

- [ ] `R06` End-state-first planning
- [ ] `R12` Stable-alpha definition
- [ ] `R04` Mod API definition lock
- [ ] `R19` Product identity: mod graph as product surface
- [ ] `R16` Rhai role as definition/description-first
- [ ] `R09` Real mod identity + packaging semantics
- [ ] `R13` Launcher startup + default modpack behavior
- [ ] `R14` superseded: `core_mod` as mandatory replaceable matched engine pillar, not optional gameplay mod
- [ ] `R15` Per-scale singleton/non-empty fail-fast invariants
- [ ] `R20` Deterministic load-time ownership authority
- [ ] `R11` Legacy extraction policy
- [ ] `R21` Macro/workflow sequencing discipline
- [ ] `R24` Plan->implement cycle policy
- [ ] `R31` Three-layer interaction model (root/mod/user)
- [ ] `R32` Role lattice and permission boundaries
- [ ] `R33` SDK product scope (tools/workflow/automation)
- [ ] `R34` Layer-specific primary app experiences
- [ ] `R35` Onboarding simplicity mandate
- [ ] `R36` Modpack authoring as user-layer capability
- [ ] `R37` Steam-centered SDK/deployment distribution strategy

Open Questions:

- What is the smallest complete alpha capability statement that still constrains all downstream design choices?
- Which planning decisions in this phase are hard-lock vs provisional-lock?

---

## Bucket 3

Draft Name (Optional): `DRAFT: Phase 3 Modding Framework Implementation`

Items:

- [ ] `R03` Runtime spine restoration
- [ ] `R22` Launcher + mod framework bootstrap implementation
- [ ] `R38` Layerized workflow implementation proof (root/mod/user)

Open Questions:

- (none currently; baseline captured in `R38` decision)

---

## Bucket 4

Draft Name (Optional): `DRAFT: Phase 4 USF Semantics Planning`

Items:

- [ ] `R07` USF/math/authority framing
- [ ] `R17` USF authority + zoom traversal semantics

Open Questions:

- What semantics contracts must be locked before semantics implementation begins?

---

## Bucket 5

Draft Name (Optional): `DRAFT: Phase 5 USF Semantics Implementation`

Items:

- [ ] `R25` USF semantics implementation execution

Open Questions:

- What minimum live proof demonstrates USF semantics are truly implemented (not just sketched)?

---

## Bucket 6

Draft Name (Optional): `DRAFT: Phase 6 USF Math Planning`

Items:

- [ ] `R18` Digit-stack positional math recovery path
- [ ] `R26` USF math planning wave

Open Questions:

- Which USF math contracts are alpha-critical and which can be postponed?

---

## Bucket 7

Draft Name (Optional): `DRAFT: Phase 7 USF Math Implementation`

Items:

- [ ] `R27` USF math implementation wave

Open Questions:

- Promote-from-quarantine vs re-spec+rebuild: which path do we execute for math implementation?

---

## Bucket 8

Draft Name (Optional): `DRAFT: Phase 8 Capability Platform Planning`

Items:

- [ ] `R05` Composition + capability/plugin framing
- [ ] `R08` Event/message seam preference
- [ ] `R28` Capability platform planning gate before gameplay

Open Questions:

- What minimum capability inventory must be planned before gameplay planning starts?

---

## Bucket 9

Draft Name (Optional): `DRAFT: Phase 9 Capability Platform Implementation`

Items:

- [ ] `R29` Capability platform implementation wave

Open Questions:

- What completion criteria make capability implementation "enough" to unblock gameplay implementation?

---

## Bucket 10

Draft Name (Optional): `DRAFT: Phase 10 Gameplay Planning`

Items:

- [ ] `R23` Basic gameplay technical-demo slice (non-zero content alpha bar)

Open Questions:

- What exact minimum gameplay loop/scope is planned for alpha proof?

---

## Bucket 11

Draft Name (Optional): `DRAFT: Phase 11 Gameplay Implementation + Human Validation`

Items:

- [ ] `R10` Human runtime oversight
- [ ] `R30` Gameplay implementation wave with human-validated proof

Open Questions:

- What exact playable proof artifact and review protocol define phase completion?

---

## Later Lock-In Checklist (Do Not Use Yet)

- [ ] Confirm bucket names (remove `DRAFT:` prefixes)
- [ ] Resolve overlaps and move misplaced items
- [ ] Set final phase ordering
- [ ] Convert buckets into milestone/issues only after full review
