# Architecture

Status:
Working orientation only. The latest glossary and owner-answer question logs outrank this file where terminology
conflicts.

Current product/platform stack:

1. [[Vapor Ecosystem]]: Steam-exclusive SDK, launcher, distribution, identity, composition, validation, [[Capability]],
   and [[Rhai Asset]] substrate.
2. Generic [[Engine]] role: selected through an [[Enginepack]] as a coupled `core_engine` plus matching `core_mod`
   fixture.
3. Generic [[Game]] role: selected through a [[Gamepack]] / `base_mod` fixture compatible with the selected Engine.
4. [[Packagepack]]: complete launchable composition containing the selected Enginepack, Gamepack, compatible modpacks,
   mods, fingerprints, and lock/resolution data.

Current boundary rules:

- Phase 3 architecture work targets Vapor platform seams: SDK, launcher, Steam/Workshop integration, capability graph
  staging, Rhai declarations, [[Vapor.toml]], [[Vapor.lock]], diagnostics, fingerprints, and launch handoff.
- The active execution anchor for those seams is `RFCS/phase_3_vapor_execution_spec.md`.
- Concrete first-party engine/game internals and USF/worldmodel concepts are outside the current Vapor-focused pass.
- `core_engine`, matching `core_mod`, and `base_mod` are reserved built-in role names in a valid Vapor product instance
  stack.
- `core_engine` plus matching `core_mod` are bundled as the selected Engine fixture, not independently mixed and matched
  by ordinary mod selection.
- `base_mod` is the selected Game fixture.
- [[Capability]] is the foundational Vapor-visible orchestration/composition abstraction, not an engine-specific
  subsystem.
- [[Vapor.toml]] owns Cargo.toml-like package/build/composition metadata; [[Rhai Asset]] files declare capabilities.
- [[Runtime Lock]] applies to launchable Engine/Game runtime composition after successful validation, not to launcher or
  SDK authoring as a fully dynamic runtime.

Change governance:

- Compatibility/version/migration policy lives in `CONTRACTS.md`.
- Glossary pages are the active concept crystallization surface during Phase 2.
- RFCs and phase drafts are historical if they conflict with current glossary wording or owner-answer logs.
