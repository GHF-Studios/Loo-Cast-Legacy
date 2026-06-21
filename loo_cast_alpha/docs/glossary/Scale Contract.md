---
canonical_name: Scale Contract
status: WIP-draft
aliases: [ ]
---

The Scale Contract defines the declaration and compatibility rules for [[Scale Definition]], [[Scale Support]], and
[[Scale Realizer]] inside the [[USF Contract]] family.
For each canonical [[Scale]] coordinate in the global -35..35 set (71 total), one scale definition and one declared
scale-realizer type must be present.
For each capability and each canonical scale coordinate, support must be declared explicitly.
Each capability-scale pair has exactly one [[Scale Support]] state: `supported` or `unsupported`.
The default runtime counterpart composes through the [[USF Runtime]] and the [[Capability Runtime]].

Implementation-facing notes: [Scale Contract Runtime Notes](Scale%20Contract%20Runtime%20Notes.md)

#glossary
