---
canonical_name: Phase 3 Vapor Scenario Suite
status: WIP-draft
aliases:
  - Phase 3 Scenario Suite
---

The Phase 3 Vapor Scenario Suite is the planned scenario/integration-test suite for proving Vapor-level composition
semantics without entering USF/worldmodel/gameplay scope.
The active execution-spec anchor is `../RFCS/phase_3_vapor_execution_spec.md`.

Current owner-answer-informed direction:

- It should be a scenario/integration-test suite, not a single command and not merely an abstract matrix.
- It should use sets of engines, games, mods, extension mods, engine mods, game mods, packagepacks, enginepacks,
  gamepacks, and modpacks.
- It should allow manual mix-and-match to verify that valid combinations work and invalid combinations error as
  expected.
- It should include hello-world-on-steroids fixtures: boring fixed-output programs whose output is parameterized by
  capability composition, contributed mods, and selected Packagepack fingerprint.
- Steam-specific flows are part of the Phase 3 scenario suite, but their verification may be manual and outside CI.
- Pure validation primitives should be automated where practical; Steam flows should have manual verification records.

Minimum scenario classes:

- local-only authoring without Steam upload
- default unmodded Packagepack
- heavily modded Packagepack with Engine Mods and Game Mods
- Extension Mod stack
- nested Modpack stack
- alternative Engine fixture
- alternative Game fixture
- already-installed offline launch/test
- Steam upload/update/publish roundtrip
- Steam subscribe/download/install/update/enable/disable/uninstall roundtrip
- invalid dependency/version/conflict/cycle/provider/visibility/fingerprint/Rhai/Vapor.toml/download cases

Boundary:
The suite must not become gameplay, rendering, save/load, USF, or worldmodel proof.
The execution-spec anchor for required scenarios is
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W11.

#glossary
