# Auditoria funcional 0.4 — ações reais e evidência

## Âmbito e dados em falta

As capturas referidas pelo utilizador e a lista exata das 38 opções não estão nos ficheiros recebidos nesta conversa/projeto. Foi pedida a lista dos nomes ou o caminho das capturas. Não são inventados 38 nomes nem é declarada correspondência com capturas não vistas. Os tópicos pedidos que já são identificáveis estão no catálogo individualmente; opções perigosas não têm ação.

## Alterações desta versão

- Seis preferências visuais via SystemParametersInfoW. Três existentes preservadas e três novas: caixas de seleção, listas e desvanecimento de seleção. São preferências de movimento, sem promessa de FPS.
- Compatibilidade por consulta da API na sessão atual. Uma API indisponível bloqueia escrita; um erro numa preferência não esconde as restantes. Interruptores selecionam propostas e ficam indisponíveis sem um estado detetado.
- Leitura automática das preferências na vista de otimizações e releitura depois de aplicar/restaurar, incluindo one-click e modos.
- Limpeza: verifica ausência após eliminar pelo handle. Relatório distingue bytes lógicos removidos de variação de espaço livre medida no volume. Não atribui toda a variação do volume à limpeza. Falhas/bloqueios/alterações concorrentes não são sucessos.
- Processos: aguarda o handle sinalizado após TerminateProcess. Pedido aceite sem saída confirmada é erro explícito.
- Gaming: preferências Game Mode/captura identificadas como valores registados, sem fingir atividade. Transparência por getter WinRT quando disponível. Processos com possível funcionalidade de overlay e maior carga, sem encerramento automático.
- Catálogo de exclusões explícitas: MSI, afinidades, latência gráfica, NVAPI/Reflex não integrado, standby, tweaks de rede, Defender, SmartScreen, mitigações, VBS, atualizações de navegador e políticas globais de background.

## Testes e critérios

Testes locais não alteram configurações do Windows pessoal. Os ensaios nativos são exclusivos de um runner Windows descartável. SPI, arranque, processo e UI executam com integridade média. SCM e alterações de energia são testados com permissão; a recusa de energia sem administrador é verificada separadamente. O programa distribuído não inclui gsudo, WebDriver ou um endpoint IPC para executar os ensaios.

Cada preferência SPI é exercitada com estado inicial conhecido, alteração real, releitura, restauro pelo diário e recuperação do valor inicial do runner. Mocks cobrem permissões recusadas, estado obsoleto, API não suportada, ausência de verificação e preservação do original. Os testes existentes de intenções interrompidas e idempotência são preservados.

Energia: mudança entre planos realmente instalados e restauro com permissão, incluindo o helper estrito; ensaio separado da recusa sem administrador sem abrir UAC. A interface só eleva a operação quando a API devolve acesso negado. Arranque: valor HKCU Run criado exclusivamente para o teste, remoção/restauro com bytes/tipo exatos e limpeza final, sem executar o comando. Processos: cópia própria do executável a aguardar, identidade errada recusada e saída confirmada. Serviços/tarefas mantêm os ensaios SCM/COM de fixtures anteriores.

Cleaner: ficheiros próprios temporários envelhecidos, ficheiros pessoais/recentes preservados, bloqueios, metadados obsoletos, caminhos fora da raiz e contabilidade apenas de eliminações verificadas.

## Matriz de operações

Validação aprovada: run 44165559878, commit 88e18bd006fd0c259c41d86732b73e6f09baeb8f. Consulte WINDOWS-BUILD.md para o instalador. Implementação não equivale a validação em todos os Windows 11.

| Nome | Estado técnico | Método | Testes | Reversão |
|---|---|---|---|---|
| Animação da área de cliente | Funcional, ensaio 0.4 aprovado | SPI GET/SETCLIENTAREAANIMATION | Mocks por opção + roundtrip nativo | Original SQLite + GET |
| Animação dos menus | Funcional, ensaio 0.4 aprovado | SPI GET/SETMENUANIMATION | Mocks por opção + roundtrip nativo | Original SQLite + GET |
| Animação das sugestões | Funcional, ensaio 0.4 aprovado | SPI GET/SETTOOLTIPANIMATION | Mocks por opção + roundtrip nativo | Original SQLite + GET |
| Animação das caixas de seleção | Funcional, ensaio 0.4 aprovado | SPI GET/SETCOMBOBOXANIMATION | Mocks por opção + roundtrip nativo | Original SQLite + GET |
| Deslocamento suave de listas | Funcional, ensaio 0.4 aprovado | SPI GET/SETLISTBOXSMOOTHSCROLLING | Mocks por opção + roundtrip nativo | Original SQLite + GET |
| Desvanecimento da seleção | Funcional, ensaio 0.4 aprovado | SPI GET/SETSELECTIONFADE | Mocks por opção + roundtrip nativo | Original SQLite + GET |
| Planos de energia instalados | Funcional, ensaio 0.4 aprovado | PowrProf | GUID estrito + alteração/restauro nativo | GUID original + verificação |
| Arranque opcional HKCU Run | Funcional, ensaio 0.4 aprovado | API de registo, allowlist | Política + fixture real sem administrador | Bytes e tipo originais |
| Serviço opcional elegível | Funcional restrito, CI 0.4 aprovado | SCM, helper UAC estrito | Allowlist/dependências + fixture Auto/Manual/Auto | Tipo e arranque diferido originais |
| Tarefa opcional elegível | Funcional restrito, CI 0.4 aprovado | Task Scheduler COM | Proprietário/XML + fixture Enable/Disable/Enable | Enabled original, XML verificado |
| Terminação de processo opcional | Funcional, ensaio 0.4 aprovado | Handle, criticidade, identidade, espera | Proteções + processo próprio descartável | Não recupera trabalho perdido |
| Temporários antigos permitidos | Funcional, ensaio 0.4 aprovado | Validação/exclusividade/eliminação por handle | Ficheiros próprios antigos, bloqueados e alterados | Não reversível; confirmação obrigatória |
| Transparência | Apenas diagnóstico/revisão | UISettings.AdvancedEffectsEnabled; Definições oficiais | Getter opcional/sintaxe; pode estar indisponível | Alteração manual externa |
| Game Mode | Apenas diagnóstico/revisão | Preferência HKCU; Definições oficiais | Inventário e UI; não verifica atividade no jogo | Alteração manual externa |
| Captura em segundo plano | Apenas diagnóstico/revisão | Permissão HKCU; Definições oficiais | Inventário e UI; não verifica gravação ativa | Alteração manual externa |
| Aplicações de fundo/overlays | Apenas diagnóstico; fecho manual separado | Inventário de processos | IPC/monitorização; presença não prova overlay | Não fecha automaticamente |
| MSI / afinidades de dispositivos | Não suportada | Driver/INF e política Windows preservados | Ausência de ação e backend allowlist | Sem alteração |
| Latência gráfica / NVAPI / Reflex | Não suportada para escrita | Orientação por jogo/controlador | Sem ação genérica nem registo inventado | Sem alteração |
| Standby/RAM cleaners e tweaks TCP | Não suportada | Diagnóstico de RAM/rede preservado | Sem ação de escrita | Sem alteração |
| Defender, SmartScreen, mitigações, VBS | Não suportada para desativação | Proteções preservadas | Catálogo sem ação; allowlist rejeita IDs | Sem alteração |
| Atualizações de navegador | Não suportada para desativação | Tarefas de update protegidas | Classificação/política | Sem alteração |
| FPS em direto | Não suportada | Importação PresentMon preservada | Estatísticas e proveniência testadas | Não altera Windows |

## Fontes primárias

- SystemParametersInfoW: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow
- Transparência, getter sem setter: https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.advancedeffectsenabled
- Páginas oficiais de definições: https://learn.microsoft.com/en-us/windows/apps/develop/launch/launch-settings
- MSI e INF: https://learn.microsoft.com/en-us/windows-hardware/drivers/kernel/enabling-message-signaled-interrupts-in-the-registry
- Afinidade de interrupções: https://learn.microsoft.com/en-us/windows-hardware/drivers/kernel/interrupt-affinity-and-priority
- NVIDIA Reflex SDK: https://developer.nvidia.com/performance-rendering-tools/reflex

A documentação de drivers não constitui uma recomendação para aplicar as suas chaves num otimizador genérico. Preferências documentadas também podem ser ignoradas visualmente por uma aplicação específica; a verificação confirma a configuração, não uma melhoria de FPS.

## Evidência final

Passaram 27 testes frontend, 31 Rust release e 28 verificações WebView2. O artefacto inclui preferences-smoke.json com cada um dos seis IDs, leitura inicial, aplicação, restauro pelo diário e estado final. Energia testada com permissão e acesso negado sem administrador; arranque e processo próprio testados sem administrador. Instalação/desinstalação aprovadas. Nenhuma configuração do Windows pessoal foi alterada. A caixa UAC interativa não foi automatizada.
