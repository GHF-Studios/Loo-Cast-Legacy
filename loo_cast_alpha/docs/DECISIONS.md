# Decisions

Format: `Date | Status | Decision | Reason`

- 2026-04-28 | Active | Use one repo for now. | Reduce moving parts while core architecture settles.
- 2026-04-28 | Active | Keep xtask focused on build/package/run/cloc/gource. | Lower complexity and maintenance cost.
- 2026-04-28 | Active | Model `core_mod` and `base_mod` as first-party mods. | Keep engine lean and enable consistent mod layering.
- 2026-04-28 | Active | Prioritize composable mods (including integration mods). | Avoid monolithic total-conversion-only mod style.
- 2026-04-28 | Active | Use short, indexed docs (`NOW/ARCHITECTURE/CONTRACTS/WORKFLOWS/DECISIONS`). | Reduce documentation fragmentation.
- 2026-04-28 | Active | Use lockstep game/contract versioning. | Simplify compatibility reasoning.
- 2026-04-28 | Active | Require migration guides for breaking changes. | Make contract breaks explicit and reviewable.
- 2026-04-28 | Active | Define published version as immutable tag + artifacts + distribution channel. | Remove ambiguity between commits and releases.
- 2026-04-28 | Active | Use semver pre-release tags (`-rc.N`, `-beta.N`) for non-stable publishes. | Support staged release validation.
- 2026-04-28 | Active | Use CODEOWNERS for contracts and first-party mod surface. | Enforce ownership at review time.
- 2026-04-29 | Active | Use lightweight RFC workflow for major design changes. | Keep architecture decisions explicit
  before implementation.
- 2026-05-01 | Active | Use `main` as release line and `develop` as active integration line. | Keep release history distinct
  from organic alpha work.
- 2026-05-01 | Active | Add Phase 0 as the bootstrap process-stabilization phase. | Formalize workflow decisions before
  relying on the phase system for later alpha work.
- 2026-05-01 | Active | Treat `0.5.0` as alpha, `0.9.0` as beta, and `1.0.0` as stable commercial baseline. | Preserve
  project milestone meaning while acknowledging SemVer initial-development rules before `1.0.0`.
- 2026-05-01 | Active | Treat GitHub labels as live repository metadata, not a committed manifest. | The GitHub CLI makes
  live label inspection and edits practical; a stale manifest would add process weight without enough value.
- 2026-05-03 | Active | Keep issue/PR metadata handling manual and do not add dedicated metadata automation jobs for now. |
  Current manual flow is low-cost and clear enough for the active solo workflow; extra automation would add maintenance
  surface without enough return.
- 2026-05-17 | Active | Treat capability/API lifecycle as projection-driven fixed-point bootstrap over a host-global
  graph. | Keep script contexts safe and contextual while allowing dynamic topological composition across Rust host
  authority and Rhai-facing projected facades.
- 2026-05-01 | Active | Keep `develop` process-enforced without a ruleset for now. | Preserve solo workflow
  flexibility while topic branches, PRs, evidence, and GitHub Actions already provide enough discipline.
