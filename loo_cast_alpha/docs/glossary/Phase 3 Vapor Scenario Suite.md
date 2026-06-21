---
canonical_name: Phase 3 Vapor Scenario Suite
status: WIP-draft
aliases:
  - Phase 3 Scenario Suite
---

The Phase 3 Vapor Scenario Suite is the planned scenario/integration-test suite for proving Vapor-level composition
semantics without entering USF/worldmodel/gameplay scope.

Current owner-answer-informed direction:

- It should be a scenario/integration-test suite, not a single command and not merely an abstract matrix.
- It should use sets of engines, games, mods, extension mods, engine mods, game mods, packagepacks, enginepacks,
  gamepacks, and modpacks.
- It should allow manual mix-and-match to verify that valid combinations work and invalid combinations error as
  expected.
- It should include hello-world-on-steroids fixtures: boring fixed-output programs whose output is parameterized by
  capability composition, contributed mods, and selected Packagepack fingerprint.
- Steam-specific flows should be tested separately and manually where needed; this suite primarily proves Vapor-level
  composition semantics.

Boundary:
The suite must not become gameplay, rendering, save/load, USF, or worldmodel proof.

#glossary
