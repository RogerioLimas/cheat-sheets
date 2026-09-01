# TASKS — Backlog (marcar [x] ao concluir; cada task = 1 sessão/commit)

> Fases 1-5 são organizadas por camada técnica (bootstrap greenfield,
> sem código prévio para cortar em fatias verticais). T034 e T054 são
> os gates que compensam isso — ver `AGENTS.md`, "Definition of Done
> por fase". Não pular T034/T054 mesmo sob pressão de prazo.

## Fase 0 — Setup
- [ ] T000: go mod init + estrutura de pastas conforme SPEC.md
- [ ] T001: golangci-lint + GitHub Actions (build+test)
- [ ] T002: "Hello World" em Fyne compilando em Windows e Linux

## Fase 1 — Core de dados (TDD)
- [ ] T010: struct CheatSheet + testes de (de)serialização JSON
- [ ] T011: store: carregar index.json de fixture -> []CheatSheet
- [ ] T012: store: salvar novo CheatSheet (atualiza index + grava .md)
- [ ] T013: store: index.json ausente/corrompido não deve crashar

## Fase 2 — Busca (TDD)
- [ ] T020: indexar []CheatSheet em memória
- [ ] T021: query vazia retorna todos, agrupados por categoria
- [ ] T022: fuzzy match por título/tags com score e ordenação
- [ ] T023: normalização de acentos e case-insensitive

## Fase 3 — Renderização Markdown (TDD)
- [ ] T030: heading/parágrafo -> modelo intermediário
- [ ] T031: suporte a tabelas (formato usado no quickref.me)
- [ ] T032: blocos de código com fonte monoespaçada
- [ ] T033: integração modelo -> widget Fyne
- [ ] T034: **Gate de cobertura (RNF04)** — rodar
  `go test ./... -cover -coverprofile=coverage.out` e confirmar
  cobertura >70% em `internal/store`, `internal/search` e
  `internal/render`. Se abaixo do limite, a fase não está concluída;
  escrever os testes faltantes antes de avançar para a Fase 4.
  Entregável: saída do comando colada no commit/PR, não apenas a
  menção do requisito em `docs/SPEC.md`.

## Fase 4 — UI principal
- [ ] T040: SearchWindow — campo de busca + lista (sem hotkey ainda)
- [ ] T041: SheetViewer — painel de conteúdo ao selecionar item
- [ ] T042: Esc fecha, ↑/↓ navega, Enter seleciona
- [ ] T043: teste fyne/test simulando digitação e seleção

## Fase 5 — Hotkey global
- [ ] T050: abstração + implementação Windows
- [ ] T051: implementação Linux (avaliar limitações X11/Wayland)
- [ ] T052: integrar hotkey -> show/hide da SearchWindow
- [ ] T053: tela de configuração da combinação de tecla
- [ ] T054: **Tracer-bullet de integração** — validar manualmente e com
  teste automatizado (fyne/test onde possível) o fluxo ponta-a-ponta
  mínimo do produto: hotkey pressionado -> SearchWindow abre em foco
  -> busca digitada -> item selecionado -> SheetViewer renderiza o
  Markdown. Esta task existe porque as Fases 1-5 foram construídas por
  camada (dados -> busca -> render -> UI -> hotkey), não por fatia
  vertical — ver `AGENTS.md`, seção "Definition of Done por fase", e
  RogerOS 2.0 (Workflow Matt Pocock, Cenário 1, exceção de bootstrap
  greenfield). Nenhuma task da Fase 6 ou 7 começa antes desta passar.

## Fase 6 — Importação
- [ ] T060: parse de fixture HTML -> Markdown (TDD, sem rede no teste)
- [ ] T061: fluxo de UI: colar URL -> preview -> confirmar -> salvar
- [ ] T062: importar arquivo .md local

## Fase 7 — Polish
- [ ] T070: diretório de config multiplataforma
- [ ] T071: tema claro/escuro/sistema
- [ ] T072: empacotamento (fyne package): .exe/installer e AppImage/.deb
- [ ] T073: checklist de teste manual end-to-end

## Fase 8 — Release
- [ ] T080: README + GIF de demonstração
- [ ] T081: tag v1.0.0, artifacts via CI