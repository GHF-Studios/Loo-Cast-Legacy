---
canonical_name: Phenomenon
status: WIP-draft
aliases:
  - Phenomena
---

A Phenomenon is a primary [[USF]] world-state, world-generation, interaction, and detail-materialization carrier.
It may be spatially localized, point-like, region-like, or distributed.
This makes phenomena overlap with some older zone-like intuitions, but without making `Zone` a separate primary
ontology authority.
Distributed phenomena are the primary replacement for zones as world authority.
Phenomena are themselves classification regions: localized phenomena have clearer physical boundaries, while distributed
phenomena may have fuzzier or more abstract boundaries.

Current owner-answer-informed framing:
Significant concrete world state belongs in phenomena rather than in [[Chunk]] storage.
Chunks may carry local, residual, insignificant, highly distributed, or intermediate state, but phenomena determine when
state or state change has crossed a significance threshold and should become canonical/entity-level material.

Phenomenon implementations define their own significance thresholds and use [[Metric]] APIs, other phenomena, and
cross-scale context to decide when chunk-local or metric-level data becomes phenomenon-relevant.
Phenomena own their own detail generation/materialization and reverse generation/re-aggregation logic.
Metrics inform that logic; metrics do not own it.
Downward detail expansion and zoom-out re-aggregation are therefore phenomenon-owned processes, even though the [[USF]]
defines the surrounding process model.

Open pressure:
Phenomenon is likely a complex/higher-order capability, an interactive runtime world-state carrier, and a declaration
type.
It is sort of entity-like but should not be collapsed into a conventional entity type without a dedicated pass.

#glossary
