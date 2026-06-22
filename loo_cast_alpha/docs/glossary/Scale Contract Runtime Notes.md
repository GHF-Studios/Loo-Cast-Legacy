# Scale Contract Runtime Notes

#tech_glossary

Related glossary terms:

- [Scale Contract](Scale%20Contract.md)
- [Scale Definition](Scale%20Definition.md)
- [Scale Support](Scale%20Support.md)
- [Scale Realizer](Scale%20Realizer.md)
- [Scale Realizer Cardinality](Scale%20Realizer%20Cardinality.md)
- [Scale Slice](Scale%20Slice.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)

Current runtime-facing assumptions carried from active glossary + legacy notes:

1. Canonical scale spine is treated as `-35..35` (`71` coordinates).
2. Each coordinate declares one scale definition and one declared scale-realizer type.
3. Scale support is declared positively for supported scale coordinates; unsupported coordinates normally remain
   implicit unless a later policy needs explicit denial.
4. Active scale slices must satisfy one-effective-realizer cardinality.
5. Scale-facing numeric behavior should be interpreted through the raw-model math contract posture
   (facade-first surfaces, explicit conversion boundaries, explicit operation policy).

Cross-scale math framing that still matters here:

- same-scale operations should prefer local numeric kernels when bounded and valid
- cross-scale paths require explicit conversion boundaries
- expensive global-USF math use should stay explicit

Legacy source pointers:

- `loo_cast_legacy/documents/markdown_summary/usf_math.md`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/*`

Rustdoc anchors:

- `crates/core_mod/src/spec/mod.rs`
