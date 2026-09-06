#!/usr/bin/env python3
"""
PostToolUse hook — cheat-sheets

Roda depois de Edit/Write. Se o arquivo afetado for .rs, aplica
'rustfmt' automaticamente; se for .ts/.svelte/.js, aplica
'prettier --write'. Isto fecha o loop da "Regra de ouro" em AGENTS.md
com enforcement real, em vez de depender só da instrução.

Nunca bloqueia (sempre exit 0) — é conveniência, não gate. O gate de
lint real continua sendo 'cargo clippy -- -D warnings' + 'npm run lint'
no processo por task (AGENTS.md, Passo 6) e no CI.
"""
import json
import subprocess
import sys

FORMATTERS = {
    (".rs",): ["rustfmt"],
    (".ts", ".svelte", ".js"): ["npx", "--no-install", "prettier", "--write"],
}


def formatter_for(file_path: str):
    for extensions, cmd in FORMATTERS.items():
        if file_path.endswith(extensions):
            return cmd
    return None


def main():
    try:
        event = json.load(sys.stdin)
    except Exception:
        sys.exit(0)

    tool_input = event.get("tool_input", {}) or {}
    file_path = tool_input.get("file_path", "") or ""

    cmd = formatter_for(file_path)
    if cmd is None:
        sys.exit(0)

    try:
        result = subprocess.run(
            [*cmd, file_path],
            capture_output=True,
            text=True,
            timeout=10,
        )
        if result.returncode != 0:
            # Informativo apenas — nunca bloqueia (PostToolUse não desfaz nada).
            print(f"{cmd[0]} falhou em {file_path}: {result.stderr}", file=sys.stderr)
    except FileNotFoundError:
        print(f"{cmd[0]} não encontrado no PATH — pulei a formatação automática.", file=sys.stderr)
    except Exception as exc:
        print(f"post_tool_use_lint: erro inesperado: {exc}", file=sys.stderr)

    sys.exit(0)


if __name__ == "__main__":
    main()
