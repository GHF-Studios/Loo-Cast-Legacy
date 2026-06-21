# NOW

Date: 2026-06-20

Purpose: short current checkpoint, not doctrine.

Current direction:

- Keep one repo.
- Keep xtask small and focused.
- Treat docs as WIP crystallization surfaces, not trusted doctrine by default.
- Treat the glossary as the primary active crystallization surface.
- Treat RFCs as stale whenever they conflict with current glossary pressure or owner answers.
- Use `docs/ai_conversation_logs/question_batch_*.txt` as the default Q&A timeline for now.
- Question batches document both sides of the conversation: agent questions and owner answers/corrections.
- New architecture threads should usually become focused question batches before glossary/RFC promotion.
- Last integrated answered thread: `question_batch_005.txt`; first focused glossary integration pass complete, with unresolved points left as pressure.
- Last answered raw thread: `question_batch_005.txt`.
- Active question thread: chat-based follow-up on batch 005 integration. Concrete engine/game internals, USF, worldmodel, rendering, and gameplay remain out of scope for this thread.
- Remove the old `WIP_QUESTION_LEDGER.txt`; question batches replace it.
- Product stack skeleton: [[Vapor Product Stack]].
- Vapor is Steam-exclusive and includes SDK/tooling plus runtime protocol/contracts/traits for engines, games, mods, packagepacks, modpacks, enginepacks, and gamepacks.
- Vapor also includes foundational Capability and Rhai authoring substrate; these are not engine/game-specific details.
- Vapor crate/workspace shape, product layer, ecosystem layer, SDK, launcher, and distribution surface are all critical; none is secondary.
- Candidate crate topology: `vapor_core`, `vapor_sdk`, `vapor_launcher`, `vapor_steam`, and `vapor_macros`, with SDK and launcher as siblings over `vapor_core`.
- `Vapor.toml` is the current pressure term for manifest-style metadata, dependencies, conflicts, target roles, version constraints, and Steam/Workshop publication fields.
- `Vapor.toml` is required for every Vapor artifact root and every folder level that groups capability declarations.
- Current Vapor.toml pressure: folder composition/nesting/organization/storage metadata plus explicit dependencies and conflicts. No implicit defaults; no special primary-dependency attachment model.
- `Capability Slot Type` is the preferred current term over `Capability Profile` / `Capability Type Template` for projected/gated capability slot/context shape.
- Callback terminology is split into `Callback Type`, `Callback Context Type`, and `Callback Signature`.
- Fingerprints are deterministic identity/compatibility summaries used for validation, diagnostics, caching, publishing, and reproducibility.
- `Vapor.lock` is the current pressure term for generated fingerprint/hash/resolution state; use plain TOML/lock-style files, not a database, for now.
- Runtime Lock currently applies to launchable Engine/Game runtime composition; SDK tool instances, launcher, launcher-time proto-graph/root/seed, and launched composition are separate capability graph environments.
- Launcher/SDK graphs are static-only/read-only/hardcoded for now; launched compositions may expose APIs for mutable substrate over an immutable startup graph core.
- `steam-like-platform-contracts` is far-future pressure only, not Phase 3 scope.
- Spacetime Engine is the first-party engine/framework product.
- USF is a public/API-facing Spacetime Engine subsystem/module, not a product pillar.
- Loo Cast Game is currently represented by `base_mod`; Loo Cast Product/Project is the broader acquired bundle.
- Phase 2 is now framed as `Phase 2: Vapor + USF Planning and Spec Crystallization`; no new coding.
- Phase 2 must produce both Phase 3 and Phase 4 execution specs before Phase 3 coding begins.
- Phase 3 is now framed as `Phase 3: Vapor Launcher and Product-Stack Proof`.
- Phase 3 should prove the Vapor launcher/product-stack/modpack/capability system through a non-USF hello-world-on-steroids proof.
- Phase 3 means full public Vapor completeness for the claimed public seams, while still allowing performance, polish, marketplace scale, and public-hardening to defer.
- Phase 3 includes real launcher, real Steam/Workshop auth/download/upload, real Rhai/capability/modpack semantics, diagnostics, and multiple proof configurations.
- Phase 3 acceptance should be a Vapor scenario/integration-test suite centered on hello-world-on-steroids fixtures, composition permutations, and separate mostly-manual Steam flow checks.
- Phase 3 excludes USF, worldmodel, rendering, save/load, chunks, metrics, phenomena, zones, and traditional gameplay/simulation.
- Phase 4 is now framed as `Phase 4: Product-Stack + USF Prototype/MVP`.
- Phase 4 is Phase 3 plus the first full working USF stack/prototype/MVP.
- `Packagepack` is the selected name for the complete user/modpack-author-facing launch composition.
- Unqualified `package` is retired from active planning unless scoped as Steam package, [[Source Artifact]],
  [[Build Artifact]], [[Distributable Artifact]], or [[Packagepack]].
- `core_engine`, `core_mod`, and `base_mod` are owner-confirmed as literal required Rust crate names for the reserved roles, not only abstract labels.
- `core_engine` plus matching `core_mod` are bundled as an Engine fixture; independent `core_mod` replacement is forbidden for Phase 3.
- `base_mod` is the literal Game crate/artifact and declares required Engine/core_mod identity/version constraints.
- Current pack vocabulary: `packagepack`, `enginepack`, `gamepack`, and `modpack`; `Extension Mod` is the likely generic mod-of-a-mod term.
- Runtime graph construction is staged: discovered artifact graph -> user/modpack projection -> shallow metadata pre-validation -> capability expansion -> deep validation -> Runtime Lock -> runtime graph.
- Rhai topology direction: one Rhai file equals one authored leaf capability/asset node; folder-level Rhai aggregators are allowed; file-internal capability definitions are private/internal by default.
- Use lockstep game/contract versioning.
- Treat publish as immutable tag + artifact + channel event.

Current scope:

- Primary tasks: `setup_sdk`, `clean_sdk`, `build`, `package`, `run`, `audit`, `cloc`, `gource`.
- `develop` is the active integration line; scoped topic branches merge into it through PRs.
- `main` is protected by ruleset; `develop` is intentionally process-enforced without a ruleset for solo integration.
- Phase 0 workflow/process bootstrap is complete as of 2026-05-03.
- GitHub labels are live repository metadata. There is no committed label manifest.
- Required labels are applied manually by policy.
- Local hooks are the first validation rail: pre-commit formats, pre-push audits.
- GitHub Actions mirrors `cargo xtask audit` as a low-maintenance remote validation rail.
- No SDK/toolchain redistribution layer right now.
- No dual-repo automation right now.
- Compatibility policy is still defined in `CONTRACTS.md`, but the active conceptual surface is the glossary.
- `CHANGELOG_DRAFT.md` and `MIGRATIONS_DRAFT.md` are active draft surfaces while structural churn is expected.
- Stable-contract mode rule: breaking changes require a new published version + formal migration guide.

Success for this phase:

- Clean developer workflow with low cognitive load.
- Clear current vocabulary around Vapor, Spacetime Engine, USF, Loo Cast, capabilities, metrics, phenomena, chunks, and Rhai assets.
- Old USF-as-product and core_mod/base_mod mandatory-vs-replaceable confusion removed from active docs.
- Phase 2/3/4 roadmap reframed around planning first, Vapor product-stack proof second, and product-stack-plus-USF proof third.
