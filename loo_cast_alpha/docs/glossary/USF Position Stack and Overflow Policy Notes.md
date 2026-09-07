# USF Position Stack and Overflow Policy Notes

#tech_glossary

Related glossary terms:

- [Scale](Scale.md)
- [USF Runtime](USF%20Runtime.md)
- [Observer-Relative Simulation](Observer-Relative%20Simulation.md)
- [Runtime Substrate](Runtime%20Substrate.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)

Quarantine extraction (`TMP_usf_pos_quarantine`) that looks reusable:

1. Position is layered: `GridVec` (recursive scale chain) + optional `SubgridXyz` + optional `UnitVec` offset.
2. Local grid/subgrid digits are balanced decimal digits in `[-5..5)` per axis.
3. Unit offsets are normalized into `[-500.0..500.0)` with carry into grid digits.
4. Math overflow has explicit policies (`wrap`, `checked`, `strict`) for digit-stack normalization.
5. Zoom in/out operations are explicit transforms across scale layers, with carry propagation and root-boundary guards.

Operational implications for alpha docs:

- overflow policy is a runtime behavior choice, not hidden implementation detail
- position normalization invariants are part of determinism guarantees
- subgrid-to-grid and unit-to-grid conversion rules should stay explicit where simulation boundaries are described
- these semantics are downstream of the raw-model math authority and should remain compatible with its
  `OpMode`/`OpPolicy`/conversion-boundary framing

Legacy source pointers:

- `loo_cast_legacy/documents/temp_stuff/TMP_usf_pos_quarantine/core_mod_api/src/backend/usf/pos/types.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_pos_quarantine/core_mod_api/src/backend/usf/pos/grid/types.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_pos_quarantine/core_mod_api/src/backend/usf/pos/subgrid/types.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_pos_quarantine/core_mod_api/src/backend/usf/pos/unit/types.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/catalog.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/op_mode/mod.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/op_policy/mod.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/field.rs`

Rustdoc anchors:

- `crates/core_mod/src/spec/usf_math.rs`
- `crates/core_mod/src/spec/usf_pos.rs`
