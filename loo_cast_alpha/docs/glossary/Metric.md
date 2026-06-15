---
canonical_name: Metric
status: WIP-draft
aliases:
  - Metrics
source_of_truth: []
---

A Metric is a scale-bound [[USF]] data/function surface for exposing measured, derived, or produced state at a specific
[[Scale]].
Current owner-answer-informed pressure is that metrics may be better understood as scale-local variables, constants,
functions, or metric producers rather than only as passive sampled values.

Metrics are public within their owning scale context, but not automatically public across all scales.
This makes them a candidate mechanism for encapsulating logic and data into a specific scale while keeping access
scale-bounded.

Implementation pressure:
Metric surfaces may ultimately map to [[Rust]] modules, [[Rhai]] declaration surfaces, or generated
binding/projection APIs.
The exact boundary between `metric`, metric producer, metric combination/symbiosis, and [[Phenomenon]] significance
threshold logic is not settled.

Current owner-answer-informed answer:
Metrics can become the global-per-scale public data/function layer that replaces some older [[DPT]], [[ZLM]], and
[[Zone-Era Concepts]] use cases without reintroducing zones as authority.

#glossary
