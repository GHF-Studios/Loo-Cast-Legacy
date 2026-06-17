Superseded planning note (2026-06-17):
This draft is stale as an active Phase 3 scope source.
Owner answers in `loo_cast_alpha/docs/ai_conversation_logs/question_batch_004a.txt` reframe Phase 3 as
`Phase 3: Vapor Launcher and Product-Stack Proof`.
The old "minimum real USF runtime spine" premise is superseded.
Phase 3 is a Vapor launcher/product-stack/modpack/capability proof with real Steam/Workshop integration and no
USF/worldmodel implementation.
Do not use this draft as active issue text without rewriting it.

Title: Phase 3: Alpha Spine and Release Proof
Labels: type:phase-tracking, phase:3

Phase Name:
Phase 3: Alpha Spine and Release Proof

Phase Number:
3

Milestone Link:
https://github.com/OWNER/REPO/milestone/3

Owner / Final Decider:
@leslieghf

Authority:
This tracking issue is the living authority for Phase 3 while the phase is open. The milestone is only a lightweight
summary. The gate issue is the canonical final exit decision.

Purpose:
Implement the minimum real USF runtime spine in `loo_cast_alpha`, prove core runtime claims, and prove the release smoke
path.

Entry Criteria:

- [ ] Phase 2 gate issue decision note linked in Phase 2 tracking issue

Scope:

- [ ] Typed entrypoint contract is implemented and documented (inputs, outputs, failure modes)
- [ ] Deterministic bootstrap sequence is implemented and documented step-by-step
- [ ] Canonical vs derived state boundary is implemented for the minimal spine path, including cache-rebuild rule
- [ ] Initial capability bridge set is explicit and implemented (named channel families only; no "misc" bucket)
- [ ] Determinism scenarios are defined and executed for the minimal spine
- [ ] Observability baseline is implemented for bootstrap/runtime/contract failures
- [ ] Release smoke path runbook is completed (tagging, artifact build, Steam dry-run checklist, rollback steps)
- [ ] One full release smoke run is executed with linked evidence
- [ ] Phase 3 tracking issue has linked child issues

Child Issue Buckets:

- [ ] P3-T01: Typed entrypoint contract
- [ ] P3-T02: Deterministic bootstrap sequence
- [ ] P3-T03: Canonical vs derived state boundary
- [ ] P3-T04: Initial named capability bridge set
- [ ] P3-T05: Determinism scenarios and evidence
- [ ] P3-T06: Observability baseline
- [ ] P3-T07: Release smoke path runbook
- [ ] P3-T08: Release smoke run evidence and Phase 4 gap conversion

Out of Scope:

- Broad feature-parity migration
- Broad capability surface completion
- Final SDK/API exposure decisions

Done Means:

- [ ] Minimal USF spine boots and runs in alpha without ad-hoc/manual patch steps
- [ ] Determinism scenarios pass with linked run evidence (test output or reproducible script output)
- [ ] Release smoke run is reproducible from runbook with linked artifacts
- [ ] Known gaps are converted to explicit Phase 4 issues with owners and acceptance checklists
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Linked Child Issues:

- [ ] (add links after Phase 3 child issue creation)

Exit Evidence Packet:

- [ ] Tracking issue link present
- [ ] All closed child issue links present
- [ ] Implementation artifact links present (PR/commit)
- [ ] Validation artifact links present (CI/tests/benchmarks when required)
- [ ] Documentation update links present
- [ ] Gate issue linked (`phase_gate_issue.yml`)

Gate Issue Link:
TBD (create when Phase 3 starts)

Gate Note Mirror (Non-Canonical):
TBD at phase closure.
