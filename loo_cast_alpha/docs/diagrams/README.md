# Diagrams

- A diagram node is a named `.puml` file.
- If a diagram expands into children, a same-named folder beside it contains the child diagrams.
- The diagram-set root starts at `overview.puml`.
- Do not use `_mod_.puml` for new diagrams.
- Zoom boxes link to the matching child `.puml` in rendered SVGs when the viewer supports PlantUML links.
- Keep diagrams short. One diagram should explain one thing.
- Grouping is allowed without claiming a strict relationship.
- Every relationship arrow must have a clear label. If it cannot be labeled, remove it.
- Prefer labels that make `source + label + target` read like a sentence.
- Prefer actor/process/phase/state/artifact node names over imperative action labels.
- Use visual types locally. Explain them in the legend; do not create a global taxonomy unless it actually pays rent.
- Legends should use colored swatches plus text labels so meaning is not text-only or color-only.
- Do not cram child-diagram detail into an edge label. If an edge needs a long explanation, make or use a child diagram.
- Keep the source diagram readable before Obsidian embeds or canvas layout. If a diagram turns into a long strip,
  aggregate sibling detail, change direction, or split the detail into a child diagram.
- Every diagram includes `_theme.puml` to keep the dark theme consistent.

Start at [overview.puml](overview.puml).

- [author_content.puml](author_content.puml)
  - [define_capability_module.puml](author_content/define_capability_module.puml)
    - [organize_module_files.puml](author_content/define_capability_module/organize_module_files.puml)
    - [declare_rhai_assets.puml](author_content/define_capability_module/declare_rhai_assets.puml)
    - [attach_native_backing.puml](author_content/define_capability_module/attach_native_backing.puml)
  - [build_authoring_artifact.puml](author_content/build_authoring_artifact.puml)
    - [validate_manifests.puml](author_content/build_authoring_artifact/validate_manifests.puml)
    - [build_rust_outputs.puml](author_content/build_authoring_artifact/build_rust_outputs.puml)
    - [pack_artifact.puml](author_content/build_authoring_artifact/pack_artifact.puml)
    - [write_fingerprints.puml](author_content/build_authoring_artifact/write_fingerprints.puml)
- [distribute_content.puml](distribute_content.puml)
  - [publish_workshop_item.puml](distribute_content/publish_workshop_item.puml)
  - [install_workshop_item.puml](distribute_content/install_workshop_item.puml)
- [compose_packagepack.puml](compose_packagepack.puml)
- [launch_packagepack.puml](launch_packagepack.puml)
  - [resolve_packagepack.puml](launch_packagepack/resolve_packagepack.puml)
    - [select_enginepack.puml](launch_packagepack/resolve_packagepack/select_enginepack.puml)
    - [check_dependencies.puml](launch_packagepack/resolve_packagepack/check_dependencies.puml)
  - [build_capability_graph.puml](launch_packagepack/build_capability_graph.puml)
  - [load_kernel_artifacts.puml](launch_packagepack/load_kernel_artifacts.puml)
  - [lock_runtime.puml](launch_packagepack/lock_runtime.puml)
- [run_locked_runtime.puml](run_locked_runtime.puml)
  - [tick_runtime.puml](run_locked_runtime/tick_runtime.puml)
  - [dispatch_callbacks.puml](run_locked_runtime/dispatch_callbacks.puml)
    - [build_cb_ctx.puml](run_locked_runtime/dispatch_callbacks/build_cb_ctx.puml)
    - [commit_apply.puml](run_locked_runtime/dispatch_callbacks/commit_apply.puml)
  - [reconcile_state.puml](run_locked_runtime/reconcile_state.puml)
