#!/usr/bin/env sh
# One-time per-machine setup. Idempotent. Called by `make setup`.
# Points git at the versioned hooks dir and installs missing scanners.
set -eu

echo "==> Activating versioned git hooks (core.hooksPath = .githooks)"
git config core.hooksPath .githooks
chmod +x .githooks/* 2>/dev/null || true

echo "==> Checking security scanners"

need() { command -v "$1" >/dev/null 2>&1; }

install_hint() {
  cat <<'EOF'
Some scanners are missing. Install them (all free, local, no account):

  # macOS (Homebrew)
  brew install semgrep trivy gitleaks

  # Linux
  #   semgrep : pipx install semgrep   (or pip install semgrep)
  #   trivy   : https://trivy.dev/latest/getting-started/installation/
  #   gitleaks: https://github.com/gitleaks/gitleaks#installing

Re-run `make setup` after installing.
EOF
}

MISSING=0
for tool in semgrep trivy gitleaks; do
  if need "$tool"; then
    echo "  ok  $tool"
  else
    echo "  --  $tool (missing)"
    MISSING=1
  fi
done

if [ "$MISSING" -eq 1 ]; then
  install_hint
  exit 0   # don't hard-fail setup; hooks degrade gracefully if a tool is absent
fi

echo "==> Setup complete. Hooks active on next commit."
