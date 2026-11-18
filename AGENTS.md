<!--
SPDX-FileCopyrightText: 2026 Alex Brandt
SPDX-License-Identifier: MIT
-->

# Agent instructions for git-worktree-poi

Rules a coding agent can't infer from the files. Linters and hooks in
`.pre-commit-config.yaml` cover everything else.

## Tooling

- Pre-commit hooks run under prek, which mise installs from `mise.toml`. Run
  them with `prek run --all-files`.
- `mise.toml` sets `locked = true`, so bumping a tool version there also needs
  `mise lock`, or the install fails.

## CI and branch protection

- Required status checks are listed by job `name:` in
  `terraform/alunduil/repositories.tf` in alunduil/alunduil-infrastructure, not
  in this repository. Renaming a CI job, or adding one that must gate merges,
  needs a matching change there.
- Renovate automerges green updates and waits only on those listed checks, so a
  new job blocks nothing until it's listed.

## Commits, issues, and pull requests

- Pull requests squash merge, and the pull request title becomes the commit
  subject. Write it as an imperative sentence in plain English, not
  Conventional Commits.
- Issue titles state the outcome as a present-tense fact, such as "Pruning
  spares a worktree a session is starting in".
