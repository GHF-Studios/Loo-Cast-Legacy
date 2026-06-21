Title: Phase 3: Vapor Launcher and Product-Stack Proof
Labels: type:phase-tracking, phase:3

Phase Name:
Phase 3: Vapor Launcher and Product-Stack Proof

Phase Number:
3

Milestone Link:
https://github.com/OWNER/REPO/milestone/3

Owner / Final Decider:
@leslieghf

Authority:
This tracking issue is the living coordination anchor for Phase 3 while the phase is open.
The milestone is only a lightweight container.
The gate issue is the final exit decision record.
The execution spec is `loo_cast_alpha/docs/RFCS/phase_3_vapor_execution_spec.md`.

Purpose:
Implement a real Vapor platform slice that proves SDK, launcher, Steam/Workshop, Packagepack composition,
Capability/Rhai substrate, diagnostics, fingerprints, and launch handoff through hello-world-on-steroids fixtures.
Phase 3 explicitly excludes USF, worldmodel, rendering, save/load, and traditional gameplay simulation.

Entry Criteria:

- [ ] Phase 2 gate issue decision note linked in Phase 2 tracking issue
- [ ] Phase 3 lock-candidate execution spec exists and has no known blocker contradiction
- [ ] Phase 4 execution spec exists at planning level before Phase 3 coding begins
- [ ] Current glossary pages for Vapor, Capability, Rhai, packs, Steam, SDK, Launcher, Runtime Lock, Vapor.toml,
      Vapor.lock, and Fingerprint are internally consistent enough for implementation

Scope:

- [ ] Vapor crate/workspace foundation is implemented
- [ ] Vapor.toml and Vapor.lock are implemented for Phase 3 data needs
- [ ] Packagepack, Enginepack, Gamepack, Modpack, Engine Mod, Game Mod, and Extension Mod data model is implemented
- [ ] Capability graph core supports staging, validation, visibility, projection, and Runtime Lock for launched
      compositions
- [ ] Rhai declaration loading/validation and focused callback proof are implemented
- [ ] SDK command surface exists for scaffold, validate, fingerprint, build/package, publish/update, and install/update
      workflows
- [ ] Launcher exists with Player Mode, Modpack Author Mode, Developer Mode, validation diagnostics, and launched-process
      logs
- [ ] Real Steam auth/identity/ownership and Workshop upload/update/subscribe/download/install/enable/disable/uninstall
      flows are implemented
- [ ] Hello-world-on-steroids Engine/Game/mod fixtures prove product-stack composition without USF
- [ ] Scenario suite covers valid and invalid Vapor platform cases
- [ ] Phase 3 docs and runbooks match implementation reality

Child Issue Buckets:

- [ ] P3-W01: Vapor workspace foundation
- [ ] P3-W02: Core data model and manifests
- [ ] P3-W03: Capability graph core
- [ ] P3-W04: Rhai declaration substrate
- [ ] P3-W05: SDK command surface
- [ ] P3-W06: Launcher modes and UX shell
- [ ] P3-W07: Steam and Workshop integration
- [ ] P3-W08: Product-stack fixtures
- [ ] P3-W09: Hello-world-on-steroids output
- [ ] P3-W10: Diagnostics and failure doctrine
- [ ] P3-W11: Scenario suite
- [ ] P3-W12: Documentation and phase close

Out of Scope:

- USF implementation
- Chunks, metrics, phenomena, zones, scales, worldmodel/entity APIs
- Save/load
- Rendering, audio, generated media, and traditional gameplay
- Storefront abstraction beyond Steam
- GitHub-backed registry
- Phase 3.5 quality-layer work unless explicitly pulled into Phase 3 by owner decision

Done Means:

- [ ] A valid local Packagepack can be created, validated, fingerprinted, installed, launched, and diagnosed through
      Vapor surfaces without Steam upload
- [ ] At least one valid publishable Packagepack path can be uploaded/updated through Steam Workshop, then downloaded,
      installed, enabled/disabled, uninstalled, and launched from installed content
- [ ] An invalid Packagepack is blocked before launch with appropriate diagnostics
- [ ] SDK and launcher share validation semantics
- [ ] Real Steam Workshop upload/update/subscribe/download/install/enable/disable/uninstall flows are verified
- [ ] Hello-world-on-steroids fixtures prove Engine/Game/mod/modpack/capability/Rhai composition without USF
- [ ] Scenario suite covers agreed valid and invalid cases
- [ ] Known gaps are converted to explicit Phase 3.5 or Phase 4 issues with rationale
- [ ] Phase exit evidence packet is complete

Linked Child Issues:

- [ ] (add links after Phase 3 child issue creation)

Exit Evidence Packet:

- [ ] Tracking issue link present
- [ ] All closed child issue links present
- [ ] Implementation artifact links present
- [ ] Validation artifact links present
- [ ] Documentation update links present
- [ ] Steam/manual verification notes present
- [ ] Gate issue linked (`phase_gate_issue.yml`)

Gate Issue Link:
TBD (create when Phase 3 starts)

Gate Note Mirror (Non-Canonical):
TBD at phase closure.
