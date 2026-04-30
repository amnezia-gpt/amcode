# Workflow Strategy

`amcode` is a fork of `openai/codex`.

- `main` is reserved for upstream Codex sync.
- `amnezia` is the default working branch.
- Pull requests for amcode development should target `amnezia`.

The upstream workflows in this directory are split so that pull requests get
fast, review-friendly signal while the upstream branch gets heavier
post-merge verification. In this fork, keep that intent but adapt branch
targets carefully: amcode development should not require feature work to land
on `main`.

## Pull Requests

- `bazel.yml` is the main pre-merge verification path for Rust code.
  It runs Bazel `test` and Bazel `clippy` on the supported Bazel targets,
  including the generated Rust test binaries needed to lint inline `#[cfg(test)]`
  code.
- `rust-ci.yml` keeps the Cargo-native PR checks intentionally small:
  - `cargo fmt --check`
  - `cargo shear`
  - `argument-comment-lint` on Linux, macOS, and Windows
  - `tools/argument-comment-lint` package tests when the lint or its workflow wiring changes

## Post-Merge Verification

- Upstream Codex uses `main` for post-merge verification.
- In amcode, equivalent post-merge checks should run for `amnezia` where they
  validate fork development.
- Workflows that exist only for upstream sync or OpenAI release infrastructure
  should stay disabled or guarded for this fork.

Upstream behavior:

- `bazel.yml` also runs on pushes to `main`.
  This re-verifies the merged Bazel path and helps keep the BuildBuddy caches warm.
- `rust-ci-full.yml` is the full Cargo-native verification workflow.
  It keeps the heavier checks off the PR path while still validating them after merge:
  - the full Cargo `clippy` matrix
  - the full Cargo `nextest` matrix
  - release-profile Cargo builds
  - cross-platform `argument-comment-lint`
  - Linux remote-env tests

## Rule Of Thumb

- If a build/test/clippy check can be expressed in Bazel, prefer putting the PR-time version in `bazel.yml`.
- Keep `rust-ci.yml` fast enough that it usually does not dominate PR latency.
- Reserve `rust-ci-full.yml` for heavyweight Cargo-native coverage that Bazel does not replace yet.
