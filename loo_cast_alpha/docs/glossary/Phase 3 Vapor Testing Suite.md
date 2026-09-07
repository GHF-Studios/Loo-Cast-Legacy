---
canonical_name: Phase 3 Vapor Testing Suite
status: WIP-draft
aliases:
  - Phase 3 Vapor Scenario Suite
  - Phase 3 Scenario Suite
  - Phase 3 Testing Suite
---

The Phase 3 Vapor Testing Suite is the planned acceptance/testing surface for proving Vapor-level composition semantics
without entering USF/worldmodel/gameplay scope.
It is broader than an automated integration-test suite.
The active execution-spec anchor is `../RFCS/phase_3_vapor_execution_spec.md`.

Current owner-answer-informed direction:

- It should combine automated validation tests, local/manual scenario runs, and manual Steam/Workshop verification.
- It should not be reduced to one command, one token matrix, or CI-only integration tests.
- It should use sets of engines, games, mods, extension mods, engine mods, game mods, packagepacks, enginepacks,
  gamepacks, and modpacks.
- It should allow manual mix-and-match to verify that valid combinations work and invalid combinations error as
  expected.
- It should include hello-world-on-steroids fixtures: minimal real executable MVP fixtures whose boring fixed output is
  parameterized by capability composition, contributed mods, and selected Packagepack fingerprint.
- It should prove public/installable/authorable/publishable Vapor artifacts and Packagepack paths, not only logs or
  fingerprints.
- Steam-specific flows are part of Phase 3 testing, but their live verification may be manual and outside CI.
- Pure validation primitives should be automated where practical; Steam/Workshop flows should have manual verification
  records.
- CI must not be treated as proof of live Steam integration unless live Steam verification is explicitly configured.

Minimum testing classes:

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
Fixture output should stay non-gameplay: logs, strings, files, fingerprints, and diagnostics.
The execution-spec anchor for required scenarios is
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W11.

#glossary
