---
canonical_name: Vapor Launcher
status: WIP-draft
aliases:
  - Launcher
---

The Vapor Launcher is the user/modpack-author/developer-facing UI surface for browsing, validating, composing,
installing, launching, and diagnosing [[Vapor Ecosystem]] content.

Current owner-answer-informed direction:

- The launcher should have explicit Player Mode, Modpack Author Mode, and Developer Mode layers.
- Player Mode should hide capability graph internals almost completely.
- Player Mode presents [[Packagepack]]s as launchable objects, but does not allow meaningful structural editing beyond
  surface-level launch options.
- Modpack Author Mode should expose packagepack/modpack composition, public capability paths, and compatibility
  diagnostics without raw implementation internals.
- Modpack Author Mode owns normal Packagepack creation/editing UX.
- Developer Mode should expose full graph paths, raw metadata, artifact roots, Steam IDs, fingerprints, and validation
  traces where useful.
- SDK CLI commands should exist before launcher UI, but a full-stack launcher remains part of Phase 3 acceptance.
- Launcher validation logs and launched process logs should be separate panes/files from day one.
- The launcher should maintain an install ledger, including local dev content.
- The launcher should discover workspace folders and installed artifacts through a simple working index at first.
- The install ledger should use plain TOML/lock-style files for now, not a database.
- Player Mode and Modpack Author Mode both still require validatable graphs; Modpack Author Mode is about editing
  pack-like objects, not bypassing validation.

Boundary:
Launcher and SDK should be sibling surfaces over shared `vapor_core` semantics.
Launcher operations should be capabilities where that gives useful encapsulation and modularity.

Phase 3 lock-candidate anchor:
The launcher scope for Phase 3 is anchored by [Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md),
especially P3-W06.
Phase 3 requires a real launcher shell, not only CLI commands: content browsing, Packagepack composition, validation
diagnostics, launcher logs, launched-process logs, Steam/Workshop actions, and launch of valid Packagepacks are all in
scope.

#glossary
