#!/usr/bin/env python3
"""
PostToolUse hook — cheat-sheets

Roda depois de Edit/Write. Se o arquivo afetado for .go, aplica
'gofmt -w' automaticamente. Isto fecha o loop da "Regra de ouro" em
AGENTS.md com enforcement real, em vez de depender só da instrução.

Nunca bloqueia (sempre exit 0) — é conveniência, não gate. O gate de
lint real continua sendo 'golangci-lint run' no processo por task
(AGENTS.md, Passo 6) e no CI.
"""
import json
import subprocess
import sys


def main():
    try:
        event = json.load(sys.stdin)
    except Exception:
        sys.exit(0)

    tool_input = event.get("tool_input", {}) or {}
    file_path = tool_input.get("file_path", "") or ""

    if not file_path.endswith(".go"):
        sys.exit(0)

    try:
        result = subprocess.run(
            ["gofmt", "-w", file_path],
            capture_output=True,
            text=True,
            timeout=10,
        )
        if result.returncode != 0:
            # Informativo apenas — nunca bloqueia (PostToolUse não desfaz nada).
            print(f"gofmt falhou em {file_path}: {result.stderr}", file=sys.stderr)
    except FileNotFoundError:
        print("gofmt não encontrado no PATH — pulei a formatação automática.", file=sys.stderr)
    except Exception as exc:
        print(f"post_tool_use_lint: erro inesperado: {exc}", file=sys.stderr)

    sys.exit(0)


if __name__ == "__main__":
    main()
