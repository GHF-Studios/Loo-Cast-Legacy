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

Open pressure:
A central manifest or aggregate lock-like file may be useful later, but it is not locked.

#glossary
