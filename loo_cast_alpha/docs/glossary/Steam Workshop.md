---
canonical_name: Steam Workshop
status: WIP-draft
aliases:
  - Workshop
---

Steam Workshop is the mandatory public distribution surface for [[Vapor Ecosystem]] mods, modpacks, and related
published artifacts in Phase 3.

Current owner-answer-informed direction:

- Vapor should support upload/publish and update through launcher UI and CLI.
- Vapor should support download/install/update, enable/disable, and uninstall through launcher UI and CLI.
- Private/unlisted Workshop test uploads should be supported for author iteration.
- A Workshop item may act as the publication ID while Vapor revisions/fingerprints identify exact contents.
- Vapor metadata may be duplicated or projected into Workshop title, tags, description, and dependency fields where
  useful.
- Workshop item tags should include Vapor target role metadata such as engine mod, game mod, extension mod, enginepack,
  gamepack, modpack, or packagepack where practical.
- Modpacks should be publishable as Workshop items and may depend on other Workshop items.
- Nested modpacks should use both Workshop dependencies and Vapor metadata dependencies where useful.
- Downloaded Workshop content should be verified against Vapor fingerprints before use.
- Vapor should auto-subscribe to dependencies when SteamUGC permits it, but prompt for confirmation per Packagepack
  resolution when a detected dependency change would subscribe to new items.

Boundary:
Vapor should manage its own compatibility/version/fingerprint semantics rather than relying on Steam load order or Steam
depot/version semantics as the authoritative composition model.
The GitHub-backed registry idea is parked until the Steam-only flow is proven; it is Phase 3.5 pressure at most.

Phase 3 lock-candidate anchor:
The Workshop scope for Phase 3 is anchored by
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W07 and P3-W11.
Phase 3 requires upload/publish, update, private/unlisted test upload, subscribe/download/install, enable/disable,
uninstall, dependency detection, prompted auto-subscribe where SteamUGC permits it, offline behavior for already
installed content, and corrupted/incomplete download diagnostics.

#glossary
