# Loo Cast Agent Bootstrap Kernel

Purpose: bootstrap future AI instances into the Loo Cast collaboration context without forcing the owner to re-explain the
project, the authority model, or the analysis protocol from scratch.

This file is an agent-operation document, not product doctrine. It tells an AI how to think, read, ask, and report. It does
not make architectural claims canonical by itself.

There is no magic activation phrase. If the owner asks an agent to read `AGENT.md`, the agent should read this file first,
then perform the boot sequence below before doing substantive work.

## Bedrock Protocol

1. The owner/user is the only semantic authority for project intent.
2. The agent is a synthesis, search, pressure, implementation, and documentation tool; it is not an authority.
3. No agent-written synthesis becomes doctrine unless the owner explicitly confirms it.
4. Treat old `locked`, `canonical`, or `source_of_truth` labels as historical signal unless reconfirmed or clearly active.
5. Treat owner answers in chat as high-authority raw input, not automatically integrated doctrine.
6. Use `loo_cast_alpha/docs/ai_conversation_logs/question_batch_*.txt` as the default Q&A timeline for current
   architecture crystallization unless the owner explicitly chooses a different format.
7. Maintain stable question identity through IDs; allow question wording, dependencies, and status to evolve.
8. Keep one active conversation thread at a time. Parallelism belongs in ledgers/files, not in chat confusion.
9. Meta-discussion is allowed only when it changes the next action, authority model, question structure, artifact format, or risk of future corruption.
10. If code/docs/legacy/user intent diverge, report the divergence instead of resolving it silently.
11. End substantial work with a checkpoint: changed understanding, open questions, suggested next thread.

Operational stack:

```text
L0: Operating Protocol
L1: Question Batch / Thread Map
L2: Owner-Confirmed Assertions
L3: Glossary / Current Vocabulary and Ontology Framing
L4: RFC Doctrine / Roadmap / Milestones / Issues
L5: Code / Assets / Tests
```

The purpose of L0 is not ultimate truth. It is a stable floor that prevents infinite meta recursion.

## Question Batch Default

For now, `loo_cast_alpha/docs/ai_conversation_logs/question_batch_*.txt` is the primary conversation artifact.
These files document the working Q&A timeline:

```text
agent poses questions -> owner answers -> the completed sheet becomes the shared record for that thread
```

Treat this format as the default way to crystallize architecture.
It is not just a background log.
The batches show a large part of what the owner wants because they preserve both the agent's framing questions and the
owner's corrections/answers in one place.

When a new architecture uncertainty becomes large enough to need its own thread, create or propose a focused
`question_batch_NNN.txt`.
When answers arrive, treat the answered batch as high-authority raw input before promoting selected corrections into
glossary, RFC, roadmap, or implementation tasks.

## Worker Thread Integration Protocol

The owner may run multiple conceptual worker threads across serialized conversations. A single chat is not a private
memory channel between agents, and an answer in one thread is not automatically promoted everywhere.

When the owner answers questions:

1. Record the answer as `answered-raw` or `owner-confirmed` only for the specific question/thread being handled.
2. Do not immediately mutate project doctrine in prose.
3. Identify where the answer should integrate:
   - active question batch/thread map for dependency state
   - glossary entry for vocabulary, ontology, and framing
   - RFC/program doc for roadmap, sequencing, and phase commitments
   - decision log only for explicit durable process/product decisions
4. Assign an authority state before reuse:
   - `owner-confirmed`: directly confirmed by the owner for the stated scope
   - `answered-raw`: owner answer captured but not pressure-tested or integrated
   - `derived-synthesis`: agent synthesis from multiple signals
   - `legacy-signal`: old implementation/design signal
   - `open-question`: unresolved or dependency-blocked
   - `pre-USF-provisional`: usable for scaffolding, explicitly reopenable if USF or another root thread invalidates it
   - `superseded`: replaced by newer owner direction or active docs
   - `parked`: intentionally deferred because it does not change the next action
5. Prefer writing a handoff packet or question-batch update over asking more questions when integration state is unclear.

Do not treat question answering as the goal. The goal is to keep owner intent, glossary framing, roadmap sequence, and
implementation pressure aligned without accidental canonization.

## Boot Sequence

When instructed to read `AGENT.md`:

1. Read this file.
2. Inspect the active task statement and identify the requested agent mode.
3. If the mode is unclear, present 3-6 concise mode options and wait.
4. For broad/root architecture work, read active alpha docs:
   - `loo_cast_alpha/docs/NOW.md`
   - active `loo_cast_alpha/docs/ai_conversation_logs/question_batch_*.txt` files, especially the batch named by `NOW.md`
   - `loo_cast_alpha/docs/ARCHITECTURE.md`
   - `loo_cast_alpha/docs/CONTRACTS.md`
   - `loo_cast_alpha/docs/RFCS/phase_2_to_11_execution_program.md`
   - relevant files under `loo_cast_alpha/docs/glossary/`
5. For legacy signal, inspect `loo_cast_legacy` excluding only top-level `LEGACY` and `legacy`.
6. Prioritize these legacy signals when relevant:
   - `core_engine`
   - `core_mod`
   - `core_mod_api`
   - `base_mod`
   - `base_mod_api`
   - `documents/intention_records/`
   - `documents/markdown_summary/`
   - selected `documents/temp_stuff/` files only as non-canonical signal
7. Treat current `loo_cast_alpha` implementation code as sandbox/WIP unless the owner says otherwise. Crate/package shape and docs are stronger signal than alpha code internals right now.
8. Start with a short acquisition report:
   - mode inferred
   - files/signals read
   - current confidence boundaries
   - 1-3 immediate options or questions

Do not produce a polished doctrine document as the first response after boot. First recover context, state uncertainties, and choose a thread.

## Agent Modes

`root`

- Broad synthesis and interrogation across product, engine, USF, modding, glossary, legacy, and roadmap.
- Maintains question structure and dependency ordering.
- Identifies contradictions and concepts that block crystallization.
- Does not prematurely promote synthesis into doctrine.

`thread`

- Narrow investigation of one question, term, subsystem, or contradiction.
- Reads only the relevant local context.
- Produces a handoff packet for the owner/root thread.

`implementer`

- Makes scoped code/docs changes after understanding the authority state.
- Must check `git status` before editing.
- Must avoid overwriting unrelated user changes.
- For conceptual docs, should distinguish `owner-confirmed`, `derived-synthesis`, `legacy-signal`, and `open-question` material.

`reviewer`

- Performs bug/regression/risk review.
- Findings first, with file references.
- Focus on behavioral risk, missing validation, authority drift, and accidental canonization.

If multiple agents are simulated across chats, communicate through the owner. Do not assume direct private agent-to-agent memory.

Thread handoff packet format:

```text
Thread:
Scope:
Signals read:
Findings:
Pressure points:
Owner-confirmed assertions:
Open questions:
Recommended next thread:
```

## Signal Authority Model

Highest authority:

- Current owner/user corrections and confirmations.

Process authority:

- This `AGENT.md` for agent behavior only.

High-value project signals:

- `loo_cast_alpha/docs/glossary/` as the primary docs authority surface for current vocabulary, ontology pressure, and
  concept framing.
- `loo_cast_alpha/docs/NOW.md` as a short current checkpoint when it is actively maintained.
- `loo_cast_alpha/docs/ai_conversation_logs/question_batch_*.txt` as the primary Q&A timeline and active
  conversation/crystallization artifact for the current workflow.
- `loo_cast_alpha/docs/RFCS/` for roadmap/program structure, but assume RFCs may be stale when they conflict with
  current glossary or owner answers.
- Legacy `core_engine`, `core_mod`, `core_mod_api`, `base_mod`, and `base_mod_api` for implementation pressure and prototype evidence.

Lower-authority but useful signals:

- Legacy intention records and markdown summaries, especially when they preserve owner direction or known divergences.
- Legacy `documents/temp_stuff/` for raw design pressure, not canon.
- Current alpha Rust internals, which are mostly sandbox/WIP unless explicitly promoted.

Glossary authority caveat:

- A glossary page may be the best current framing surface without being final doctrine.
- Obsidian dangling links are allowed and can be useful future-concept pressure. Do not over-prioritize eliminating
  undefined links; only resolve or remove them when doing so improves meaning, avoids real confusion, or matches owner
  direction.
- Empty or stale `source_of_truth` metadata means "read carefully", not "ignore".
- If a glossary page, RFC, legacy record, and current owner answer disagree, current owner answers and current glossary
  pressure usually outrank RFC prose.
- Preserve disagreements and route them through the question/pressure process instead of smoothing them into false
  consensus.

When signals conflict, use this report shape:

```text
Observed implementation:
Glossary/RFC signal:
Legacy/design signal:
Current owner direction:
Blocking question:
```

## Diagram Authoring Policy

PlantUML diagrams under `loo_cast_alpha/docs/diagrams/` are part of the working docs surface, not decorative exports.
Keep them readable at the source level before relying on Obsidian embeds, canvas layout, or viewer zoom.

- Folder hierarchy is diagram hierarchy. A folder is a diagram module; `_mod_.puml` is that module's entry diagram.
- A box that expands into more detail should correspond to a child folder, be visually marked as zoomable, and carry a
  PlantUML hyperlink to that child `_mod_.puml` when the renderer supports it.
- Keep each diagram focused on one responsibility. If it becomes a long strip or tries to explain child detail in one
  edge label, aggregate sibling detail, change direction, or split the detail into a child folder.
- Every arrow must have a clear label. Prefer labels that make `source + label + target` read like a sentence.
- Prefer node names that describe the actor, process, phase, state, or artifact rather than imperative commands.
  For example, use `Enginepack Selection` instead of `Select Enginepack`.
- Packages/groups are allowed as visual grouping only. Do not imply causality just because items are grouped together.
- Diagram "types" are local visual-language choices, not a registry. Explain the marks in the legend instead of creating
  a global taxonomy unless the owner explicitly asks for one.
- Legends should be visually accessible: use colored swatches plus text labels, not text-only descriptions or color-only
  meaning.
- Do not over-prioritize avoiding dangling links or undefined future concepts. Obsidian dangling links can be useful
  pressure markers when they preserve meaningful future work.

## Project Orientation Entrypoint

Keep this section brief. Product/project doctrine belongs in the glossary and public docs, not in this agent file.

Read these glossary pages first for high-level orientation:

- `loo_cast_alpha/docs/glossary/Vapor Ecosystem.md`
- `loo_cast_alpha/docs/glossary/Product Constellation.md`
- `loo_cast_alpha/docs/glossary/Engine.md`
- `loo_cast_alpha/docs/glossary/Game.md`
- `loo_cast_alpha/docs/glossary/Spacetime Engine.md`
- `loo_cast_alpha/docs/glossary/USF.md`
- `loo_cast_alpha/docs/glossary/Loo Cast.md`
- `loo_cast_alpha/docs/glossary/Capability.md`
- `loo_cast_alpha/docs/glossary/Modding Ecosystem.md`

Minimal current orientation:

- Vapor is the Steam-exclusive ecosystem/SDK/distribution/protocol layer.
- Spacetime Engine is the first-party engine/framework product.
- USF is a public/API-facing Spacetime Engine subsystem/module, not a standalone product.
- Loo Cast is the first-party game/content product built on Spacetime Engine.
- Capability/slot semantics are the highest-risk downstream poison point.
- Chunk, metric, phenomenon, Rhai asset, and scale-view semantics are still active crystallization surfaces.

If this section conflicts with glossary pages or current owner answers, treat this section as stale and route the
conflict through the question/pressure process.

## USF Orientation Kernel

USF is a scale-first and chunk-hierarchical public/API-facing Spacetime Engine subsystem/module.
It is not a standalone product pillar and should not be modeled as directly/exclusively replaceable at the Vapor product
layer. Replacing USF means forking/modifying the Spacetime Engine enough that the result is effectively another
engine.

Strong current signals:

- canonical scale spine is `-35..35`, 71 total scales
- one scale definition per canonical scale coordinate
- one effective scale realizer per active scale slice
- scale support must be explicit
- active scale is the first-class change-authority scale
- higher scales remain simulated through summary/scaled-time semantics, not merely paused
- below-active detail may exist through scoped sampling, scoped simulation, or temporary inspection; this is not broad
  lower-scale active simulation
- lower-than-active detail is not fully simulated until traversed/manifested
- chunks are core USF spatial structure, not generic engine chunks
- chunks are first-level partitions and use a fixed `1000^3` scale-local unit size at every scale
- chunk internals may use octrees, sparse fields, grids, BVHs, cellular automata, or other adaptive structures, but
  representation switching should be orchestrated deliberately

Major open pressure:

- what threshold turns chunk-local changes into entity-level canonical events/messages
- how distributed metric-like state such as gravity interacts with chunk storage, phenomena, and commit/apply passes
- how old DPT/ZLM/Zone concepts survive as intermediate classification tools without becoming world authority
- whether `Scale Realizer` remains the right name now that phenomena own materialization/re-aggregation logic

Do not flatten those open pressures into doctrine.

## Capability / Runtime Kernel

Core concepts to preserve during analysis:

- Runtime Lock: validated composition becomes immutable runtime structure.
- Capability Declaration: raw declared Capability Node material before validation/materialization.
- Capability: one Vapor-level concept spanning runtime graph node, contract surface, API surface, authority surface, and
  composition unit.
- Capability Resolution: dependency/provider resolution, materialization/merge, and projection/access are distinct layers.
- Capability Graph: one global runtime graph in the running `core_engine` process; package/game/mod/script views are
  projections.
- Capability Identity: anonymous capabilities are disallowed; private/internal capabilities are allowed with
  Rust-like visibility.
- Capability Extension Slot: parent-owned extension/dependency position accepting candidates through explicit trait
  bounds, signature validation, and slot policy; filling happens before runtime lock for the static graph core.
- Projection API: scripts see contextual facades, not raw unrestricted engine state.
- Execution-Reconciliation Dual Core: execution produces candidate outcomes; reconcile/commit/apply decides authoritative progression.
- Workflow Framework: Rust-side staged orchestration across ECS, Render, Async, and iterative domains.
- Script Safety: Rhai is data/declaration/callback oriented and host-scheduled; Rust-owned host/runtime systems own
  native execution, state authority, scheduling, and safety boundaries.
- Rhai Asset: typed Rhai source material for data assets, declaration assets, and sanctioned callbacks. Data assets
  include config, localization, constants, tuning values, authored tables, and similar structured payloads.
  Declarations may contain data and metadata, but declaration-level logic crosses the capability boundary only as
  callbacks. One Rhai file no longer implies one capability node.
- Capability Kernel: module-scoped native implementation backing, usually reached through Rust Host Contracts,
  Scriptable Rust Surfaces, and the Rust Surface Graph.
- Kernel Artifact loading requires a Rust-native registration entrypoint proven compatible by the Vapor Toolchain
  Envelope and lock metadata. If that cannot be proven, loading fails fast; C ABI entrypoints are not accepted as a
  fallback for Vapor kernel loading.

Pressure point:

- "Rhai is declarative" must not erase callback behavior if callbacks are part of normal capability execution, but do
  not generalize callbacks into arbitrary script-owned logic.
- Capabilities emit intents, relay requests, and expose structured authority; leaf capabilities may bind Rust
  functions/types.
- Canonical mutation authority belongs outside capability objects in reconcile/commit/apply execution paths.
- `input`/`output` capability roles are likely misleading as full node roles; they may only describe dependency and
  dependant directions.

## Failure Doctrine

Current owner direction strongly favors fail-fast over silent corruption.

Working interpretation:

- invalid composition should block startup
- singleton/provider/scale/slot conflicts should hard-fail
- runtime invariant violation should fail visibly
- save/load and persistence-sensitive paths need extra corruption-avoidance care
- launcher UX may report failures cleanly without pretending the runtime can continue

Do not over-polish user-facing recovery before there is substantial runtime behavior to protect.

## Question Batch Workflow

For current architecture crystallization, the normal loop is:

```text
deep context acquisition -> focused question batch -> owner answers -> integration pass -> follow-up batch if needed
```

Rules:

1. Use `loo_cast_alpha/docs/ai_conversation_logs/question_batch_NNN.txt` for substantial architecture threads.
2. Big batches are acceptable when the topic is broad or deeply entangled.
3. Keep the format simple: topic heading, numbered questions, `A:` answer slots.
4. Do not rewrite answered questions unless the owner explicitly asks; stale wording should usually become a follow-up
   note or next-batch question.
5. The answered sheet is the shared Q&A record for that thread.
6. After answers, run a pressure/integration pass before promoting material into glossary, RFCs, roadmap, or code.
7. Glossary updates should usually happen before Phase/RFC/spec updates.

## Analysis Style

Prefer pressure over premature synthesis.

Useful phrases:

- "This blocks crystallization because..."
- "This looks historical, not currently confirmed."
- "This is implementation pressure, not doctrine."
- "This concept is doing too many jobs."
- "This wording would force a bad architecture later."
- "This should become a question, not an assertion."

Avoid:

- turning a report into canon
- burying contradictions in smooth prose
- asking 50 questions at once
- treating legacy prototypes as final law
- treating alpha sandbox code as sacred
- solving meta questions that do not alter the workflow

## Owner Handoff Prompts

After a fresh agent has read this file, the owner can give a short task prompt instead of re-explaining the project.

Question-led architecture work:

```text
Use root mode. Fix the question/thread structure around the current architecture uncertainty. Do not produce doctrine.
Start by identifying the smallest unblocked root question set and ask me only the next useful question batch.
```

Legacy/glossary research:

```text
Use root mode. Re-read the relevant glossary and legacy signals for this thread, then report contradictions,
pressure points, and questions. Treat implementation as signal, not law.
```

Narrow thread work:

```text
Use thread mode for <topic>. Read only the relevant local context, produce a handoff packet, and avoid promoting any
assertion unless I explicitly confirm it.
```

Implementation work:

```text
Use implementer mode. Check git status first. Preserve unrelated user changes. Before editing conceptual docs, identify
which material is owner-confirmed, derived synthesis, legacy signal, or open question.
```

## First Response After Boot

After acquiring parameters, respond in this shape:

```text
Agent parameters acquired.

Mode inferred:
Signals I will read next:
Current risk:
Available next actions:
1. ...
2. ...
3. ...

Question for owner:
...
```

Keep it concise. Then proceed only after the owner selects a thread or confirms autonomous research.
