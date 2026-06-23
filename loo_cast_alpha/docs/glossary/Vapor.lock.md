---
canonical_name: Vapor.lock
status: WIP-draft
aliases:
  - Vapor Lockfile
---

Vapor.lock is the current pressure term for resolved dependency, fingerprint, and hash state in the [[Vapor Ecosystem]].
It is the lockfile counterpart to [[Vapor.toml]].

Current owner-answer-informed direction:

- Use plain TOML/lock-style files, not a database, for now.
- Lockfiles can exist for many artifact roots rather than one monolithic global lockfile.
- Lockfiles should store generated fingerprints and hashes for build/publish output.
- Lockfiles can support packagepacks, modpacks, enginepacks, gamepacks, mods, and nested capability declaration folders
  where resolved dependency state matters.
- Lockfiles should eventually record resolved [[Kernel Artifact]] identity, platform target, ABI/toolchain envelope,
  native dependency fingerprints, exported registration metadata, and compatibility proof.
- Lockfiles should record the [[Vapor Toolchain Envelope]] used for native capability-kernel builds where that affects
  compatibility.

Open pressure:
A central manifest or aggregate lock-like file may be useful later, but it is not locked.
Exact kernel/toolchain lock fields remain unsettled.

Phase 3 lock-candidate anchor:
The Phase 3 lockfile scope is anchored by [Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md),
especially P3-W02, P3-W09, and P3-W11.
Phase 3 must read/write lockfile state for resolved Packagepack/pack composition, fingerprints, hashes, and generated
build/publish output where resolved dependency state matters.

#glossary
