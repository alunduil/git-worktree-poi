<!--
SPDX-FileCopyrightText: 2026 Alex Brandt
SPDX-License-Identifier: MIT
-->

# Agent instructions for git-worktree-poi

Run the pre-commit hooks with `prek run --all-files`.

## Change a CI job

alunduil/alunduil-infrastructure lists this repository's required status
checks, by job `name:`, in `terraform/alunduil/repositories.tf`. Update that
list in the same change:

- A new job gates nothing until listed, and Renovate automerges without it.
- A renamed job leaves its old name required, which blocks every merge.

## Title pull requests and issues

- Write a pull request title as an imperative sentence. Squash merge makes it
  the commit subject.
- Write an issue title as the outcome, stated as a present-tense fact:
  "Pruning spares a worktree a session is starting in".
