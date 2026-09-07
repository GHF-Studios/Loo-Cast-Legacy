Title =>

Phase 3: Vapor Launcher and Product-Stack Proof

Due date =>

Gate-based (unlocked only after Phase 2 gate issue decision note).

Description =>

Authority note:
This milestone is a lightweight phase container.
The Phase 3 tracking issue is the living coordination anchor while the phase is open.
The gate issue is the final exit decision record.
The execution spec is `loo_cast_alpha/docs/RFCS/phase_3_vapor_execution_spec.md`.

Purpose:
Implement a real Vapor platform slice proving SDK, launcher, Steam/Workshop distribution, Packagepack composition,
Capability/Rhai substrate, diagnostics, fingerprints, and launch handoff through hello-world-on-steroids fixtures.

Summary scope:

- [ ] Vapor workspace/crate foundation is implemented
- [ ] Vapor.toml, Vapor.lock, fingerprints, packs, and artifact data models are implemented
- [ ] Capability graph staging, validation, visibility, projections, and Runtime Lock are implemented
- [ ] Rhai declaration loading/validation and focused callback proof are implemented
- [ ] SDK commands and launcher modes exist and share validation semantics
- [ ] Real Steam/Workshop publish, update, download, install, enable, disable, and uninstall flows are verified
- [ ] Product-stack fixtures prove Engine/Game/mod/modpack composition without USF/worldmodel scope
- [ ] Scenario suite covers valid and invalid Vapor platform cases

Exit summary:

- [ ] Valid Packagepack lifecycle works end to end
- [ ] Invalid Packagepack lifecycle fails before launch with projected diagnostics
- [ ] Steam/manual verification notes are recorded
- [ ] Phase 3.5 and Phase 4 deferrals are explicit
- [ ] Gate issue records final Phase 3 decision and unlocks Phase 4

Tracking linkage:

- Phase tracking issue: TBD (issue from `.github/ISSUE_TEMPLATE/phase_tracking_issue.yml`)
- Gate issue: TBD (issue from `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)
