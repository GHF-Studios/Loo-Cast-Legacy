# USF Instantiation Capability Slot Notes

#tech_glossary

Related glossary terms:

- [USF Instantiation Scripts](USF%20Instantiation%20Scripts.md)
- [USF Definition Lifecycle](USF%20Definition%20Lifecycle.md)
- [USF Instance Graph](USF%20Instance%20Graph.md)
- [Capability Slot Type](Capability%20Slot%20Type.md)
- [Callback Type](Callback%20Type.md)
- [Callback Context Type](Callback%20Context%20Type.md)
- [Callback Signature](Callback%20Signature.md)
- [Capability Declaration](Capability%20Declaration.md)
- [Capability](Capability.md)
- [Rhai Capability](Rhai%20Capability.md)
- [Capability Projection API](Capability%20Projection%20API.md)
- [Script Safety](Script%20Safety.md)
- [Capability Runtime](Capability%20Runtime.md)
- [Runtime Substrate](Runtime%20Substrate.md)
- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)

Current capability-slot direction notes (legacy MVP slice alignment):

1. Script file profiles are explicit (`scale`, `metric`, `phenomenon`, `phenomenon_realizer`).
2. Capability use is context-rooted and profile-gated.
3. Alias preprocessing (`use ... as ...`) is part of the script-loading flow.
4. Definition content is loaded, validated, and transitioned through Runtime Lock for runtime progression.
5. Each old "script profile" concept should now be re-read as some mix of [[Capability Type]], [[Capability Trait]],
   [[Capability Extension Slot]], and projection policy.
6. One script/file no longer defines one singleton-like [[Capability Declaration]] by active doctrine.
   A file is one typed [[Rhai Asset]] contributing to [[Capability Module]] / [[Capability Node]] material.
7. Rust-side host validation/materialization wiring constrains declarations for those type/trait/callback surfaces.
8. Capabilities in scripts are [[Rhai Capability]] dynamic API objects (human-readable string IDs), with profile/policy
   grant or deny access.
9. Executing script declaration code yields typed declaration/data material for a Capability Module or Capability Node,
   not necessarily one capability declaration.
10. API graph topology is hierarchical: atomic capability nodes plus composite/category nodes.
11. Profile selects access via include/exclude path declarations over that graph.
12. `ctx` is object-based and dynamic, so domains/subdomains can open/close over time.
13. Access used during declaration entrypoint execution is separate from callback invocation access; callback closures
    run with callback-scoped `ctx` masks resolved by allow/deny policy, which may be narrower or otherwise different.
14. Complex declarations are authored as module/node source trees with typed files and manifest classification rather
    than as one file/one capability declaration.
15. Raw unrestricted host graph access is not script-safe; scripts use projected facades (`ctx` objects) only.
16. Runtime later materializes USF [[Capability Instance]]s (for example Scale/Phenomenon instances) from declarations
    and host definitions established during staged startup and Runtime Lock.
17. These Capability Instances invoke sanctioned callback closures through type/trait/callback-tailored `ctx`
    capability-object subgraphs.
18. Canonical lifecycle, cyclic Rust/Rhai loop semantics, callback-path semantics, and multiplicity classes are
    defined in [[Capability]].
19. This note focuses on profile selection, `ctx` graph shaping, and declaration authoring ergonomics.
20. Scripts are object descriptors first, effectively the closest thing to project assets in this model.
21. Dependency semantics are layered: declaration dependencies are `ctx` path requirements, while provider and runtime
    dependencies live in separate layers.

Current startup-flow shape used as reference:

1. Read script files.
2. Resolve include/exclude capability-path declarations against type/trait/callback API graph topology.
3. Preprocess aliases.
4. Compile and execute declaration entrypoints with type/trait/callback-tailored `ctx` capability-object subgraphs to
   emit capability-node declaration material.
5. Activate runtime and materialize capabilities from capabilities established at Runtime Lock.
6. Emit runtime proof logging.
7. Runtime-lock definition-side mutation.

Reference-scope caveat:

1. This section captures profile/pipeline semantics.
2. Concrete legacy path wiring in MVP bootstrap code may drift from current asset layout snapshots.
3. When path-level details diverge, keep these semantics and update path examples from the active tree.

Legacy source pointers:

- `loo_cast_legacy/documents/markdown_summary/usf_script_profiles_and_mvp_slice.md`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/documents/markdown_summary/rhai_script_ergonomics.md`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/documents/markdown_summary/scripting_runtime_reference.md`

Adjacent notes:

- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)
- [Rhai Value Semantics and AccessCell Notes](Rhai%20Value%20Semantics%20and%20AccessCell%20Notes.md)

Rustdoc anchors:

- `crates/core_mod/src/spec/mod.rs`
