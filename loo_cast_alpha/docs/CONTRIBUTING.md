# Contributing

Current audience: internal/solo workflow for active alpha development.

## Flow

1. Start from a named issue.
2. Update local `develop` and create a topic branch from it.
3. Run `cargo xtask ...` from repository root (root alias shim) or from `loo_cast_alpha/` workspace root.
4. Run `cargo xtask setup_sdk` once per clone (or when hooks are missing).
5. Make the scoped change.
6. Add/update tests when needed.
7. Run `cargo xtask audit`.
8. Push the topic branch.
9. Open a PR into `develop`.
10. Link related issue(s) in the PR sidebar (`Development`).
11. Close linked issue(s) before merge.

## Code Style

- Follow Rust standard style. The pre-commit hook runs `cargo fmt`.
- No clippy warnings in workspace code. `cargo xtask audit` runs clippy with `--no-deps` and `-D warnings`.
- Add tests when behavior changes.
- Update docs when behavior/workflow changes.

## Local Hooks

- `pre-commit` formats the alpha workspace.
- `pre-push` runs `cargo xtask audit`.
- If hooks are missing, run `cargo xtask setup_sdk`.
- To remove managed hooks installed by `setup_sdk`, run `cargo xtask clean_sdk`.

## Questions?

Open an issue or start a discussion!
