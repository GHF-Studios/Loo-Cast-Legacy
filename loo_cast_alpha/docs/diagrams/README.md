# Diagrams

- A folder is a diagram node.
- `_mod_.puml` explains that node.
- A box expands when a matching child folder exists.
- Zoom boxes link to the matching child `_mod_.puml` in rendered SVGs when the viewer supports PlantUML links.
- Keep diagrams short. One diagram should explain one thing.
- Grouping is allowed without claiming a strict relationship.
- Every relationship arrow must have a clear label. If it cannot be labeled, remove it.
- Prefer labels that make `source + label + target` read like a sentence.
- Prefer actor/process/phase/state/artifact node names over imperative action labels.
- Use visual types locally. Explain them in the legend; do not create a global taxonomy unless it actually pays rent.
- Legends should use colored swatches plus text labels so meaning is not text-only or color-only.
- Do not cram child-diagram detail into an edge label. If an edge needs a long explanation, make or use a child folder.
- Keep the source diagram readable before Obsidian embeds or canvas layout. If a diagram turns into a long strip,
  aggregate sibling detail, change direction, or split the detail into a child folder.
- Every diagram includes `_theme.puml` to keep the dark theme consistent.

Start at [_mod_.puml](_mod_.puml).

- [author_content/](author_content/_mod_.puml)
  - [define_capability_module/](author_content/define_capability_module/_mod_.puml)
    - [organize_module_files/](author_content/define_capability_module/organize_module_files/_mod_.puml)
    - [declare_rhai_assets/](author_content/define_capability_module/declare_rhai_assets/_mod_.puml)
    - [attach_native_backing/](author_content/define_capability_module/attach_native_backing/_mod_.puml)
  - [build_authoring_artifact/](author_content/build_authoring_artifact/_mod_.puml)
    - [validate_manifests/](author_content/build_authoring_artifact/validate_manifests/_mod_.puml)
    - [build_rust_outputs/](author_content/build_authoring_artifact/build_rust_outputs/_mod_.puml)
    - [pack_artifact/](author_content/build_authoring_artifact/pack_artifact/_mod_.puml)
    - [write_fingerprints/](author_content/build_authoring_artifact/write_fingerprints/_mod_.puml)
- [distribute_content/](distribute_content/_mod_.puml)
  - [publish_workshop_item/](distribute_content/publish_workshop_item/_mod_.puml)
  - [install_workshop_item/](distribute_content/install_workshop_item/_mod_.puml)
- [compose_packagepack/](compose_packagepack/_mod_.puml)
- [launch_packagepack/](launch_packagepack/_mod_.puml)
  - [resolve_packagepack/](launch_packagepack/resolve_packagepack/_mod_.puml)
    - [select_enginepack/](launch_packagepack/resolve_packagepack/select_enginepack/_mod_.puml)
    - [check_dependencies/](launch_packagepack/resolve_packagepack/check_dependencies/_mod_.puml)
  - [build_capability_graph/](launch_packagepack/build_capability_graph/_mod_.puml)
  - [load_kernel_artifacts/](launch_packagepack/load_kernel_artifacts/_mod_.puml)
  - [lock_runtime/](launch_packagepack/lock_runtime/_mod_.puml)
- [run_locked_runtime/](run_locked_runtime/_mod_.puml)
  - [tick_runtime/](run_locked_runtime/tick_runtime/_mod_.puml)
  - [dispatch_callbacks/](run_locked_runtime/dispatch_callbacks/_mod_.puml)
    - [build_cb_ctx/](run_locked_runtime/dispatch_callbacks/build_cb_ctx/_mod_.puml)
    - [commit_apply/](run_locked_runtime/dispatch_callbacks/commit_apply/_mod_.puml)
  - [reconcile_state/](run_locked_runtime/reconcile_state/_mod_.puml)
