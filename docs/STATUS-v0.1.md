# Estado de implementação — 0.1.0

## Verificações executadas nesta sessão

| Verificação | Resultado |
|---|---|
| TypeScript | Passou |
| ESLint | Passou |
| Vitest | 12 testes passaram: cálculo, importação CSV, proveniência, catálogo e renderização SSR |
| SQLite, Node `node:sqlite` | Passou: migração idempotente, intenção, histórico e unicidade |
| Scripts PowerShell | Sintaxe de hardware, tarefas e jogos validada |
| Inventário real CIM | Bloqueado: `Access denied` na sandbox |
| Rust / Cargo | Não disponíveis; testes e compilação nativos não executados |
| Instalador NSIS | Tentado; falhou em `cargo metadata`: programa não encontrado |
| Build do frontend | Passou; saída em `dist/` |
| Preview HTTP | Servidor iniciou em `127.0.0.1:1420` |
| Inspeção visual com Edge headless | Não concluída: falha de inicialização Crashpad no ambiente restrito |
| Desenvolvimento HMR | Arranque Vite funcionou, mas esbuild recusou leitura de pastas ascendentes; usar build + preview nesta sandbox |

Testes SSR não substituem testes de cliques, acessibilidade ou execução em WebView2.

## Fases

| Fase | Implementado em código | Trabalho ainda necessário |
|---|---|---|
| 1 | Estrutura Tauri/React, 18 vistas, SQLite, métricas `sysinfo`, inventário CIM, gráficos e tratamento de indisponibilidade | Compilação Rust e teste de hardware real, sensores opcionais e QA visual |
| 2 | Inventários de serviços, arranque, COM Task Scheduler e processos; detalhe SCM/dependências; terminação limitada com consentimento e identidade PID/início | Gestão reversível direta de arranque/tarefas/serviços, trigger start, assinatura/editor, grafo visual e integração real |
| 3 | 105 inspeções e 3 alterações SPI; pré-visualização, intenção persistida, verificação e restauro; testes Rust simulados escritos | Executar testes Rust/Win32, ampliar ações com evidência e compatibilidade, restauro por categoria |
| 4 | Inventário Win32/AppX, encaminhamento para desinstalação interativa oficial, limpeza Temp restrita, eventos de erro e alertas de memória/espaço | Fluxo integrado de desinstalação/reinstalação, tamanhos AppX, outras categorias de limpeza, relatórios causais, testes reais de permissões/reparse/locks |
| 5 | CPU, núcleos, RAM, pagefile, GPU/controlador por CIM; interfaces/DNS; ICMP; Steam/Epic; perfis e lançamento explícito | Sensores/VRAM/carga GPU, medição de throttling, fugas de memória longitudinais, overlays, atribuição de rede, alterações de energia, ciclo temporário de perfis |
| 6 | Importação PresentMon por PID/swapchain, médias/1% low/percentis, CSV/JSON; amostras instantâneas e comparação com condições explícitas | Captura FPS em direto, histórico persistente de sessões FPS, benchmarks controlados de duração fixa, arranque e condições automaticamente comparáveis |
| 7 | 12 testes TS/SSR, teste SQLite separado, testes Rust simulados escritos, configuração NSIS/ícones, scripts, CI e documentação | Compilar/executar Rust, testes completos de integração numa VM, revisão independente, QA WebView2, medições do custo do monitor, assinatura e instalador validado |

**Nenhuma fase é apresentada como totalmente concluída.** Existem implementações reais para parte das funções, não um produto final com todos os requisitos do pedido.

## Semântica do catálogo

Há 108 entradas únicas: 105 propriedades/inspeções e 3 alterações visuais documentadas. Estes números não representam ganhos nem 108 alterações implementadas. Fontes vazias, indisponíveis, recusadas por permissões ou campos nulos não produzem valores inventados.

Os campos de inventário não equivalem todos a diagnósticos automáticos de problemas. Por exemplo, a versão de um controlador é um dado de inventário, não prova de que existe uma versão melhor ou de que atualizar melhora o desempenho. A presença de um serviço não significa que seja desnecessário. O estado WMI de disco não substitui SMART nem testes físicos.

## Próxima validação necessária

1. Instalar os pré-requisitos e correr `cargo test`, `cargo check` e `npm run installer` num Windows com acesso aos fornecedores locais.
2. Corrigir qualquer incompatibilidade de binding/API detetada pelo compilador. O backend foi revisto por leitura e por documentação, mas não foi compilado nesta sessão.
3. Numa VM descartável, verificar o ciclo completo das três preferências SPI, incluindo recuperação após interrupção e falhas de acesso.
4. Testar a limpeza com ficheiros bloqueados, alterados entre análise e eliminação, reparse points e erros de permissão. Não usar ficheiros pessoais.
5. Só depois disponibilizar ações nativas a utilizadores finais e continuar as funcionalidades em falta.

