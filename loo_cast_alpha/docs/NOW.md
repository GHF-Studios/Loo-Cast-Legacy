# NOW

Date: 2026-06-17

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
- Last integrated answered thread: `question_batch_004a.txt`; targeted glossary/roadmap checkpoint integration complete.
- Last answered raw thread: `question_batch_004a.txt`.
- Active question thread: none yet. Next likely follow-up: Rhai-side capability usage vs Rust-side capability kernel usage, hook/callback attachment semantics, and Phase 3 acceptance matrix minimization.
- Remove the old `WIP_QUESTION_LEDGER.txt`; question batches replace it.
- Product stack skeleton: `Vapor Ecosystem -> Spacetime Engine -> Loo Cast`.
- Vapor is Steam-exclusive and includes SDK/tooling plus runtime protocol/contracts/traits for engines, games, mods, packagepacks, modpacks, enginepacks, and gamepacks.
- Spacetime Engine is the first-party engine/framework product.
- USF is a public/API-facing Spacetime Engine subsystem/module, not a product pillar.
- Loo Cast Game is currently represented by `base_mod`; Loo Cast Product/Project is the broader acquired bundle.
- Phase 2 is now framed as `Phase 2: Vapor + USF Planning and Spec Crystallization`; no new coding.
- Phase 2 must produce both Phase 3 and Phase 4 execution specs before Phase 3 coding begins.
- Phase 3 is now framed as `Phase 3: Vapor Launcher and Product-Stack Proof`.
- Phase 3 should prove the Vapor launcher/product-stack/modpack/capability system through a non-USF hello-world-on-steroids proof.
- Phase 3 means full public Vapor completeness for the claimed public seams, while still allowing performance, polish, marketplace scale, and public-hardening to defer.
- Phase 3 includes real launcher, real Steam/Workshop auth/download/upload, real Rhai/capability/modpack semantics, diagnostics, and multiple proof configurations.
- Phase 3 excludes USF, worldmodel, rendering, save/load, chunks, metrics, phenomena, zones, and traditional gameplay/simulation.
- Phase 4 is now framed as `Phase 4: Product-Stack + USF Prototype/MVP`.
- Phase 4 is Phase 3 plus the first full working USF stack/prototype/MVP.
- `Packagepack` is the selected name for the complete user/modpack-author-facing launch composition.
- Unqualified `package` is retired from active planning unless scoped as Steam package, build artifact, source package, or runtime artifact.
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
