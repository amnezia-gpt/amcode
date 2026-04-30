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

### Issue Task Control

Use GitHub issues as the shared task-control system between people and Codex
runs. The issue should be useful when opened later in a different thread or
working session.

Keep the issue body stable after planning is complete. Treat it as the
canonical scope, not as a running execution log. It should contain:

- context and goal;
- scope and explicit non-scope;
- high-level task checklist or links to sub-issues;
- acceptance criteria;
- known risks and compatibility notes;
- suggested verification.

Use issue comments for execution history:

- planning notes that refine the approach;
- progress updates;
- implementation decisions;
- blockers;
- verification results;
- links to related branches and pull requests.

It is fine to edit the issue body while turning a rough issue into an
actionable plan. After implementation starts, prefer comments over repeated
body edits. Only edit the body again when the scope, acceptance criteria, or
canonical task list actually changes.

For large work, prefer separate issues or GitHub sub-issues over a long
checkbox list. A task should become its own issue when it can be implemented or
reviewed independently, has separate risks, or may need a separate pull request.

Do not use repo-local `PLAN.md` files as the durable source of truth for normal
GitHub-tracked work. Temporary local notes are fine, but durable task state
belongs in GitHub issues, comments, sub-issues, and pull requests.

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
- any issue task-control updates that reviewers should know about.

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
