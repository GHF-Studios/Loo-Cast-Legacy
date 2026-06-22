# Glossary Alignment Issue Ledger

Status: concise rewrite after owner rejection of the verbose first-pass ledger.

Format:

- `FIX`: concrete wording/content fix.
- `RENAME`: likely name/title/canonical-name change.
- `DELETE`: likely page/concept removal.
- `SPLIT`: concept should split into smaller concepts.
- `MERGE`: concept should fold into another concept.
- `MOVE`: page/tag/location/classification issue.
- `AUDIT`: cross-page consistency pass needed.
- `DECIDE`: unresolved owner/design decision required before rewrite.
- `DONE`: resolved; kept temporarily for review before archive/removal.

This ledger is intentionally terse. It is a work queue, not doctrine.

## Source-Control Issues

1. `FIX` Modified summary now mixes summary text and owner corrections; preserve it as an input artifact, not polished docs.
2. `FIX` The summary may be ahead of many source pages; promote corrected summary language back into source pages where valid.
3. `AUDIT` The source glossary and summary now diverge in owner-intent level; build a systematic source-vs-summary comparison pass.
4. `FIX` Future summary docs should not include raw owner comments inline.
5. `FIX` Keep the alphabetic review mode because it helps holistic review.
6. `FIX` Add terse issue ledgers for future alignment passes rather than essay-style ledgers.
7. `AUDIT` Add a second pass that finds issues not explicitly mentioned by owner comments.
8. `AUDIT` Add a third pass that checks each glossary page against every relevant linked concept.
9. `FIX` Preserve exact source filenames when summaries or issue ledgers reference pages.
10. `FIX` Distinguish “owner correction,” “assistant inference,” and “source-doc contradiction” in future ledgers.

## Obsidian / Summary Infrastructure

11. `FIX` Add stable `## Summary` sections to glossary pages so Obsidian embeds can use `![[Page#Summary]]`.
12. `FIX` Keep `## Summary` sections concise enough to embed cleanly.
13. `FIX` Add separate persistent summary index for `#glossary`.
14. `FIX` Add separate persistent summary index for `#tech_glossary`.
15. `FIX` Summary indexes should use embeds instead of duplicating summary text manually.
16. `MOVE` Decide where generated/review summaries live so they do not pollute the canonical glossary folder.
17. `FIX` README should explain embed conventions if the vault adopts summary sections.
18. `FIX` README should explain that `.obsidian/` files are editor state, not glossary content.
19. `AUDIT` Check whether every page has a single stable heading suitable for embedding.
20. `AUDIT` Check whether aliases/canonical names conflict with Obsidian link targets.

## Glossary vs Tech Glossary Split

21. `FIX` Define hard criteria for `#glossary` vs `#tech_glossary`.
22. `MOVE` Implementation notes should consistently use `#tech_glossary`.
23. `MOVE` Legacy/quarantine evidence pages should not look like stable concept pages.
24. `MOVE` Phase execution/testing/runbook pages may need a tag other than plain `#glossary`.
25. `MOVE` Crate/workspace topology pages may be tech/plan notes, not stable glossary concepts.
26. `MOVE` `Vapor.toml` and `Vapor.lock` may be hybrid schema concepts; decide tag treatment.
27. `MOVE` `SDK` and `Vapor Launcher` may be product/tool concepts but also implementation plans; decide tag treatment.
28. `FIX` README tag explanation currently makes naive scans count README as both glossary and tech glossary.
29. `FIX` Add standard status vocabulary for `stable concept`, `WIP concept`, `tech note`, `legacy signal`, `quarantine signal`, and `stale`.
30. `AUDIT` Reclassify every page after status vocabulary exists.

## Vapor / Product Stack

31. `FIX` Vapor must remain the foundational layer for capabilities, Rhai authoring, SDK, launcher, composition, and distribution.
32. `FIX` Vapor should be independently understandable without first-party Spacetime/Loo Cast internals.
33. `FIX` Vapor is Steam-exclusive for Phase 3; storefront abstraction stays future-pressure.
34. `FIX` `steam-like-platform-contracts` should remain future-pressure and not weaken Steam-first implementation.
35. `FIX` Vapor is more than modding; remove or deprecate overly narrow “Vapor Modding Ecosystem” framing.
36. `AUDIT` Search docs for old “platform” vs “ecosystem” confusion.
37. `DECIDE` Decide whether “Vapor Platform” should become a top-level concept or remain avoided.
38. `FIX` Generic Engine/Game roles are Vapor composition roles, not Spacetime/Loo Cast templates.
39. `FIX` A Vapor Engine/Game can expose minimal modding after bootstrap; docs should mark this as allowed but not the intended ergonomic path.
40. `FIX` First-party examples should not smuggle first-party internals into generic Vapor role definitions.
41. `FIX` Engine page currently couples non-Spacetime engines and USF boundary awkwardly; split those concerns.
42. `FIX` Game page should stay about `base_mod`/Game role, not mod artifact internals.
43. `FIX` Enginepack/Gamepack/Packagepack terminology is mostly settled; protect it from old unqualified `package` language.
44. `AUDIT` Remove old `package`, `composite package`, and vague manifest wording from roadmap/RFC docs.
45. `FIX` Packagepack remains launchable composition, not source/build/distribution artifact.
46. `FIX` Enginepack must select coupled `core_engine` + matching `core_mod`.
47. `FIX` Gamepack must select one `base_mod` compatible with selected Enginepack.
48. `FIX` Modpack nesting must stay visible in fingerprints/diagnostics and not be conceptually flattened.
49. `FIX` Extension Mod attachment must be explicit metadata, not implicit folder/code behavior.
50. `FIX` Reserved role pages should keep “mandatory but replaceable by valid selection” crisp.
51. `FIX` `core_mod` independent replacement is forbidden for Phase 3; docs should not imply mix-and-match.
52. `AUDIT` Ensure `core_engine`, `core_mod`, and `base_mod` are always described as reserved role names and literal crate names where relevant.
53. `FIX` Loo Cast page should distinguish Game, Product bundle, and Project/repo context more sharply.
54. `DONE` Spacetime Engine page should not claim ownership of Vapor-level capability runtime.
55. `FIX` USF should stay public/API-facing Spacetime subsystem, not product pillar.
56. `AUDIT` Search for phrasing that makes USF directly replaceable without replacing/forking the Engine.
57. `FIX` Pillar Dependency Topology should reflect future multi-project split pressure.
58. `FIX` Vapor Crate Topology should connect to likely future repo split.
59. `DECIDE` Current `NOW.md` says keep one repo; owner now expects future split. Clarify current policy vs future plan.
60. `FIX` Project Authoring Structure should mention eventual Vapor / Spacetime Engine / Loo Cast split.

## Phase 3 / Testing / Execution Plans

61. `RENAME` `Phase 3 Vapor Scenario Suite` likely becomes `Phase 3 Vapor Testing Suite`.
62. `FIX` Avoid “integration-test suite” as the main term; some flows are manual and not CI-runnable.
63. `FIX` Distinguish automated validation tests, local manual scenarios, and Steam manual verification.
64. `FIX` Phase 3 testing language should include manually verified Steam/Workshop flows.
65. `FIX` Phase 3 should prove public/installable/authorable/publishable artifacts, not only logs/fingerprints.
66. `FIX` Phase 3 remains Vapor/Capability/Rhai/Steam proof, not USF/worldmodel proof.
67. `FIX` Phase 3 still requires real executable launch fixtures.
68. `FIX` Hello-world-on-steroids fixtures should be minimal real MVPs, not fake placeholders.
69. `FIX` Phase 3 output should stay non-gameplay: logs, strings, files, fingerprints, diagnostics.
70. `AUDIT` Update `phase_3_vapor_execution_spec.md` after glossary terminology changes.
71. `AUDIT` Update `phase_2_to_11_execution_program.md` after testing-suite rename.
72. `AUDIT` Update `NOW.md` after testing-suite rename.
73. `AUDIT` Update glossary backlinks after testing-suite rename.
74. `FIX` Phase 3 acceptance should include broad valid/invalid permutation coverage, not one token matrix.
75. `FIX` Published schema migration can remain deferred/pre-alpha-nukable.
76. `FIX` CI should not imply live Steam integration testing by default.
77. `FIX` Steam flows should fail with structured diagnostics, not panics, where possible.
78. `FIX` Workshop verification should validate fingerprints without claiming hostile-code sandboxing.
79. `FIX` Phase 3 docs should say Vapor.lock/fingerprints are mandatory despite older alpha docs saying no hashes.
80. `AUDIT` Remove stale Phase 3-as-USF wording anywhere still present.

## Capability Bedrock

81. `FIX` Capability remains intentionally broad: graph node, contract surface, API surface, authority surface, metadata unit, orchestration seam.
82. `FIX` Capability breadth should be described as intentional, not accidental overload.
83. `FIX` Vapor defines what Capability means before engines/games define their own capability types.
84. `FIX` Capabilities can be Rust-only with no Rhai declaration surface.
85. `FIX` Rhai support itself is a capability.
86. `FIX` Pure Rhai capabilities without meaningful Rust host support should remain disallowed except trivial local computation.
87. `FIX` Native/hardcoded Rust capabilities must be projectable into Rhai contexts.
88. `FIX` Capability graph is Vapor-level; Spacetime/USF are users/extensions, not owners.
89. `FIX` Running `core_engine` process should have one large runtime capability graph, with separate metadata registries/projections as needed.
90. `FIX` The raw capability metadata registry may differ from the active runtime graph.
91. `FIX` Capability graph should be heavily concurrent/multithread-friendly if everything routes through it.
92. `FIX` Capabilities should not be anonymous; private/internal is okay.
93. `FIX` Visibility should roughly follow Rust-like semantics where useful.
94. `FIX` Private/internal nodes remain real full-graph nodes, not just hidden projections.
95. `FIX` Leaf-like capabilities should not hide large private subgraphs by default.
96. `FIX` Umbrella/composite capabilities may justify private subgraphs.
97. `FIX` A capability can serve as type/category for other capabilities but cannot be its own type.
98. `FIX` Self-typing, self-dependency, and dependency cycles are invalid bootstrap shapes.
99. `FIX` Composite capabilities are first-class nodes, not named views.
100. `FIX` Composite capabilities may own policy unknown to child capabilities.
101. `FIX` “Capability Instance” remains suspect terminology; avoid unless a later pass locks it.
102. `FIX` Capability Declaration remains pre-lock authored payload, not runtime object.
103. `FIX` Capability Slot Type remains the projected/gated slot/context shape, not callback type.
104. `FIX` Capability Slot Type creation must be explicit opt-in, not automatic per declaration.
105. `FIX` Capability edge taxonomy remains unresolved; docs should not imply final dependency/slot/API/authority edge model.
106. `FIX` Capability path is addressing/policy input, not dependency or causality.
107. `FIX` Capability Location remains unresolved implementation vocabulary.
108. `FIX` Capability Projection API may be enough to absorb Scripting Projection Meta-Layer.
109. `DECIDE` Decide whether Capability Contract should split into metadata, declaration rules, projection rules, and runtime rules.
110. `FIX` Capability Contract currently carries too much legacy/USF/Rhai/runtime pressure in one page.
111. `DONE` Capability Runtime should be Vapor-defined infrastructure embedded/adapted by launched compositions.
112. `DONE` Spacetime Engine should utilize/extend capability runtime, not host or define it.
113. `DONE` Runtime Substrate should not sound like it owns Capability semantics.
114. `FIX` Capability Runtime and Modding Runtime are deeply coupled; docs should not overstate sibling separateness.
115. `FIX` Modding is a major composition use of capability semantics.
116. `FIX` Mod runtime representation should say mods resolve into capability graph contributions.
117. `FIX` Some capabilities emit intents/requests; others directly bind Rust kernel operations.
118. `FIX` Reconcile/commit/apply still owns canonical state progression where state authority matters.
119. `FIX` Capability roles beyond input/output remain unresolved.
120. `FIX` Input/output may remain useful directional vocabulary in scale/runtime contexts.
121. `DECIDE` Authority/reconciler/realizer/bridge/mutator role taxonomy needs later pressure testing.
122. `FIX` Dynamic authority resolution should stay phase/operation relative.
123. `FIX` Global Capability Surface should not grant global domain-state authority.
124. `FIX` Global Capability API Graph must remain host-authoritative and not script-safe.
125. `FIX` Capability Graph Diagnostics should keep player/modpack-author/developer projections distinct.
126. `FIX` Explicit mod-wide conflicts are author-friendly metadata, not replacement for graph validation.
127. `FIX` Steam/Workshop failures should be recoverable diagnostics where possible.
128. `FIX` Internal invariant violations can remain panic-fast in development.
129. `FIX` Capability graph diagnostics must not imply hostile-code sandboxing.
130. `FIX` Capability Bootstrap Fixed-Point Cycle should stay deterministic and cycle-free.

## Slots / Static Graph Core / Dynamic Substrate

131. `FIX` Slots are static composition mechanics for startup graph core.
132. `FIX` Runtime dynamism after lock should be modeled by capabilities/registries/kernels, not slot mutation.
133. `FIX` Filled slot is itself a capability node in parent/child relation.
134. `FIX` Slot is parent-owned child position with type/cardinality/policy.
135. `FIX` Slot cardinality should not be baked into Capability Slot Type.
136. `FIX` One slot should accept one concrete node/capability type unless later generalized.
137. `FIX` Deep acyclic nesting is allowed; cycles are not.
138. `FIX` Slot graph composition belongs before Runtime Lock.
139. `FIX` Immutable startup core vs dynamic runtime substrate needs consistent vocabulary.
140. `FIX` Optional provider is underexplored; do not present as settled.
141. `FIX` Integration aperture is underexplored; do not present as settled.
142. `FIX` Exclusive slot, variadic slot, ordered registry, optional provider, and integration aperture need definitions or demotion.
143. `DECIDE` Decide which slot policy names deserve standalone glossary pages.
144. `FIX` Ordered registry may imply lookup/query semantics, not just ordering; mark unresolved.
145. `FIX` User-selected load order should not be normal conflict-resolution mechanism.

## Rhai / Scripting

146. `FIX` Rhai is declaration-first and effectively declaration-only, but callbacks/closures can contain behavior.
147. `FIX` “Scripts do not own lifecycle scheduling” is better than “scripts do not orchestrate lifecycle.”
148. `FIX` Scripts may influence lifecycle through sanctioned extension/configuration points.
149. `FIX` Scripts must not define fundamental scheduler structure.
150. `FIX` Rhai Asset should say Phase 3 proves end-to-end capability/Rhai stack, not only one callback path.
151. `FIX` Focused callback proof remains useful but should not understate Rhai’s Phase 3 role.
152. `FIX` One Rhai file maps to one authored leaf declaration by default.
153. `FIX` File-internal capability definitions should default private/internal.
154. `FIX` Folder-level Rhai aggregation plus Vapor.toml grouping is allowed.
155. `FIX` Vapor.toml owns manifest/dependency/publication metadata; Rhai owns declarations.
156. `FIX` Sidecar `.meta` files remain disfavored.
157. `FIX` Generated textures/models/sounds remain outputs/caches/delivery artifacts, not canonical authored source.
158. `FIX` Rhai Capability should include projected native hardcoded capabilities.
159. `FIX` Rhai Capability should separate origin from projection/use.
160. `FIX` Rhai Capability should say Rust owns heavy/most state authority, not all conceivable state authority.
161. `FIX` Callback Type, Callback Context Type, and Callback Signature must stay separate.
162. `FIX` Callback Context Type needs stronger host metadata / Rust function-shape framing.
163. `FIX` Callback metadata should help validate or reject required context graph projection early.
164. `FIX` Callback access masks differ from declaration access masks.
165. `FIX` Callback access outside resolved mask hard-fails.
166. `FIX` Rhai generic dispatch cannot rely on runtime Rust monomorphization.
167. `FIX` Rhai Generic Dispatch page is substance-correct but too dense; add concise summary.
168. `DONE` Reflection macro surface is useful legacy signal, not final doctrine.
169. `DONE` Bridge/access-provider notes are useful legacy/quarantine signal, not stable target.
170. `DONE` Value semantics / AccessCell notes are provisional and need status banner.
171. `DONE` Add consistent `legacy_signal` / `quarantine_signal` status blocks to Rhai tech notes.
172. `FIX` Script Safety should be projection-safety, not broad anti-malware guarantee.
173. `FIX` Native engine/mod binaries cannot be made non-malicious merely by Vapor docs.
174. `FIX` Workshop content is validated for integrity/compatibility/fingerprint, not sandboxed as hostile code.
175. `MERGE` Scripting Projection Meta-Layer may fold into Capability Projection API.
176. `RENAME` If kept, Scripting Projection Meta-Layer needs a clearer name.
177. `DECIDE` Decide whether Scripting Projection Meta-Layer survives at all.
178. `FIX` Generic Rhai declaration semantics should not be buried inside USF instantiation pages.
179. `SPLIT` Separate generic Rhai declaration substrate from USF-specific script profiles.
180. `FIX` Future scripting-language support should remain possible without making Rhai semantics too engine-specific.

## USF / Scale / Simulation

181. `FIX` USF is public/API-facing Spacetime subsystem, not Vapor product pillar.
182. `FIX` Replacing USF effectively means replacing/forking enough of Spacetime Engine to be another Engine.
183. `FIX` USF Contract is internal to Spacetime Engine product stack, not Vapor-level contract family.
184. `DONE` USF Runtime should compose with Vapor capability runtime, not redefine it.
185. `FIX` USF Definition Lifecycle applies to immutable startup-constructed core, not every dynamic runtime structure.
186. `FIX` Runtime evolution can add dynamic state/substrate over locked core if explicitly modeled.
187. `FIX` USF Instantiation Scripts feel wobbly because generic Rhai and USF-specific profiles are mixed.
188. `FIX` USF Instantiation Capability Slot Notes should be clearly legacy MVP slice alignment.
189. `FIX` USF Math Raw Model Foundation is outdated as implementation authority.
190. `FIX` USF math should prefer existing crates and `num_traits` where practical.
191. `FIX` Preserve raw-model semantics that still matter: conversion boundaries, operation policy, determinism, panic contracts.
192. `FIX` Remove “highest-authority draft math foundation” wording if custom math lib is no longer direction.
193. `FIX` USF Position Stack remains useful but must derive from current math posture.
194. `FIX` Scale remains canonical semantic coordinate, not runtime realization by itself.
195. `DONE` Scale Contract should not require explicit unsupported declarations for every capability-scale pair.
196. `DONE` Scale Support should become positive support declaration plus default absence semantics unless specific denial is needed.
197. `DONE` Scale Support may need to generalize beyond capabilities to scale-aware semantic surfaces.
198. `DONE` Scale Contract Runtime Notes must stop saying each pair is explicitly supported/unsupported.
199. `DECIDE` Confirm whether `supported` / `unsupported` enum survives as internal resolved state.
200. `DECIDE` Confirm whether every canonical scale still requires one scale definition and one realizer type.
201. `DECIDE` Confirm whether every active scale requires at least one Phenomenon and Metric.
202. `DONE` Scaled Capability Channel should become a USF-compatible capability pattern, not general capability law.
203. `RENAME` Consider `Scale-Scoped Capability Channel` or `USF-Scoped Capability Channel`.
204. `DONE` Global utilities such as logging/math may be unscaled or differently scoped.
205. `FIX` Observer-Relative Simulation needs internal coherence pass.
206. `FIX` Higher scales also participate in significance flow; current wording underexplores this.
207. `FIX` Larger-scale changes affecting lower-scale detail need a home concept.
208. `FIX` Cross-scale significance flow may deserve a page.
209. `FIX` Scale View should be substrate for camera/render/chunk loading later, not those implementations.
210. `FIX` Entity Proxy, Entity Plane Split, and Portal Traversal need clearer mechanism-vs-semantics split.
211. `DONE` Entity Plane Split should mention usefulness for f32/f64 technologies such as physics engines.
212. `DONE` Do not overcommit to Rapier specifically.
213. `DONE` Portal Traversal should link Entity Plane Split.
214. `FIX` Entity Proxy can simplify world wrapping mechanically, while Portal Traversal owns semantic continuity.
215. `FIX` Zone-era pages should keep “superseded but signal-bearing” status.

## Artifacts / Source / Packaging / Project Structure

216. `DONE` Build Artifact needs concrete examples: linked shared objects/binaries before packaging.
217. `DONE` Build Artifact should distinguish “built output” from assembled runtime library payload set.
218. `DONE` Distributable Artifact should remain final packaged/uploadable object.
218a. `DECIDE` `Authoring Artifact` naming pressure: alias/rename candidate for built outputs once surrounded by runtime libraries, payloads, manifests, dependencies, and publication metadata; avoid collision with `Source Artifact`.
219. `DONE` Source Artifact should include Vapor.toml and Rhai declarations explicitly.
220. `DONE` Redistributable Mod Implementation Library should remain runtime-deliverable platform library set.
221. `DONE` Redistributable Mod Contract Source should remain development/dependency source package.
222. `AUDIT` Check artifact pages for source/build/distributable/runtime confusion.
223. `FIX` Project Artifact Structure feels too wobbly; tighten or fold.
224. `MERGE` Project Artifact Structure may fold into Artifact + Project Structure if no unique role remains.
225. `FIX` Project Authoring Structure must discuss intended multi-repo/multi-project future.
226. `FIX` Project Structure may be too generic unless it carries real authoring/artifact/runtime taxonomy value.
227. `DONE` Project Runtime Representation should avoid making Capability Runtime sound Spacetime-owned.
228. `FIX` Project Ethos needs inspiration and systemic-reality language.
229. `FIX` Project Ethos should mention showing deep interconnection rather than simplified lies.
230. `FIX` Player-to-Creator Path should connect more strongly to inspiration and systemic literacy.
231. `DONE` SDK should include LSP/editor support where applicable.
232. `FIX` SDK command surface should stay public creator-facing contract by Phase 3.
233. `FIX` SDK vs xtask boundary should remain clear.
234. `FIX` Vapor Launcher modes should stay player/modpack-author/developer separated.
235. `FIX` Launcher and SDK should be sibling surfaces over Vapor core, not nested.

## Workflow / Legacy Runtime

236. `DONE` Workflow Execution Trace Notes should be removed.
237. `DONE` Remove backlinks to Workflow Execution Trace Notes.
238. `DONE` Workflow Usage Patterns Legacy Notes should become the canonical trace/example page.
239. `DONE` Workflow pages need consistent legacy implementation signal status.
240. `DONE` Workflow Framework should remain Rust-side orchestration, not Rhai lifecycle ownership.
241. `DONE` Workflow stage execution should stay Bevy-system-visible where possible.
242. `DONE` Workflow control-plane exclusive `&mut World` usage should be documented as refactor debt.
243. `DONE` Stage Buffer backlog should be marked as a real observed legacy problem.
244. `DONE` Backlog caused visual holes/lag; current neutral wording understates severity.
245. `DONE` Single-item poll progress should be marked deterministic but throughput-limited.
246. `DONE` Unsafe output/input `transmute` contract should be marked legacy hazard.
247. `DONE` Placeholder stage slot lifecycle should remain refactor-sensitive invariant.
248. `DONE` Active-run key gate should be flagged as concurrency bottleneck.
249. `DONE` RenderWhile sharding should be described as partial parallelism-preserving mechanism, not full parallelism.
250. `DONE` Stage Sender Cache can remain legacy mechanism signal.
251. `DONE` Normal vs composite workflow distinction is useful and should stay.
252. `DONE` Workflow Type timeout behavior should distinguish panic defaults from controlled retry/abort path.
253. `DONE` Consider moving workflow legacy notes out of primary glossary later.
254. `DONE` Decide whether workflow framework remains future target architecture or only legacy evidence.
255. `DONE` Check workflow terms for accidental current-target wording.

## Failure / Runtime Lock / Determinism

256. `FIX` Runtime Lock applies to launchable Engine/Game composition, not launcher/SDK dynamic runtimes.
257. `FIX` Runtime Lock should distinguish immutable startup graph core from dynamic runtime substrate.
258. `FIX` Post-lock graph mutation is forbidden by default.
259. `FIX` Runtime dynamism after lock requires explicit capability/registry policy.
260. `FIX` Determinism should be “deterministic-by-default” where absolute determinism is too strong.
261. `FIX` Deterministic activation/composition should be separated from runtime nondeterminism/external IO.
262. `FIX` Asymmetric Failure Doctrine should keep startup invalidity from crashing launcher if clean diagnostics exist.
263. `FIX` Runtime invariant violations may panic even in user builds.
264. `FIX` Persistence paths need special backup/autosave/corruption-avoidance behavior.
265. `DECIDE` `panic-fast` vs `fail-fast` doctrine wording remains unsettled.
266. `FIX` Closed Runtime and Open Design feels too broad/weak.
267. `MERGE` Closed Runtime and Open Design may fold into Runtime Lock + Managed Ambiguity.
268. `DECIDE` Decide whether Closed Runtime and Open Design survives as a page.
269. `FIX` Managed Ambiguity is useful if it names unresolved edges without excusing drift.
270. `FIX` Dynamic Authority Resolution should stay but needs concrete examples eventually.

## Cross-Doc Propagation

271. `AUDIT` Update `docs/ARCHITECTURE.md` after capability runtime ownership correction.
272. `AUDIT` Update `docs/NOW.md` after repo-split and testing-suite corrections.
273. `AUDIT` Update `docs/RFCS/phase_3_vapor_execution_spec.md` after Phase 3 testing terminology changes.
274. `AUDIT` Update `docs/RFCS/phase_2_to_11_execution_program.md` after Packagepack/testing/capability terminology changes.
275. `AUDIT` Update `docs/RFCS/alpha_doctrine_draft.md` for stale Phase 3/USF references.
276. `AUDIT` Check docs root files for old package/manifest/capability wording.
277. `AUDIT` Check diagram filenames/descriptions for obsolete terminology.
278. `AUDIT` Check question batch references before promoting new doctrine.
279. `AUDIT` Add a migration note for renamed/deleted glossary pages.
280. `AUDIT` Add backlink cleanup pass after page rename/delete decisions.

## Immediate Low-Risk Edit Queue

281. `DONE` Add Entity Plane Split link to Portal Traversal Semantics.
282. `DONE` Add traditional f32/f64 tech motivation to Entity Plane Split.
283. `DONE` Add Build Artifact examples.
284. `DONE` Add LSP/editor support pressure to SDK.
285. `DONE` Add legacy/quarantine status labels to Rhai tech notes.
286. `DONE` Mark Stage Buffer backlog as known legacy problem.
287. `DONE` Remove Workflow Execution Trace backlinks.
288. `DONE` Reword Spacetime Engine so it utilizes Capability Runtime.
289. `DONE` Reword Scaled Capability Channel as USF-specific pattern.
290. `DONE` Reword Scale Support away from mandatory explicit unsupported entries.

## Higher-Risk Rewrite Queue

291. `SPLIT` Capability Contract likely needs structural rewrite.
292. `MERGE` Scripting Projection Meta-Layer likely merges into Capability Projection API.
293. `RENAME` Phase 3 Vapor Scenario Suite likely renames to Phase 3 Vapor Testing Suite.
294. `MERGE` Closed Runtime and Open Design likely merges into Runtime Lock/Managed Ambiguity.
295. `SPLIT` USF Instantiation Scripts likely splits generic Rhai declaration semantics from USF-specific profiles.
296. `FIX` USF Math Raw Model Foundation needs full rewrite to existing-crates/num_traits posture.
297. `FIX` Project Authoring Structure needs multi-repo future rewrite.
298. `FIX` Project Ethos needs full expansion.
299. `AUDIT` Capability vs Modding relationship needs broad cross-page rewrite.
300. `AUDIT` Vapor/Spacetime/USF ownership boundaries need full cross-page rewrite.
