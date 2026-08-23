# SPEC Técnica: CheatSheet Launcher

## Stack
- Linguagem: Go 1.22+
- UI: Fyne v2.5+
- Markdown: goldmark
- HTML -> Markdown (importação): JohannesKaufmann/html-to-markdown
- Hotkey global: golang.design/x/hotkey
- Busca fuzzy: sahilm/fuzzy
- Testes: testing nativo + testify

## Camadas
- cmd/cheatlauncher/main.go — bootstrap, registra hotkey, inicia Fyne
- internal/hotkey — interface HotkeyListener (Windows/Linux)
- internal/store — interface Store: leitura/escrita de index.json e .md
- internal/search — interface Searcher: indexação e busca fuzzy
- internal/importer — interface Importer: fetch + conversão HTML->MD
- internal/render — interface Renderer: Markdown -> widgets Fyne
- ui/ — SearchWindow, SheetViewer, SettingsPanel

## Modelo de dados
```go
type CheatSheet struct {
    ID        string    `json:"id"`
    Title     string    `json:"title"`
    Category  string    `json:"category"`
    Tags      []string  `json:"tags"`
    File      string    `json:"file"`
    SourceURL string    `json:"source_url,omitempty"`
    UpdatedAt time.Time `json:"updated_at"`
}
```
`index.json` contém os metadados; o conteúdo de cada sheet vive em
`sheets/<id>.md`.

## Fluxos principais
1. Abrir launcher: hotkey -> callback do SO -> ui.ShowSearchWindow()
   -> foco no campo de busca.
2. Buscar e visualizar: usuário digita -> search.Query(term) -> lista
   ordenada por score -> seleção -> store.LoadContent(id) ->
   render.ToWidget(markdown) -> exibido no painel.
3. Importar de URL: usuário cola URL -> confirmação explícita ->
   importer.FetchAndConvert(url) -> preview do Markdown -> usuário
   confirma -> store.Save(sheet).

## Estratégia de testes
- internal/search: ranking, string vazia, acentos, case-insensitive.
- internal/store: roundtrip do index.json, arquivo ausente/corrompido.
- internal/importer: fixtures de HTML locais (sem chamadas de rede
  reais nos testes).
- internal/render: fixtures de .md validando o modelo intermediário.
- ui: fyne/test para fluxos críticos (abrir, buscar, fechar).

## Engenharia
- CI (GitHub Actions): `go test ./...`, `go vet`, build cross-compile
  para windows/linux/darwin.
- Lint: golangci-lint.
- Versionamento semântico a partir de v0.1.0.