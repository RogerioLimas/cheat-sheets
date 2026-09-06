#!/usr/bin/env python3
"""
PreToolUse hook — cheat-sheets

Le o JSON do evento no stdin e decide bloquear (exit 2) ou permitir
(exit 0) a chamada de ferramenta ANTES de ela executar.

Isto e o enforcement determinístico que falta quando a regra só existe
em AGENTS.md/CLAUDE.md (ver "Rules Files, Orca Orchestration and Agent
Guardrails" — prompt-only compliance cai para ~48% sob pressão;
hooks continuam valendo mesmo sob --dangerously-skip-permissions).

Nunca edite este arquivo para "liberar" algo pontualmente — mude a
lista abaixo com o Roger, fora de uma sessão de execução (Protocolo
Anti-Impulso).
"""
import json
import re
import sys

# Bibliotecas aprovadas (ver docs/SPEC.md, Seção "Stack" / AGENTS.md)
APPROVED_CARGO_DEPS = {
    "tauri",
    "tauri-plugin-global-shortcut",
    "notify",
}
APPROVED_NPM_DEPS = {
    "svelte",
    "@tauri-apps/api",
    "@tauri-apps/plugin-fs",
    "@tauri-apps/plugin-http",
    "fuzzysort",
    "markdown-it",
    "gray-matter",
    "turndown",
}

# Padroes de comando Bash sempre bloqueados
DENY_BASH_PATTERNS = [
    (r"\bgit\s+push\s+.*(--force|-f\b)", "git push --force/-f é bloqueado. Peça confirmação humana explícita."),
    (r"\bsudo\b", "sudo é bloqueado."),
    (r"\bchmod\s+(-R\s+)?777\b", "chmod 777 é bloqueado."),
    (r">\s*/dev/", "redirecionamento para /dev/ é bloqueado."),
    (r"\bgit\s+reset\s+--hard\b.*\borigin/main\b", "reset --hard contra main é bloqueado."),
    (r"\brm\s+-rf\b(?!\s+(dist|build)/)", "rm -rf fora de dist/ ou build/ é bloqueado."),
    (r"\brm\b.*\bdocs/(PRD|SPEC|TASKS)\.md\b", "docs/PRD.md, docs/SPEC.md e docs/TASKS.md são contratos de design — não podem ser deletados por comando."),
]


def deny_bash(command: str):
    for pattern, reason in DENY_BASH_PATTERNS:
        if re.search(pattern, command):
            return reason

    m = re.search(r"\bcargo\s+add\s+(-D\s+|--dev\s+)?([^\s]+)", command)
    if m:
        pkg = m.group(2).split("@")[0]
        if pkg not in APPROVED_CARGO_DEPS:
            return (
                f"'cargo add {pkg}' adiciona uma dependência fora da lista aprovada "
                f"em AGENTS.md. Pare e peça aprovação explícita do Roger antes de "
                f"instalar — não tente contornar editando Cargo.toml na mão."
            )

    m = re.search(r"\bnpm\s+(install|i|add)\s+(-D\s+|--save-dev\s+)?([^\s]+)", command)
    if m:
        pkg = m.group(3).split("@")[0] if not m.group(3).startswith("@") else m.group(3).rsplit("@", 1)[0]
        if pkg not in APPROVED_NPM_DEPS:
            return (
                f"'npm install {pkg}' adiciona uma dependência fora da lista aprovada "
                f"em AGENTS.md. Pare e peça aprovação explícita do Roger antes de "
                f"instalar — não tente contornar editando package.json na mão."
            )
    return None


def deny_edit_write(file_path: str):
    protected_exact = {"Cargo.toml", "Cargo.lock", "package.json", "package-lock.json"}
    protected_suffix = ("docs/PRD.md", "docs/SPEC.md", "docs/TASKS.md")
    name = file_path.replace("\\", "/")
    base = name.rsplit("/", 1)[-1]
    if base in protected_exact:
        return (
            f"Edição direta de {base} é bloqueada. Dependências passam por "
            f"'cargo add <pacote aprovado>' ou 'npm install <pacote aprovado>'; "
            f"qualquer outra mudança em {base} exige aprovação explícita do Roger."
        )
    if any(name.endswith(suffix) for suffix in protected_suffix):
        # Edição de conteúdo é permitida (é assim que /rogeros-update funciona);
        # o que é bloqueado é a exclusão, tratada no Bash e por permissions.deny.
        return None
    return None


def main():
    try:
        event = json.load(sys.stdin)
    except Exception:
        sys.exit(0)  # nunca quebrar o fluxo por hook malformado

    tool_name = event.get("tool_name", "")
    tool_input = event.get("tool_input", {}) or {}

    reason = None
    if tool_name == "Bash":
        reason = deny_bash(tool_input.get("command", "") or "")
    elif tool_name in ("Edit", "Write"):
        reason = deny_edit_write(tool_input.get("file_path", "") or "")

    if reason:
        print(reason, file=sys.stderr)
        sys.exit(2)

    sys.exit(0)


if __name__ == "__main__":
    main()
