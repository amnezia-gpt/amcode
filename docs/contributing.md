## Contributing

`amcode` is a fork of `openai/codex`. We keep the upstream Codex engineering
style where it helps, but use a fork-specific branch workflow:

- `main` is reserved for syncing upstream `openai/codex`.
- `amnezia` is the default working branch for amcode development.
- Feature and fix branches are created from `amnezia`.
- Pull requests target `amnezia`.

### Issue-First Workflow

Start every non-trivial change with a GitHub issue. The issue should describe:

- the problem or goal;
- the intended scope;
- acceptance criteria;
- relevant risks or compatibility concerns;
- suggested tests, when known.

Keep issues focused. Multiple unrelated changes should be tracked by separate
issues and implemented in separate pull requests.

### Branch Naming

Create issue branches from `amnezia`:

```bash
git checkout amnezia
git pull origin amnezia
git checkout -b codex/issue-2-amcode-home
```

Use this branch pattern:

```text
codex/issue-<number>-<short-slug>
```

Examples:

```text
codex/issue-2-amcode-home
codex/issue-3-amnezia-auth
codex/issue-4-provider-strategy
```

### Development Workflow

- Keep changes focused on the linked issue.
- Prefer small pull requests with clear review boundaries.
- Follow the Rust and TUI conventions in `AGENTS.md`.
- If you change Rust code, run `just fmt` in `codex-rs` after edits.
- Run the most specific relevant tests for the crates or packages changed.
- If you change config types, update generated schemas as required by
  `AGENTS.md`.
- If you change dependencies, update the matching Bazel lockfiles as required.

### Pull Requests

Open pull requests against `amnezia`, not `main`.

The PR body should include:

- a linked issue, preferably `Fixes #<number>` or `Refs #<number>`;
- a short summary of the change;
- tests run, or a clear note if tests were not run;
- any follow-up work or compatibility risks.

Example:

```text
Fixes #2

## Summary
- Add AMCODE_HOME resolution.
- Move default home from ~/.codex to ~/.amcode.

## Tests
- cargo test -p codex-core config_home
```

### Review Process

Reviews should focus first on correctness, compatibility, and test coverage.
For fork-specific changes, also check that:

- `main` remains suitable for upstream sync;
- Codex user data is not modified accidentally;
- OpenAI/Codex-specific behavior is either preserved intentionally or replaced
  explicitly for amcode;
- docs and schemas are updated when user-facing behavior changes.

### Upstream Sync

Use `main` only to track upstream Codex:

```bash
git checkout main
git fetch upstream
git merge upstream/main
git push origin main
```

Then merge the updated upstream snapshot into `amnezia`:

```bash
git checkout amnezia
git merge main
git push origin amnezia
```

Resolve conflicts in `amnezia`. Do not put amcode feature work directly on
`main`.
