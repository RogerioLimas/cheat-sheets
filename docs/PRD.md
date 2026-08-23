# PRD: CheatSheet Launcher

## Visão do produto
App desktop leve que, ao pressionar um atalho global (ex.: Ctrl+Shift+.),
abre uma janela para o usuário buscar e visualizar cheat sheets
(linguagens, comandos, apps) sem interromper o fluxo de trabalho.

## Problema
Desenvolvedores esquecem sintaxes usadas esporadicamente (awk, vim, git,
regex). Alternar para o navegador e buscar quebra a concentração.

## Público-alvo
Desenvolvedores e power users em Windows, Linux e (opcionalmente) macOS.

## Objetivos mensuráveis (v1)
- Janela aparece em menos de 150ms após o atalho.
- Busca instantânea (<50ms) para até 200 cheat sheets locais.
- Binário final abaixo de 25MB.
- Funciona 100% offline após a importação inicial.

## Fora de escopo (v1)
- Sincronização entre múltiplas máquinas.
- Web scraping automático em tempo real (só importação manual/batch).
- Edição colaborativa online.

## Requisitos funcionais
- RF01: Hotkey global configurável (padrão Ctrl+Shift+.).
- RF02: Janela modal "always on top", fecha com Esc ou clique fora.
- RF03: Busca fuzzy em tempo real por título, tags e categoria.
- RF04: Lista de cheat sheets agrupada por categoria.
- RF05: Renderização de Markdown (tabelas, código, headers).
- RF06: Importar cheat sheet de arquivo .md local.
- RF07: Importar cheat sheet de URL (parse HTML -> Markdown), sempre
  com preview e confirmação explícita do usuário antes de salvar.
- RF08: Criar/editar cheat sheet customizado manualmente.
- RF09: Persistência local na pasta de configuração do usuário.
- RF10: Configuração de hotkey e tema (claro/escuro/sistema).

## Requisitos não funcionais
- RNF01: Multiplataforma — Windows e Linux no v1, macOS best-effort.
- RNF02: Binário estático, sem runtime externo.
- RNF03: RAM em idle abaixo de 50MB.
- RNF04: Cobertura de testes automatizados acima de 70% no core
  (parser, busca, storage).

## Critérios de aceite
- Atalho pressionado em qualquer app do SO abre a janela em foco.
- Termo de busca digitado atualiza resultados em tempo real.
- Cheat sheet selecionado renderiza Markdown corretamente.

## Riscos conhecidos
- Hotkeys globais podem conflitar com outros softwares.
- Parsing de HTML de terceiros é frágil a mudanças de layout do site.
- Fyne é uma lib relativamente jovem; alguns widgets (ex.: markdown
  rico) podem exigir bibliotecas extras.