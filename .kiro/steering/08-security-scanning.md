---
inclusion: auto
name: Local Security Scanning
description: >-
  How to run the project's free, local, zero-token security scans (Semgrep,
  Trivy, Gitleaks) for SQL injection, vulnerable dependencies, secrets, and
  OWASP checks. Apply when the task involves security review, scanning, CI
  gates, git hooks, or verifying code before commit/merge.
---
# Local Security Scanning (free, offline, zero-token)

Deterministic scanners — no LLM, no tokens, code never leaves the machine
(only public CVE DB downloads). Configs live in `.security/`.

## Commands
- `make setup` — one-time per machine: activates versioned hooks
  (`core.hooksPath .githooks`) and checks that scanners are installed.
- `make security` — full scan (secrets + SAST + deps/IaC). Use before merge.
- `make security-sast` — Semgrep: SQL injection, XSS, OWASP Top 10.
- `make security-deps` — Trivy: vulnerable lib versions + insecure IaC.
- `make security-secrets` — Gitleaks: credentials in the tree.
- `make db-update` — refresh Trivy CVE DB (the only network step).

## Gates
- **pre-commit hook** (`.githooks/pre-commit`): fast, staged-diff only.
  Degrades gracefully if a scanner is absent.
- **CI** (`.github/workflows/security.yml`): mandatory, machine-independent.
  Make it a required check in branch protection.

## Rules for the agent
- Run scans via these `make` targets, not ad-hoc tool flags, so config stays
  in `.security/`.
- Never add a scanner mode that uploads source to a cloud service. Local only.
- Reference cheat sheets in `docs/cheat-sheets/` for stack-specific guidance
  (Next.js, .NET API / Lambda).
