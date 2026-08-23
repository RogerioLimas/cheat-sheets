# Convenções do projeto (lido automaticamente pelo Claude Code)

## Regra de ouro
Nunca marcar uma task de docs/TASKS.md como concluída sem que os
testes relacionados existam e estejam passando. Sempre seguir o
ciclo: escrever teste (falhando) -> implementar mínimo -> passar ->
refatorar -> lint -> commit.

## Comandos
- Testes: `go test ./... -v`
- Lint: `golangci-lint run`
- Build: `go build ./...`
- Build cross-compile: `GOOS=windows GOARCH=amd64 go build ./...`

## Estilo
- Go idiomático, `gofmt` obrigatório, sem pacotes não usados.
- Interfaces pequenas (internal/store.Store, internal/search.Searcher
  etc.), favorecendo injeção de dependência para testes.
- Commits atômicos por task, mensagem no formato `T0XX: descrição`.

## Processo obrigatório por task
1. Ler a task correspondente em docs/TASKS.md e docs/SPEC.md.
2. Se houver ambiguidade, parar e perguntar antes de codar.
3. Escrever o(s) teste(s) primeiro e mostrar que falham.
4. Implementar o código mínimo para passar.
5. Rodar toda a suíte de testes, não só a nova.
6. Rodar lint.
7. Fazer self-review do diff como um revisor sênior de Go.
8. Marcar a task como [x] em docs/TASKS.md.
9. Commitar.