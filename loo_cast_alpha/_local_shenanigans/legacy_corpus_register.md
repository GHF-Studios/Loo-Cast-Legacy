# Legacy Corpus Register Draft

Purpose:
Define the Phase 2 legacy review corpus before restoration work starts.

Generation model:

- `loo_cast_alpha/`: current alpha source and documentation. This is the target generation.
- Direct children of `loo_cast_legacy/`: previous-generation Rust/Rhai-era material. Treat as deprecated but highly
  relevant until mapped.
- Paths under `loo_cast_legacy/` with names such as `legacy`, `LEGACY`, `*_legacy*`, or `legacy_*`: older-generation
  archive material. Treat as historical by default unless a Phase 2 decision promotes a specific item.
- Unity/Lua-era material usually lives in the older-generation archive bucket. It may still inform restoration, but it
  should not override Rust/Rhai-era material without an explicit authority-map decision.

Trust tags:

- `normative`: historical legacy-authority signal only; must still be reconciled against current owner answers and active
  glossary wording before promotion.
- `historical`: useful context, not binding.
- `speculative`: planning material that needs confirmation before use.
- `deprecated`: known stale or superseded material.

Default exclusions:

- Unity-era build outputs under `loo_cast_legacy/legacy/builds/`
- Binaries and generated artifacts
- Lore, narrative, and prompt-history documents unless directly needed for runtime/content contract work

Source priority:

1. Current alpha docs/code in `loo_cast_alpha/`
2. Previous-generation direct `loo_cast_legacy/` Rust/Rhai-era docs/code
3. Older-generation nested/marked legacy archive material

Scripting priority within legacy review:

1. Rhai scripts, Rhai binding code, and Rhai-facing scripting docs
2. Lua scripts from older-generation base-mod material
3. Other scripting notes only when they resolve a concrete USF/runtime uncertainty

| Path                                                     | Generation          | Type                        | Trust tag      | Relevant phase | Canonical pointer | Notes                                                                                    |
|----------------------------------------------------------|---------------------|-----------------------------|----------------|----------------|-------------------|------------------------------------------------------------------------------------------|
| `loo_cast_alpha/`                                        | current alpha       | docs/code                   | active/TBD     | Phase 1+       | TBD               | Target generation. Legacy material must be reconciled into this shape.                   |
| `loo_cast_legacy/documents/intention_records/`           | previous generation | diagram atlas               | TBD            | Phase 2        | TBD               | Candidate canonical legacy intent source; verify against alpha direction.                |
| `loo_cast_legacy/documents/markdown_summary/`            | previous generation | implementation summaries    | TBD            | Phase 2        | TBD               | Implementation-oriented summaries; align with diagram atlas and alpha docs.              |
| `loo_cast_legacy/documents/temp_stuff/`                  | previous generation | planning notes/quarantine   | TBD            | Phase 2        | TBD               | Treat as non-canonical until promoted or explicitly superseded.                          |
| `loo_cast_legacy/core_mod_api/src/backend/rhai_binding/` | previous generation | Rhai binding code           | TBD            | Phase 2/3      | TBD               | Primary scripting migration input.                                                       |
| `loo_cast_legacy/core_mod/assets/`                       | previous generation | Rhai/content assets         | TBD            | Phase 2/3      | TBD               | Inspect for active Rhai script profiles and content contracts.                           |
| `loo_cast_legacy/core_engine/`                           | previous generation | runtime code                | TBD            | Phase 2/3      | TBD               | Candidate runtime/bootstrap authority input.                                             |
| `loo_cast_legacy/core_mod/`                              | previous generation | first-party mod code/assets | TBD            | Phase 2/4      | TBD               | Candidate mod/content authority input.                                                   |
| `loo_cast_legacy/base_mod/`                              | previous generation | gameplay mod code/assets    | TBD            | Phase 2/4      | TBD               | Candidate gameplay restoration input.                                                    |
| `loo_cast_legacy/legacy/misc/base_mod/src/`              | older archive       | Lua scripts                 | historical/TBD | Phase 2/4      | TBD               | Secondary scripting input after Rhai surfaces.                                           |
| `loo_cast_legacy/legacy/srcs/`                           | older archive       | old Rust snapshots          | historical/TBD | Phase 2        | TBD               | Use only to resolve concrete runtime/model questions.                                    |
| `loo_cast_legacy/LEGACY/`                                | older archive       | Unity-era project/assets    | historical/TBD | Phase 2        | TBD               | Exclude by default unless a targeted content/runtime question requires it.               |
| `loo_cast_legacy/documents/legacy_stuff/`                | older archive       | historical notes            | historical/TBD | Phase 2        | TBD               | Exclude lore/narrative by default. Pull only targeted runtime/content contract evidence. |
