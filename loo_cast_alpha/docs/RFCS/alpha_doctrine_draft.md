# Alpha Doctrine Draft (Incremental)

Status: Draft in active conversation loop. Sections are locked only when explicitly confirmed.
Consolidation note: this V1 infodump source is now consolidated under `phase_2_to_11_execution_program.md`.

Superseded caveat (2026-06-17):
This draft is historical where it conflicts with `question_batch_004a.txt`, `NOW.md`, or the current glossary.
In particular, USF is not a standalone product/package slot; it is a public/API-facing subsystem inside the
[[Spacetime Engine]].
Current Phase 3 planning centers the Vapor launcher/product-stack proof, not a USF runtime spine.

## 1) Ontology (Locked v1)

- A `mod` is a package unit with dependencies and optional capability exports.
- A `mod` may also declare conflicts.
- A `capability` is a globally namespaced contract/API surface (`mod_id::capability_id`) provided by mods and resolved by defined rules.
- A single mod may export multiple capabilities.
- A `modpack` is a runtime composition spec selecting mods and may recursively include other modpacks.
- Private mods are not valid as direct modpack entries.
- `USF` is composed behavior built from capability-providing mods, not a monolithic special-case package.
- `core_mod` is an entry/facade mod that may depend on a graph of internal core mods.

Open questions for this section:

- (none)

## 2) Authority Model (Locked v1)

- Authority should be explicit and fail-fast where ambiguity would create undefined runtime behavior.
- Resolution should be contract-first: capabilities and ownership rules decide what is valid, not incidental load order.
- Singleton/critical definitions (for example per-scale ownership-like constraints) must resolve to exactly one effective authority.
- Conflicts are first-class constraints, not advisory hints.
- The launcher/loader validates authority and compatibility before runtime activation.
- User configuration (including load order) may only choose among already-valid outcomes.
- Invalid user configuration must be blocked by a clear gate (for example red-highlighted invalid state + launch disabled).
- Conflicts are symmetric at resolution time: if `A` conflicts with `B`, `{A,B}` is invalid regardless of declaration side.
- Alpha policy: hard-fail on invalidity/conflict; no soft-allow fallback.

Open questions for this section:

- (none)

## 3) Phase Intent Model (Locked v1)

- Phases alternate between planning and implementation by default.
- Planning phases are for locking concepts, contracts, boundaries, and proof expectations.
- Implementation phases are for proving previously locked plans with minimal sufficient execution.
- If implementation reveals a true conceptual gap, flow returns to the matching planning phase (explicit loopback, not silent drift).
- There is no mandatory single output artifact type per phase.
- Implementation phases are not strictly forbidden from scope expansion by a hard gate.
- A formal loopback marker is required when implementation is blocked by unresolved planning ambiguity.

Draft intent map:

- Phase 2: planning lock for alpha doctrine + mod/framework contracts.
- Phase 3: implement modding framework baseline and prove end-to-end minimal loop.
- Phase 4: plan USF semantics contracts.
- Phase 5: implement USF semantics contracts.
- Phase 6: plan USF math contracts.
- Phase 7: implement USF math contracts.
- Phase 8: plan broad capability platform needed before gameplay.
- Phase 9: implement capability platform baseline.
- Phase 10: plan gameplay slice target.
- Phase 11: implement gameplay slice + human validation proof.

Open questions for this section:

- (none)

## 4) Layer Identity Model (Locked v1)

- The project defines three encapsulated layer experiences:
  - Root-Level Development
  - Mod-Level Development
  - User-Level Deployment
- Layers are related but not interchangeable; each has distinct goals, constraints, and success criteria.
- Role lattice applies: `rooter ⊃ modder ⊃ user`.

Root-Level Development (platform-authoring layer):

- Primary concern: contracts, core architecture, foundation capabilities, and platform governance.
- Primary app/tool surface: repository + maintainer development environment (IDE/toolchain of choice).
- Freedom level: highest.
- Responsibility level: highest.

Mod-Level Development (SDK-authoring/consuming layer):

- Primary concern: creating mods against published contracts with strong ergonomics.
- Primary app/tool surface: modding SDK + tooling workflow, with optional IDE integrations.
- Freedom level: high within contract boundaries.
- Responsibility level: maintain compatibility and declared capability/conflict semantics.

User-Level Deployment (composition/runtime layer):

- Primary concern: compose, configure, activate, and run mod/modpack setups reliably.
- Primary app/tool surface: launcher + game/engine binary (dual-app model).
- Freedom level: constrained to valid resolved configurations.
- Responsibility level: choose from valid options; runtime safety gates remain enforced.

Identity principle:

- Each layer must feel complete and coherent on its own, with a simple default path and optional advanced depth.
- Root-only operations should be explicitly labeled (`root-only`) in docs/tooling to prevent accidental boundary crossing.
- Modpack publishing is user-layer by default, with optional advanced controls exposed for modder workflows.
- Launcher complexity is split into basic vs advanced mode:
  - basic: download/use existing modpacks, create straightforward modpacks
  - advanced: clone existing modpacks and perform explicit content additions/changes behind an intentional edit-toggle
  - invalid configurations remain visibly gated and non-launchable

Open questions for this section:

- (none)

## 5) Governance Worldview (Locked v2)

- Governance should maximize ecosystem openness without sacrificing deterministic safety and coherence.
- Contract clarity is preferred over implicit social conventions.
- Invalid states are blocked by system rules, not deferred to post-breakage support.

Trust model:

- Root layer is trusted to evolve core contracts and platform internals.
- Modders are trusted to innovate within published contracts.
- Users are trusted to compose configurations, but launcher/runtime gates enforce validity boundaries.

Compatibility ethics:

- Compatibility claims should be explicit (declared), not assumed.
- Conflict and capability semantics must be machine-resolvable where possible.
- Breaking changes should be intentional, visible, and paired with migration intent.

Publishing norms:

- Mods and modpacks are both publishable artifacts.
- Default UX favors safe and reproducible compositions over hidden magic.
- Ownership and provenance of published artifacts should remain visible.

Operational principle:

- "Open by default, gated by invariant safety."
- Doctrine defines provenance trust tiers for artifacts.
- Trust tiers are a doctrine-level visibility/safety construct, not a full moderation regime.
- Initial trust tiers are minimal: `official` and `unofficial`.
- `official` maps to built-in first-party artifacts; `unofficial` maps to non-built-in artifacts (for example Workshop-acquired).
- In alpha, trust tier does not change behavior by itself; it is primarily a classification distinction.
- Compatibility declarations are mandatory for publishable artifacts from day one of public SDK release.
- Root/official artifacts must not rely on undocumented behavior.
- Community artifacts may experiment more freely, but launcher/runtime safety gates still enforce invariant boundaries.

Open questions for this section:

- (none)

## 6) Defer/Commit Policy + Anti-Goals (Locked v1)

Defer/commit policy:

- Commit now when a decision affects cross-layer contracts, safety gates, or phase ordering.
- Defer when a choice is implementation-local and does not alter published contracts.
- Defer by default only if a clear revisit trigger is recorded.
- If a deferred decision starts causing repeated ambiguity, escalate it to committed doctrine.

Revisit trigger rule:

- Every deferred item should carry a trigger condition (for example "before Phase 3 implementation", "before public SDK", or "before gameplay planning lock").

Anti-goals (what this project should not become):

- Not a silent-fallback system that launches invalid configurations.
- Not a monolithic hardcoded engine where mods are superficial.
- Not a loader that treats conflicts/capability ambiguity as best-effort warnings.
- Not a documentation-only SDK without practical tooling/workflow support.
- Not a UX that requires expert-level steps for basic user/modder onboarding.

Doctrine safety preference:

- Prefer explicit constraints over hidden permissiveness.
- Prefer deterministic failure over nondeterministic success.
- Deferred decisions are tracked in doctrine/RFC documentation only (no separate index artifact).
- No numeric cap is imposed on unresolved deferred items per phase.

Open questions for this section:

- (none)
