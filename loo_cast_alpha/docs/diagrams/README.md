# Diagrams

This directory is reset to a single root overview before rebuilding any child diagram set.

- [overview.puml](overview.puml) is the only active architecture diagram.
- [_theme.puml](_theme.puml) defines the local visual language for the current overview-map diagram type.
- Former child diagrams were removed to avoid preserving stale decomposition while the overview is still being tuned.
- Future diagrams should only be added after the overview establishes useful stable boundaries.

Current rules:

- Keep source readable before relying on Obsidian embeds, canvas layout, or viewer zoom.
- Every relationship arrow must have a clear label. Prefer labels that make `source + label + target` read like a sentence.
- Use visual types locally and explain them in the legend with color swatches plus text.
- Do not create a global diagram taxonomy unless it pays rent.
