# NOW

Date: 2026-06-16

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
- Active question thread: `question_batch_003.txt` focuses on capability/slot semantics.
- Remove the old `WIP_QUESTION_LEDGER.txt`; question batches replace it.
- Product stack skeleton: `Vapor Ecosystem -> Spacetime Engine -> Loo Cast`.
- Vapor is Steam-exclusive and includes SDK/tooling plus runtime protocol/contracts/traits for engines, games, mods, and packages.
- Spacetime Engine is the first-party engine/framework product.
- USF is a public/API-facing Spacetime Engine subsystem/module, not a product pillar.
- Loo Cast Game is currently represented by `base_mod`; Loo Cast Product/Project is the broader acquired bundle.
- Phase 2 focuses on glossary/docs/contract crystallization enough to avoid poisoning implementation.
- Phase 3 should prove `launcher -> Vapor package -> Spacetime Engine -> Loo Cast` as a product-stack MVP, not a USF MVP.
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
- Phase 2/3 roadmap reframed around product-stack proof before USF-specific implementation.
