Title: Phase 5: Boundary Hardening and SDK/API Finalization
Labels: type:phase-tracking, phase:5

Phase Name:
Phase 5: Boundary Hardening and SDK/API Finalization

Phase Number:
5

Milestone Link:
https://github.com/OWNER/REPO/milestone/5

Owner / Final Decider:
@leslieghf

Authority:
This tracking issue is the living coordination anchor for Phase 5 while the phase is open. The milestone is only a
lightweight summary. The gate issue is the final exit decision record.

Purpose:
Finish architecture hardening by enforcing boundaries in automation, closing planned decoupling work, and finalizing
SDK/API contract posture.

Entry Criteria:

- [ ] Phase 4 gate issue decision note linked in Phase 4 tracking issue

Scope:

- [ ] Crate-boundary policy is encoded in automated checks that run in CI
- [ ] Cross-system seams targeted for decoupling are listed explicitly and tracked to closure
- [ ] SDK/API candidate table from Phase 2 is fully resolved (`keep-public` or `internalize`; no unresolved `defer`)
- [ ] Documentation requirements are enforced for in-scope migrated crates (public module docs + workflow docs
  alignment)
- [ ] Outstanding Phase 4 debt is reconciled (closed or explicitly deferred with rationale)
- [ ] Phase 5 tracking issue has linked child issues

Hardening Registers:

- [ ] Boundary policy check list
- [ ] Cross-system decoupling target list
- [ ] SDK/API candidate resolution table
- [ ] Outstanding Phase 4 debt reconciliation table

Child Issue Buckets:

- [ ] P5-T01: Automated crate-boundary policy checks
- [ ] P5-T02: Cross-system decoupling closure
- [ ] P5-T03: SDK/API candidate table final resolution
- [ ] P5-T04: Documentation enforcement for migrated crates
- [ ] P5-T05: Phase 4 debt reconciliation

Out of Scope:

- New large feature work unrelated to architecture/contract hardening

Done Means:

- [ ] Boundary violations are caught by automated checks (not only by manual review)
- [ ] Public SDK/API contract state is explicitly documented for downstream modding work
- [ ] In-scope docs and quality gates are green in CI
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Linked Child Issues:

- [ ] (add links after Phase 5 child issue creation)

Exit Evidence Packet:

- [ ] Tracking issue link present
- [ ] All closed child issue links present
- [ ] Implementation artifact links present (PR/commit)
- [ ] Validation artifact links present (CI/tests/benchmarks when required)
- [ ] Documentation update links present
- [ ] Gate issue linked (`phase_gate_issue.yml`)

Gate Issue Link:
TBD (create when Phase 5 starts)

Gate Note Mirror (Non-Canonical):
TBD at phase closure.
