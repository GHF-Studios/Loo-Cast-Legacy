---
canonical_name: Asymmetric Failure Doctrine
status: WIP-draft
aliases: []
source_of_truth: []
---

Asymmetric Failure Doctrine means panic-fast is the default runtime integrity posture, while persistence-sensitive
paths (especially save/load) are handled with higher recovery care and corruption-avoidance policy.
This asymmetry is intentional.
It preserves fast failure and clear fault visibility without treating persistence risks as ordinary transient runtime
faults.

Current owner-answer-informed interpretation:
startup invalidity should hard-fail the game/runtime launch flow without crashing the launcher process when the launcher
can report the failure cleanly.
Runtime invariant violations should still fail visibly, including in user builds, because silent corruption is worse than
a crash.
Persistence safety should rely on corruption-avoidance, frequent backup/autosave strategy, and hard failure when the
safe path is no longer trustworthy.

Terminology note:
`fail-fast` and `panic-fast` remain unresolved as doctrine wording.

See also:

- [[Runtime Lock]]
- [[Project Runtime Representation]]

#glossary
