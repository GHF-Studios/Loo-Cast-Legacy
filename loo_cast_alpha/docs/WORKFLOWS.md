# Workflows

Daily developer loop:

Run `cargo xtask ...` from repository root (via root alias shim) or from `loo_cast_alpha/` workspace root.

1. `cargo xtask setup_sdk` once per clone.
2. `cargo xtask build`
3. `cargo xtask package`
4. `cargo xtask run`

Support tools:

- `cargo xtask audit`
- `cargo xtask clean_sdk`
- `cargo xtask cloc`
- `cargo xtask gource`

xtask command-surface policy:

1. Default developer tasks should be parameterless (`cargo xtask <named-task>`).
2. Prefer explicit task names for variants (`build_release`, `package_windows_release`) over flag matrices.
3. Avoid introducing argument combinations that create implicit invalid states.
4. Use arguments only for exceptional internal tooling cases, and document those cases explicitly.

Third-party utility binaries:

- `cargo xtask cloc` uses bundled `cloc` binaries from `loo_cast_alpha/third_party/cloc/`.
- Keep bundled third-party utilities out of alpha workspace root so ownership and provenance stay explicit.

Local validation rails:

1. `cargo xtask setup_sdk` installs git hooks.
2. `pre-commit` runs `cargo fmt --manifest-path loo_cast_alpha/Cargo.toml --all` and writes formatting changes to disk.
3. `pre-push` runs `cargo run --manifest-path loo_cast_alpha/Cargo.toml -p xtask -- audit`.
4. `cargo xtask audit` checks workspace formatting, lints workspace targets with `--no-deps`, and runs workspace
   library/binary tests.
5. `cargo xtask clean_sdk` removes only managed `setup_sdk` hooks and leaves non-managed custom hook content untouched.

GitHub Actions audit rail:

1. `.github/workflows/audit.yml` runs `cargo xtask audit` on pull requests, pushes to `develop`, pushes to `main`,
   and manual dispatch.
2. The workflow uses standard Linux GitHub-hosted runners only and uploads no artifacts.
3. The workflow restores Cargo registry, Cargo git, and `target/` caches before audit. The runner VM itself is
   disposable; only explicit caches survive between runs.
4. Cache hits can reuse Cargo downloads and compatible build/incremental artifacts, but source/toolchain/config changes
   can still trigger recompilation.
5. For zero-cost mode, keep the repository public or attach a self-hosted runner before relying on private-repository
   workflow runs.
6. Local hooks are the first guardrail; GitHub Actions is secondary parity validation.

Mod author loop (current):

1. Add a crate under `crates/`.
2. Mark mod crate with `.loo_cast_mod`.
3. Implement mod code against public mod API.
4. Build/package/run through xtask.

Composition loop:

1. Add two mods with separate APIs.
2. Add one integration mod that depends on both.
3. Verify build order and runtime compatibility.

Contract-safe change flow:

1. Change engine/mod code without breaking `CONTRACTS.md`.
2. Let pre-commit format changed Rust files.
3. Run `cargo xtask build`, `cargo xtask package`, `cargo xtask run`.
4. Run `cargo xtask audit` before pushing.
5. Update docs only if behavior/workflow changed.

Breaking change flow:

1. Bump target published version per `CONTRACTS.md`.
2. Update affected contract definitions.
3. During structural-churn phase, record migration-impact notes in `docs/MIGRATIONS_DRAFT.md`.
4. Once stable-contract mode is active, add formal migration guide at `docs/migrations/<from>-to-<to>.md`.
5. Record release-note intent in `docs/CHANGELOG_DRAFT.md`.
6. When stable-contract mode is active, promote draft release notes into the formal changelog process.
7. Draft/update RFC in `docs/RFCS/` for design rationale.
8. Record decision in `DECISIONS.md`.
9. Validate build/package/run before release.

Publish flow (stable):

1. Merge release-ready changes to `main`.
2. Create tag `vX.Y.Z`.
3. Build immutable artifacts from that tag.
4. Push artifacts to stable channel.

Publish flow (pre-release):

1. Merge release-candidate changes to `main`.
2. Create tag `vX.Y.Z-rc.N` (or `-beta.N`).
3. Build immutable artifacts from that tag.
4. Push artifacts to non-stable channel.

Repository branch roles:

1. `main` is the stable/release line.
2. `develop` is the integration line for active alpha work.
3. Topic branches carry reviewable work into `develop`.
4. Do not treat `develop` as a scratch branch.

Work modes:

1. Phase-managed work is the default for planned development.
2. Keep a single active execution stream: one active phase branch + one active phase PR.
3. Do not run parallel topic/feature PRs while a phase stream is active.
4. Simple incidental fixes can be applied immediately inside the active stream.
5. Complex unrelated fixes run as standalone fix-pass work.
6. Every commit needs clear intent.
7. Do not lock commit intent into a fixed prefix taxonomy until the repo has enough examples to justify one.

Phase-managed work:

1. Phase-managed work requires:
   - milestone
   - phase task issues
   - branch and pull request for changes that merge into `develop`
   - recorded evidence
   - completion checks before closure
2. Phase branches do not need to map to product features. They may represent docs alignment, workflow cleanup, contract
   adjustment, architecture settlement, or another bounded slice of the evolving framework.
3. Milestone descriptions are the active phase authority (intent, scope, out-of-scope, and exit criteria).
4. Phase issues carry concrete task execution context and should stay concise.
5. Active phase execution is intentionally monolithic at this stage: use one composite phase PR as the integration
   container for phase issues and evidence.
6. Later phases use the current baseline unless a new decision changes it.

Fix handling:

1. Simple incidental fixes discovered during active phase work should be applied immediately in the active phase stream.
2. For simple incidental fixes:
   - keep the fix trace in the commit title/body
   - mention it once in the active phase PR notes/progress update
3. Complex fixes that are unrelated to the active phase scope should use a standalone fix issue and dedicated fix-pass PR.
4. Fix-pass branches should use `fixpass/<slug>` naming and target `develop`.
5. While a fix-pass PR is active, pause new phase implementation changes.
6. After fix-pass merge, sync the active phase branch with `develop` before continuing phase work.

GitHub phase workflow (built-in/free features):

1. Milestones are the phase authority surface. Use `.github/MILESTONE_TEMPLATE/phase_milestone.md` as copy/paste source
   when creating or editing a milestone.
2. Issue creation uses focused issue forms in `.github/ISSUE_TEMPLATE/`:
   - `phase_task.yml` for all phase task issues
   - `maintenance_fix.yml` for standalone complex fix-pass issues
   - blank issues are disabled for non-maintainers via `.github/ISSUE_TEMPLATE/config.yml`
3. The issue form auto-applies `type:phase-task`.
4. Apply matching `phase:N` labels manually until metadata automation exists.
5. Labels are live GitHub repository metadata, not a committed manifest.
6. PRs use one template: `.github/PULL_REQUEST_TEMPLATE.md`.
7. Link related issues in the PR sidebar (`Development`) instead of listing issue numbers in the PR body.
8. Linked issues should be closed before PR merge.
9. Metadata automation is tracked separately and can evolve this flow later.

CODEOWNERS:

1. `.github/CODEOWNERS` is the review-ownership record for workflow governance, contract docs, and first-party mod
   platform surface.
2. The default owner is `@Leslieghf`.
3. Workflow/template changes, contract-policy docs, architecture/decision docs, current-state docs, AI collaboration
   prompts, and first-party mod crates are explicitly owned.

Branch protection and rulesets:

1. `main` is protected by a GitHub ruleset.
2. `develop` is the active integration line. It intentionally has no ruleset while this is a solo-development repo.
3. `develop` branch+PR discipline is process-enforced rather than ruleset-enforced:
   - use topic branches for reviewable work
   - open PRs into `develop`
   - record issue relationships and validation evidence in the PR
4. Revisit a `develop` ruleset when collaborators need enforced review, required status checks, required linear history,
   or direct-push prevention.

Pull request template workflow:

1. Phase and general PRs use `.github/PULL_REQUEST_TEMPLATE.md`.
2. Dedicated complex fix-pass PRs use `.github/PULL_REQUEST_TEMPLATE/fixing_phase.md`.
3. Keep one active phase PR as the composite phase execution container.
4. Keep at most one active fix-pass PR while phase execution is paused for the fix-pass.
5. Link related issues in the PR sidebar (`Development`).
6. Keep PR body content concise: summary, validation, and optional notes.
7. Close linked issues before merging.
8. If a PR is intentionally closed without merge (for example, superseded, abandoned, or replaced during branch rename),
   leave a closing comment that states what happened and links to the replacement PR/issue/branch when one exists.
9. PR supersession/migration protocol (when replacing an open PR):
   - open the replacement PR first
   - carry forward summary/evidence/checklist context
   - apply equivalent metadata (milestone/labels/linked issues)
   - comment on the old PR with reason + replacement link
   - close old PR without merge
   - update milestone/RFC/docs references to the replacement PR
   - verify only one active phase PR remains

AI collaboration workflow:

1. Use `AI_COLLABORATION.md` as the conversation starter for supervised AI work.
2. AI-assisted changes should start from a named GitHub issue or PR.
3. Approval gates are required before local inspection, editing, validation, and PR creation/update.
4. For multi-line GitHub CLI content (issue/PR bodies, comments, code/text blocks), use temporary files with `--body-file`/`-F` instead of escaped newline strings.

RFC trigger rule (minimum):

1. RFC required for contract changes.
2. RFC required for crate-boundary policy changes.
3. RFC required for major USF runtime model changes.
4. RFC required for irreversible migration decisions.
5. RFC optional for local refactors that do not alter contracts or boundaries.
