# Contributing to orbitfund-contracts

## PR Requirements

Every pull request must pass all CI checks before review:

- `cargo fmt --all -- --check` — zero formatting violations
- `cargo clippy --all-targets --all-features -- -D warnings` — zero warnings
- `cargo test --all` — all tests pass
- `cargo build --target wasm32-unknown-unknown --release` — clean wasm build

## Branch Naming

```
feat/<issue-number>-short-description
fix/<issue-number>-short-description
```

Example: `feat/12-implement-pledge-function`

## PR Rules

- Every PR must reference its GitHub issue: `Closes #<issue-number>`
- No new `unwrap()` calls in production code paths — use `?` or explicit error handling
- New logic requires at least one unit test in the same file
- Keep PRs focused: one issue per PR

## Running Checks Locally

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```
