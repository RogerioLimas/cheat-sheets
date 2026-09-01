# AGENTS.md — cheat-sheets (fonte única de verdade)

> `CLAUDE.md` apenas importa este arquivo (`@AGENTS.md`). Qualquer
> convenção, comando ou proibição do projeto vive aqui, não lá.
> Alinhado ao RogerOS 2.0 — Workflow Matt Pocock (Cenário 1: projeto
> novo, complexo).

## Regra de ouro

Nunca marcar uma task de `docs/TASKS.md` como concluída sem que os
testes relacionados existam e estejam passando. Sempre seguir o ciclo:
escrever teste (falhando) → implementar mínimo → passar → refatorar →
lint → commit.

Esta regra tem enforcement determinístico, não é só uma instrução: ver
`.claude/hooks/post_tool_use_lint.py` (formata automaticamente todo
`.go` editado) e a Seção "Guardrails" abaixo. A marcação `[x]` em si
continua sendo responsabilidade da sessão de execução, verificável a
qualquer momento por `/rogeros-sync docs/` (Task-Reality Check).

## Comandos

- Testes: `go test ./... -v`
- Cobertura: `go test ./... -cover -coverprofile=coverage.out`
- Lint: `golangci-lint run`
- Build: `go build ./...`
- Build cross-compile: `GOOS=windows GOARCH=amd64 go build ./...` (e `GOOS=linux`)
- Scan de segurança da configuração do agente: `npx ecc-agentshield scan`

## Estilo

- Go idiomático, `gofmt` obrigatório, sem pacotes não usados.
- Interfaces pequenas (`internal/store.Store`, `internal/search.Searcher`
  etc.), favorecendo injeção de dependência para testes.
- Commits atômicos por task, mensagem no formato `T0XX: descrição`.

## Processo obrigatório por task

1. Ler a task correspondente em `docs/TASKS.md` e `docs/SPEC.md`.
2. Se houver ambiguidade, parar e perguntar antes de codar.
3. Escrever o(s) teste(s) primeiro e mostrar que falham.
4. Implementar o código mínimo para passar.
5. Rodar toda a suíte de testes, não só a nova.
6. Rodar lint.
7. Fazer self-review do diff como um revisor sênior de Go — ou invocar
   `@go-code-reviewer`.
8. Marcar a task como `[x]` em `docs/TASKS.md`.
9. Commitar.

## Política de dependências

Bibliotecas aprovadas (ver `docs/SPEC.md`, Seção "Stack"): Fyne v2.5+,
goldmark, `JohannesKaufmann/html-to-markdown`, `golang.design/x/hotkey`,
`sahilm/fuzzy`, `testify`.

**Nenhuma dependência nova entra sem aprovação explícita do Roger.**
`go get` de um pacote fora dessa lista é bloqueado por hook (ver
`.claude/hooks/pre_tool_use_deny.py`) — se for genuinamente necessário,
pare e peça aprovação antes de tentar de outra forma.

## Boundaries (nunca tocar sem confirmação explícita)

- `git push --force`, `git push -f`, reset/rebase destrutivo em `main`.
- `rm -rf` fora de diretórios de build (`dist/`, `build/`).
- Deleção de `docs/PRD.md`, `docs/SPEC.md`, `docs/TASKS.md` — são os
  contratos de design do projeto.
- Edição de `go.mod`/`go.sum` fora do fluxo `go get`/`go mod tidy`
  aprovado.
- `.claude/settings.json`, hooks e `.claude/agents/*.md` — mudar aqui
  exige rodar `npx ecc-agentshield scan` antes do próximo PR (gate
  obrigatório, ver RogerOS 2.0 Seção 1.4 / Seção "PR/ship").

## Guardrails técnicos (não apenas instrução)

- `PreToolUse` (`.claude/hooks/pre_tool_use_deny.py`): bloqueia os
  padrões da seção Boundaries mesmo sob modo de bypass de permissões.
- `PostToolUse` (`.claude/hooks/post_tool_use_lint.py`): roda `gofmt -w`
  automaticamente em todo `.go` editado/criado. Nunca bloqueia — é
  conveniência, não gate.
- `.claude/settings.json`: `permissions.deny` como camada adicional
  (redundante de propósito com os hooks, por robustez).

## Definition of Done por fase (RNF mensuráveis)

Nenhuma fase do `docs/TASKS.md` é considerada concluída sem:

- Suite de testes completa passando (`go test ./...`).
- `golangci-lint run` limpo.
- Para fases que tocam UI/hotkey/importação: 1 ticket de
  **tracer-bullet de integração** validando o fluxo ponta-a-ponta
  mínimo antes de avançar para polish (ver `docs/TASKS.md`, T054).
- RNF04 (cobertura >70% no core): gate explícito em `docs/TASKS.md`
  (T034), não apenas menção em `docs/SPEC.md`.

## Subagentes deste projeto

- `@go-code-reviewer` — revisor read-only, roda depois de qualquer
  implementação de task, antes do commit. Também cobre coverage e
  cross-compile (Windows/Linux).
- `@spec-guardian` — verifica alinhamento com `docs/PRD.md`/`docs/SPEC.md`
  ao final de cada fase.

Não criar agente novo por tarefa pontual — ampliar o prompt de um
agente existente é preferível (Protocolo Anti-Impulso aplicado a
agentes).

## Sincronização spec ↔ código

Spec root deste projeto: `docs/` (layout Matt Pocock — `PRD.md` +
`SPEC.md` + `TASKS.md`, sem seções REASONS).

- `/rogeros-update docs/ <mudança>` — requisito/decisão → design.
- `/rogeros-sync docs/` — implementação → design, incluindo
  Task-Reality Check de `TASKS.md`.

Rodar `/rogeros-sync docs/` antes de qualquer PR onde código e spec
possam ter divergido.
