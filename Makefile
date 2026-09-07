# Security pipeline — free, local, zero-token, offline (after DB fetch).
# Code never leaves the machine. No LLM. Deterministic scanners only.
.DEFAULT_GOAL := help
SEMGREP_PACKS := --config p/owasp-top-ten --config p/secrets

.PHONY: help setup security security-sast security-deps security-secrets db-update

help: ## Show available targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN{FS=":.*?## "}{printf "  %-18s %s\n", $$1, $$2}'

setup: ## One-time: activate versioned hooks + check scanners
	@sh .security/setup.sh

security: security-secrets security-sast security-deps ## Full local scan (all gates)
	@echo "==> All security scans passed"

security-sast: ## SAST: SQL injection, XSS, OWASP Top 10, unsafe patterns
	semgrep --config .security/semgrep.yml $(SEMGREP_PACKS) --error .

security-deps: ## SCA + IaC: vulnerable lib versions, insecure config
	trivy fs --config .security/trivy.yaml .

security-secrets: ## Secret scan (full history-aware, working tree)
	gitleaks detect --config .security/gitleaks.toml --redact --verbose

db-update: ## Refresh Trivy vulnerability DB (only network step)
	trivy image --download-db-only
