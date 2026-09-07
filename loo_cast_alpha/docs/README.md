# Loo-Cast Docs

This directory is the canonical docs surface for active `loo_cast_alpha/` work.
If you arrived from repository root, use this file as the docs map and read-order anchor.

## Read Order

1. [NOW.md](NOW.md) (current truth and active scope)
2. [ARCHITECTURE.md](ARCHITECTURE.md) (layer boundaries and ownership cadence)
3. [CONTRACTS.md](CONTRACTS.md) (compatibility/version/migration policy)
4. [WORKFLOWS.md](WORKFLOWS.md) (daily loop, phase rules, and GitHub process policy)
5. [AI_COLLABORATION.md](AI_COLLABORATION.md) (supervised AI workflow prompt and gate sequence)
6. [DECISIONS.md](DECISIONS.md) (durable policy decisions and rationale)
7. [RFCS/README.md](RFCS/README.md) (phase/program-level strategy and design rationale)
8. [CHANGELOG_DRAFT.md](CHANGELOG_DRAFT.md) (pre-stable release-note drafting while structure is still shifting)
9. [MIGRATIONS_DRAFT.md](MIGRATIONS_DRAFT.md) (pre-stable migration-impact drafting while contracts are still shifting)
10. [migrations/README.md](migrations/README.md) (formal migration-guide location/rules once stable-contract mode is active)
11. [RFCS/phase_3_vapor_execution_spec.md](RFCS/phase_3_vapor_execution_spec.md) (Phase 3 lock-candidate execution spec)
12. [RFCS/phase_2_to_11_execution_program.md](RFCS/phase_2_to_11_execution_program.md) (consolidated roadmap drafting surface for Phases 2..11 and viewpoint infodumps)
13. [glossary-summary.md](glossary-summary.md) and [tech-glossary-summary.md](tech-glossary-summary.md) (canonical page-by-page review summaries for glossary and tech-glossary pages)
14. [glossary/](glossary/) (concept + technical glossary terms in one folder; split by `#glossary` and `#tech_glossary`
    tags)
15. [diagrams/](diagrams/) (working PlantUML architecture/governance/runtime diagram set)

## Entry By Intent

- I want current direction: [NOW.md](NOW.md)
- I want to build/package/run/audit: [WORKFLOWS.md](WORKFLOWS.md)
- I want compatibility guarantees: [CONTRACTS.md](CONTRACTS.md)
- I want architecture boundaries: [ARCHITECTURE.md](ARCHITECTURE.md)
- I want policy history: [DECISIONS.md](DECISIONS.md)
- I want phase/program execution strategy: [RFCS/README.md](RFCS/README.md)
- I want the current Phase 3 execution anchor: [RFCS/phase_3_vapor_execution_spec.md](RFCS/phase_3_vapor_execution_spec.md)
- I want consolidated Phase 2..11 roadmap drafting + viewpoint infodump capture: [RFCS/phase_2_to_11_execution_program.md](RFCS/phase_2_to_11_execution_program.md)
- I want glossary review summaries: [glossary-summary.md](glossary-summary.md), [tech-glossary-summary.md](tech-glossary-summary.md)
- I want concept terms and semantic relationships: [glossary/](glossary/) (`#glossary`)
- I want implementation-heavy technical term notes: [glossary/](glossary/) (`#tech_glossary`)
- I want the current architecture/governance/runtime diagrams: [diagrams/](diagrams/)
- I want draft release/migration notes during structural churn: [CHANGELOG_DRAFT.md](CHANGELOG_DRAFT.md), [MIGRATIONS_DRAFT.md](MIGRATIONS_DRAFT.md)
- I want crate docs posture and known deferrals: [RUSTDOC_BASELINE.md](RUSTDOC_BASELINE.md)

## Rust Docs Baseline

[RUSTDOC_BASELINE.md](RUSTDOC_BASELINE.md) inventories active crates and marks each as:

- documented and sufficient for current bootstrap scope, or
- intentionally minimal with rationale and follow-up routing.

This avoids silent documentation gaps while preserving Phase 0 scope discipline.
