# Workflow Usage Patterns Legacy Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
- [Workflow Runtime Structure Notes](Workflow%20Runtime%20Structure%20Notes.md)
- [Composite Workflow Runtime Notes](Composite%20Workflow%20Runtime%20Notes.md)
- [Normal Workflow Runtime Notes](Normal%20Workflow%20Runtime%20Notes.md)
- [Workflow Type Request and Timeout Notes](Workflow%20Type%20Request%20and%20Timeout%20Notes.md)
- [Workflow Stage Runtime Notes](Workflow%20Stage%20Runtime%20Notes.md)
- [Workflow Invariant Ledger Notes](Workflow%20Invariant%20Ledger%20Notes.md)

This note is the canonical home for concrete legacy workflow usage examples.
Other workflow runtime notes should link here instead of duplicating examples.
Status note: examples here are evidence for useful workflow shapes and pain points, not target API commitments.

## Pattern A: Startup Completion Signal (`Core::FinishStartup`)

1. `startup_system` builds `composite_workflow!(Startup, { workflow!(Core::FinishStartup); })`.
2. The composite runtime invokes a typed workflow request for `Core::FinishStartup`.
3. The workflow runtime initializes stage `InsertResource: Ecs`.
4. Stage logic inserts `StartupFinished`.
5. The completion path sends a terminal typed response and clears composite context.

Source pointers:

- `loo_cast_legacy/core_mod_api/src/backend/core/systems.rs`
- `loo_cast_legacy/core_mod_api/src/backend/core/workflows.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`

## Pattern B: Chunk Boundary While-Loop Orchestration (`UsfChunk::SpawnChunks`)

1. `chunk_detection_system` computes spawn/despawn deltas.
2. `chunk_management_system` starts a composite workflow when spawn inputs exist.
3. Composite logic calls `run_workflow_ioe_with_timeout_control::<...spawn_chunks::TypeIOE>()`.
4. The workflow initializes `ValidateAndSpawnAndWait: EcsWhile`.
5. Setup delegates to external spawn setup and stores progress-wrapped state.
6. Run polling emits `Wait` until all chunk entities are visible, then emits `Done`.
7. Completion finalizes the workflow instance and clears in-flight management state.

Source pointers:

- `loo_cast_legacy/core_mod_api/src/backend/chunk/systems.rs`
- `loo_cast_legacy/core_mod_api/src/backend/chunk/workflows/mod.rs`
- `loo_cast_legacy/core_mod_api/src/backend/chunk/workflows/external/spawn_chunks.rs`

## Pattern C: GPU Setup Cross-Domain Bootstrap (`Gpu::SetupTextureGenerator`)

Declared stage chain:

1. `Ecs`
2. `RenderWhile`
3. `Ecs`

Behavioral role:

- bootstrap GPU texture-generator setup across main/render domains
- wait on render-side readiness before returning to ECS continuation

Source pointer:

- `loo_cast_legacy/base_mod_api/src/backend/gpu/workflows.rs`

## Pattern D: GPU Generation Multi-Domain Pipeline (`Gpu::GenerateTextures`)

Declared stage chain:

1. `PrepareBatch: Ecs`
2. `GetTextureViews: RenderWhile`
3. `DispatchBatch: Render`
4. `WaitForBatch: EcsWhile`

Behavioral shape:

- ECS prepares texture handles and parameter buffers.
- RenderWhile waits for extract-visible image views.
- Render dispatches GPU work and registers completion receivers.
- EcsWhile polls for completion, then returns texture handles.

Note:

- This is the declared workflow-stage shape from legacy definitions.
- In the currently scanned legacy call-sites, explicit runtime invocation wiring for this exact workflow was not found
  in the same style as core/chunk orchestration call-sites.

Source pointers:

- `loo_cast_legacy/base_mod_api/src/backend/gpu/workflows.rs`
- `loo_cast_legacy/base_mod_api/src/lib.rs`
