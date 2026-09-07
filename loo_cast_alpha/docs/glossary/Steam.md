---
canonical_name: Steam
status: WIP-draft
aliases: []
---

Steam is the exclusive external identity, ownership, authorization, and distribution substrate for the
[[Vapor Ecosystem]] in the current plan.
Vapor is not storefront-agnostic for Phase 3.

Current owner-answer-informed direction:

- Steam account identity is mandatory for published/public Vapor usage.
- Local/offline authoring can exist without live Steam auth.
- Local/offline authoring identities may later bind to Steam identity.
- Steam-native ownership/auth flows should be used instead of custom account infrastructure.
- Steam-specific terms should be mirrored in Vapor docs where they matter, including AppID, PublishedFileId, UGC item,
  depot, build, branch, package, and Workshop item.
- If Steam is unavailable, public download/update should fail gracefully while local authoring and already-installed
  offline play/test can remain possible.

Future pressure:
[[steam-like-platform-contracts]] may be possible later, but it is not Phase 3 work and should not weaken the current
Steam-exclusive Vapor direction.

Phase 3 lock-candidate anchor:
The Steam scope for Phase 3 is anchored by [Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md),
especially P3-W07.
Phase 3 requires real Steam auth/identity, AppID ownership/entitlement checks for public/published Vapor usage, and
real Workshop-backed distribution flows.
Steam failures should become recoverable diagnostics where possible, not internal panics.

#glossary
