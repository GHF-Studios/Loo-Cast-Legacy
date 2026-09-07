---
canonical_name: Metric
status: WIP-draft
aliases:
  - Metrics
---

A Metric is a scale-bound [[USF]] [[Capability]] and data/function surface for exposing measured, derived, or computed
state at a specific [[Scale]].
Metrics include scale-local variables, constants, primitive stored fields, and composite/computed functions rather than
only passive sampled values.

Metrics are public within their owning scale context, but not automatically public across all scales.
This makes them a candidate mechanism for encapsulating logic and data into a specific scale while keeping access
scale-bounded.
Metrics may be compatible with a range of scales through explicit support/conversion rules, but cross-scale metric
semantics remain underdefined.

Metric kinds:

- Primitive metrics store or expose direct scale-local data such as temperature, pressure, humidity, or gravity-like fields.
- Composite metrics are first-class computed metrics produced from functions over one or more other metrics.

Phenomenon relationship:
[[Phenomenon]] implementations use metrics to evaluate significance thresholds and materialization/re-aggregation logic.
Phenomena do not own or produce metrics directly; metrics are integral to a scale's parameter space.
If a metric exists at a scale, a phenomenon that operates at that scale must account for it somehow, even if the local
implementation is effectively a no-op.
Distributed or field-like state may still be represented through metrics when object-to-object checks would be the wrong
execution model.
Metrics may be updated through explicit commit/apply-style passes when that is better than direct object-to-object
mutation.

Current owner-answer-informed answer:
Metrics can become the global-per-scale public data/function layer that replaces some older [[DPT]], [[ZLM]], and
[[Zone-Era Concepts]] use cases without reintroducing zones as authority.

Open pressure:
"Metric producer" is not currently a clear term and should not be used as if phenomena produce metrics.
Cross-scale metric compatibility and bridge/conversion APIs remain underdefined.

#glossary
