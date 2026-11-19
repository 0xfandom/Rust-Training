---
name: developer-writing
description: Write precise, natural commit messages, pull requests, issues, and technical documentation for the Rust training repository.
---

# Developer writing

Use this skill whenever creating or editing commits, PRs, issues, reviews, README text, or documentation in this repository. Read the actual diff and CI status first. Prefer plain developer language to generic templates.

## Commits

- Use Conventional Commits when the change fits: `feat`, `fix`, `test`, `refactor`, `perf`, `docs`, `ci`, `chore`.
- Format: `type(scope): imperative summary`; use a concrete scope such as `sorting`, `search`, `graphs`, or `workspace`.
- Keep the subject short (aim for 72 characters or fewer); describe what changed, not the act of working.
- Keep commits focused. Include a body only when motivation, invariants, tradeoffs, or compatibility need explanation.
- Don't use filler such as "update code", "improve implementation", or "successfully added". Don't claim a performance gain without measurements.
- Squash-merge with a meaningful final subject; preserve useful reasoning in the PR description.

Good:
```text
feat(sorting): add stable merge sort
test(search): cover empty and duplicate inputs
fix(graphs): mark nodes visited before enqueueing
ci(workspace): check formatting and clippy
```

For a non-obvious fix:
```text
fix(graphs): avoid duplicate BFS queue entries

Mark nodes visited when enqueueing rather than when dequeuing.
This prevents repeated work when multiple edges reach the same node.
```

## Pull requests

- Use a specific, human-sounding title that matches the diff.
- In a few sentences explain the change, why it matters, and any non-obvious design choice.
- State the tests that actually ran and their results. If checks are pending or unavailable, say so explicitly.
- Mention real risks or follow-ups only when relevant; don't invent them.
- Link the correct issue with `Closes #N`.
- Open a regular, ready-for-review PR against `main`, from a branch created at the latest `main`. Never use a feature branch as the PR base.

Example:
```markdown
Adds merge sort for integer slices, using one reusable buffer across recursive merges.
Tests cover empty inputs, reversed arrays, duplicates, and integer boundaries.

Validation: cargo fmt, workspace tests, and Clippy passed in GitHub Actions.

Closes #6
```

## Issues and documentation

- Issues should state the problem, acceptance criteria, edge cases, expected complexity, and relevant tests.
- Document algorithm contracts, invariants, and time/space complexity. Explain why tricky code exists rather than narrating each line.
- Keep READMEs factual and useful; avoid marketing language or repeated boilerplate.
- Avoid artificial enthusiasm, generic summaries, unnecessary emojis, and phrases like "seamlessly", "leverages", and "comprehensive solution".
- Never fabricate benchmarks, review approvals, test results, or authorship. Do not add AI/automation attribution gratuitously, but follow any disclosure requirements.

## Before publishing

1. Re-read the diff and confirm every statement is supported.
2. Check for duplicate algorithms, issues, and PRs.
3. Confirm the branch starts from latest `main` and PR base is `main`.
4. Run or inspect actual `cargo fmt --all -- --check`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings` results.
5. Leave the PR open if validation is missing, failing, or mergeability is uncertain.
6. After successful checks, squash-merge and refresh `main` before new work.
