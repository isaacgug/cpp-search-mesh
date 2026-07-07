## Summary

Describe what changed and why.

Fixes #

## Type Of Change

- [ ] `feat` - new functionality
- [ ] `fix` - bug fix
- [ ] `docs` - documentation only
- [ ] `refactor` - code change without behavior change
- [ ] `test` - test-only change
- [ ] `build` / `ci` / `chore` - tooling, dependencies, repository maintenance

## Scope

- [ ] `search-mesh-core`
- [ ] `search-mesh-mcp`
- [ ] `docs`
- [ ] repository tooling

## Verification

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] Manual MCP/stdin-stdout testing, if protocol behavior changed

If any check was not run, explain why:

## Notes For Reviewers

Call out protocol changes, user-visible behavior changes, performance tradeoffs, or follow-up work.

## Checklist

- [ ] The change is scoped to the stated problem.
- [ ] Public protocol or tool schema changes are documented in `docs/mcp-protocol.md`.
- [ ] Architecture or phase changes are documented in `docs/architecture.md` or `docs/roadmap.md`.
- [ ] No `.unwrap()` or `.expect()` was added outside tests.
- [ ] `Cargo.lock` is updated when dependencies changed.
