# USF Math Raw Model Foundation Notes

#tech_glossary

Related glossary terms:

- [Scale](Scale.md)
- [Scale Contract](Scale%20Contract.md)
- [USF Runtime](USF%20Runtime.md)
- [Runtime Substrate](Runtime%20Substrate.md)

This note treats `TMP_usf_math_raw_model` as the highest-authority draft math foundation for alpha-era semantics.
Other technical notes (including position/overflow semantics) should derive from this layer, not bypass it.

Canonical foundation points from the raw model sketch:

1. Facade-first contract posture:
   semantic traits define behavior + panic contracts, but script-facing use should go through explicit facades/bindings.
2. `OpMode` is type-level (`Kind`, `Repr`) and not runtime configuration state.
3. Runtime operation variance is modeled by `OpPolicy<T>` (`DeferToGlobal` vs `Explicit(T)`), and concrete algorithms
   must validate compatibility.
4. Mixed-representation paths are explicit unions (`UsfOrNormal*` aliases over `OneOf2`), with branch validity guarded
   by panic-fast checks.
5. Math surfaces are split into `CoreOps`, `FieldOps`, and `BridgeOps` contracts, then recomposed as full contracts.
6. Runtime field state/mutability is explicit (`Field<T>` with immutable/mutable policy, lock readiness states,
   contention/poison panic behavior).
7. Shape/domain constraints are contract-level, not hidden assumptions (for example `Vector<D>` requires `D >= 2`,
   matrix excludes vector-equivalent shapes).
8. USF cross-scale math is a first-class taxonomy (global numeric core, local projection, conversion boundaries,
   scale-indexed coordinates, cross-scale operators, temporal coupling, precision/range budgeting, deterministic
   parameterization).
9. Validation includes explicit panic-contract verification as a first-class quality lane.

How this affects current alpha notes:

- Position stack semantics should be read as a specialized downstream realization under this math foundation.
- Rhai/runtime binding notes should reference facade and dispatch usage over monomorphized contract surfaces, not direct
  trait-level generic calls.
- Scale/runtime lifecycle notes should preserve explicit conversion-boundary and policy-boundary semantics.

Legacy source pointers (authority layer):

- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/catalog.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/op_mode/mod.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/op_policy/mod.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/field.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/scalar/shared.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/vector/shared.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/matrix/shared.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/transform/shared.rs`

Rustdoc anchors:

- `crates/core_mod/src/spec/usf_math.rs`
