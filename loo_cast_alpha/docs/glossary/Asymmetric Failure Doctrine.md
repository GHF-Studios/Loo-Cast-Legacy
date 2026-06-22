---
canonical_name: Asymmetric Failure Doctrine
status: WIP-draft
aliases: []
---

Asymmetric Failure Doctrine means panic-fast is the default runtime integrity posture, while persistence-sensitive
paths (especially save/load) are handled with higher recovery care and corruption-avoidance policy.
This asymmetry is intentional.
It preserves fast failure and clear fault visibility without treating persistence risks as ordinary transient runtime
faults.

Current owner-answer-informed interpretation:
startup invalidity should hard-fail the game/runtime launch flow without crashing the launcher process when the launcher
can report the failure cleanly.
The launcher should block launch and surface structured diagnostics for authoring/configuration/Steam states it can
classify.
Runtime invariant violations should still fail visibly, including in user builds, because silent corruption is worse than
a crash.
Persistence safety should rely on corruption-avoidance, frequent backup/autosave strategy, atomic or replace-safe write
patterns where applicable, and hard failure when the safe path is no longer trustworthy.

Terminology note:
`fail-fast` and `panic-fast` remain unresolved as doctrine wording.
Until the wording is locked, docs should prefer precise phrases such as `blocked launch with diagnostics`,
`recoverable external failure`, `panic-fast invariant violation`, or `persistence-safe abort`.

See also:

- [[Runtime Lock]]
- [[Project Runtime Representation]]

#glossary
