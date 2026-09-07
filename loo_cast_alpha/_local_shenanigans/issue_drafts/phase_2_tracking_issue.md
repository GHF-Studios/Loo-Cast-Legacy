Superseded planning note (2026-06-17):
This draft is stale as an active Phase 2 scope source.
Owner answers in `loo_cast_alpha/docs/ai_conversation_logs/question_batch_004a.txt` reframe Phase 2 as
`Phase 2: Vapor + USF Planning and Spec Crystallization`.
Do not use this draft as active issue text without rewriting it around question batches, glossary integration, and Phase
3/Phase 4 execution specs.

Title: Phase 2: Legacy Truth Map and USF/SDK Executable Plan
Labels: type:phase-tracking, phase:2

Phase Name:
Phase 2: Legacy Truth Map and USF/SDK Executable Plan

Phase Number:
2

Milestone Link:
https://github.com/OWNER/REPO/milestone/2

Owner / Final Decider:
@leslieghf

Authority:
This tracking issue is the living coordination anchor for Phase 2 while the phase is open. The milestone is only a
lightweight summary. The gate issue is the final exit decision record.

Purpose:
Remove legacy ambiguity and convert USF/SDK complexity into an executable migration plan before restoration
implementation.

Entry Criteria:

- [ ] Phase 1 gate issue decision note linked in Phase 1 tracking issue

Scope:

- [ ] Explicit in-scope legacy corpus list is defined for this phase (path list + doc types), starting from
  `loo_cast_alpha/_local_shenanigans/legacy_corpus_register.md`
- [ ] Legacy generation model is applied: `loo_cast_alpha/` is current target, direct `loo_cast_legacy/` material is
  previous-generation input, nested/marked legacy material is older archive input by default
- [ ] Legacy corpus exclusions are explicit: Unity-era build outputs, binaries/generated artifacts, and lore/narrative
  docs unless directly needed for runtime/content contract work
- [ ] Each in-scope legacy doc is tagged as `normative`, `historical`, `speculative`, or `deprecated`
- [ ] Contradiction register exists with status per item: `resolved`, `deferred`, or `superseded`
- [ ] Canonical authority map exists for compile/build, bootstrap/startup, runtime state, scripting surface, save/load
  semantics, and platform integration
- [ ] USF uncertainty register exists with severity (`blocker`, `high`, `medium`, `low`), next action, and target
  decision checkpoint
- [ ] USF capability inventory is captured with stable slice IDs
- [ ] Scripting migration inventory prioritizes Rhai scripts/surfaces first and Lua scripts secondarily
- [ ] Dependency graph and migration order are documented with rationale
- [ ] Risk class and acceptance checklist template exist per slice (`core-spine`, `contract-sensitive`,
  `perf-sensitive`, `integration-sensitive`)
- [ ] SDK/API candidate surface table exists with status per item (`keep-public`, `internalize`, `defer`)
- [ ] RFC trigger rules for USF/SDK contract deltas are documented in workflow docs
- [ ] Phase 2 tracking issue has linked child issues

Working Artifacts:

- [ ] `loo_cast_alpha/_local_shenanigans/legacy_corpus_register.md`
- [ ] Contradiction register
- [ ] Canonical authority map
- [ ] USF uncertainty register
- [ ] USF capability inventory with stable slice IDs
- [ ] Dependency graph and migration order
- [ ] Risk class and acceptance checklist template per slice
- [ ] SDK/API candidate surface table

Corpus Rules:

- [ ] Current alpha docs/code are target-generation authority unless explicitly changed
- [ ] Direct `loo_cast_legacy/` Rust/Rhai-era docs/code are previous-generation inputs
- [ ] Nested/marked legacy paths (`legacy`, `LEGACY`, `*_legacy*`, `legacy_*`) are older archive inputs by default
- [ ] Rhai scripts, Rhai binding code, and Rhai-facing scripting docs are primary scripting inputs
- [ ] Lua scripts are secondary scripting inputs
- [ ] Unity-era build outputs are excluded by default
- [ ] Binaries and generated artifacts are excluded by default
- [ ] Lore, narrative, and prompt-history docs are excluded unless directly needed for runtime/content contract work

Child Issue Buckets:

- [ ] P2-T01: Define and tag in-scope legacy corpus
- [ ] P2-T02: Build contradiction register and resolve/defer/supersede items
- [ ] P2-T03: Build canonical authority map
- [ ] P2-T04: Build USF uncertainty register
- [ ] P2-T05: Inventory USF capabilities and stable slice IDs
- [ ] P2-T06: Define dependency graph and migration order
- [ ] P2-T07: Define risk classes and acceptance checklist templates
- [ ] P2-T08: Resolve SDK/API candidate table to Phase 3/4 inputs

Out of Scope:

- Runtime restoration execution
- Repository-wide decoupling implementation
- Final public contract freeze

Done Means:

- [ ] All in-scope legacy docs have trust tags and canonical pointers where applicable
- [ ] Every priority slice has dependency parents, risk class, and acceptance checklist
- [ ] First execution batch for Phase 3/4 is selected with no unresolved `blocker` unknowns
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Linked Child Issues:

- [ ] (add links after Phase 2 child issue creation)

Exit Evidence Packet:

- [ ] Tracking issue link present
- [ ] All closed child issue links present
- [ ] Implementation artifact links present (PR/commit)
- [ ] Validation artifact links present (CI/tests/benchmarks when required)
- [ ] Documentation update links present
- [ ] Gate issue linked (`phase_gate_issue.yml`)

Gate Issue Link:
TBD (create when Phase 2 starts)

Gate Note Mirror (Non-Canonical):
TBD at phase closure.
