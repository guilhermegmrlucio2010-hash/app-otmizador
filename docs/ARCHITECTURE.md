# Arquitetura

```text
React / TypeScript / Tailwind / Radix / Recharts
                     │ IPC tipado, comandos específicos
                  Tauri 2
                     │
      ┌──────────────┼───────────────────────┐
   Monitor      Inventário                 Operações
   sysinfo      CIM / COM / SCM             SPI / ficheiros / processos
      │             │                          │
      └─────────────┴────────────┬─────────────┘
                             SQLite
                   intenção / resultado / original
```

## Frontend

`src/api.ts` é a fronteira IPC. Recusa chamadas quando não existe Tauri. Não existe fallback com hardware simulado. `src/types.ts` descreve as estruturas principais. Os inventários variáveis usam linhas de propriedades de origem e conservam os valores comunicados.

`src/catalog.ts` contém as definições e a extração de evidência, separando inspeções de alterações. `src/fps.ts` importa o CSV PresentMon e separa os fluxos por PID e swapchain. `src/components` contém cartões, tabelas, indicador de risco, botão com variantes, gráficos e diálogos acessíveis Radix. `App.tsx` coordena as vistas e pedidos. Um ErrorBoundary contém falhas de renderização.

Os estilos e ícones são locais. Não é usado CDN nem conteúdo remoto no WebView. Os perfis SAFE/CUSTOM/ADVANCED são preferências da interface nesta versão e não habilitam permissões adicionais.

## Backend

- `system_info`: estado persistente de `sysinfo`, métricas e processos. As taxas de rede são deltas divididos pelo tempo real entre recolhas.
- `windows_inventory`: seleção fechada de fornecedores. Executa PowerShell do diretório System32 com scripts empacotados, sem perfil, sem interação e sem políticas de execução alteradas. Trata UTF-8, JSON, limite de saída e timeout de 45 segundos.
- `scripts/hardware.ps1`: CIM e leitura de algumas preferências, sem escrita no registo.
- `scripts/tasks.ps1`: API COM Schedule.Service, leitura recursiva das pastas.
- `scripts/games.ps1`: manifestos locais Steam e Epic; não lê credenciais.
- `service_manager`: consulta nativa SCM com `windows-service` e enumeração de dependentes Win32.
- `process_manager`: terminação limitada a nomes reconhecidos, validação de início/PID, sem elevação.
- `optimization_engine`: allowlist fechada de seis preferências SPI; adaptador real e adaptador de testes.
- `disk_cleaner`: análise sem recursão, exclusão de reparse points, verificação de idade/identidade, abertura exclusiva e eliminação pelo handle.
- `network_monitor`: validação `Ipv4Addr`, ICMP através de `System.Net.NetworkInformation.Ping`, sem strings de comandos recebidas da interface.
- `game_manager`: validação de executáveis .exe absolutos, regulares, fora do diretório Windows.
- `storage`: migração inicial SQLite, queries parametrizadas e tabelas permitidas explicitamente.
- `main`: registo de comandos IPC, persistência, lançamento de perfis e allowlist de páginas `ms-settings:`.

Na versão 0.2, `startup_manager`, `benchmark_engine`, `rollback_manager`, `session_commands` e `game_optimizer` foram separados. CPU, memória, tarefas e inventário de aplicações continuam parcialmente consolidados nos fornecedores. Não foram criados ficheiros vazios para simular modularidade.

## Novos fluxos 0.2

- `startup_manager`: apenas HKCU Run, seleção conservadora de executáveis opcionais, backup do tipo/bytes e remoção verificada; restauro recusa colisões com entradas externas. Referência: [Run/RunOnce — Microsoft](https://learn.microsoft.com/en-us/windows/win32/setupapi/run-and-runonce-registry-keys).
- `benchmark_engine`: estatísticas FPS puras e recolha CPU/RAM temporizada com monitor próprio, para não reutilizar deltas do painel. `session_commands` limita durações, serializa sessões, permite cancelamento e recalcula importações.
- `game_optimizer`: caminho Fortnite fixo, ficheiro limitado, UTF-8, rejeição de reparse points, jogo fechado, patch estrito de três opções já existentes e backup completo. Usa `ReplaceFileW` com nome de backup adicional. Falhas de substituição precisam de ensaios Win32; a existência de SQLite não elimina falhas de disco ou de APIs.
- `rollback_manager`: encaminha cada tipo de alteração para o adaptador correto. A categoria é persistida em SQLite v2 e utilizada na interface.
- `scripts/gpu.ps1`: só executa NVIDIA SMI no caminho suportado depois de validar a assinatura; não descarrega nem instala fornecedores.
- `src/features`: componentes isolados de modos, arranque, medições temporizadas, histórico FPS, GPU e configuração de jogos. Modelos puros cobrem comparabilidade e classificação de serviços/aplicações.

As opções Fortnite são fundamentadas pela [referência de escalabilidade da Unreal Engine](https://dev.epicgames.com/documentation/en-us/unreal-engine/scalability-reference-for-unreal-engine) e pela [localização de configurações documentada pela Epic](https://www.epicgames.com/help/c-34254770/c-37371353/a22311370). Isso não equivale a uma garantia de que todas as versões do Fortnite respeitam essas opções.

## Ciclo de alteração

1. A pré-visualização lê o valor por SPI.
2. O utilizador confirma os valores concretos.
3. Um mutex serializa as alterações/restauros.
4. O backend valida o identificador e compara o valor esperado com o atual.
5. Insere o original e o novo valor em `changes`, com estado `pending`, antes da escrita.
6. Chama a API suportada, persiste no perfil e notifica a alteração.
7. Relê o estado. Só marca `applied` depois de verificar; falhas conservam o original.
8. O restauro verifica conflitos e a ordem de alterações, reaplica o original e confirma a leitura.

O restauro é por preferência efetiva, não por bytes de registo. O motor não altera diretamente o registo. Não são prometidos ganhos de FPS para nenhuma destas ações.

Referência: [SystemParametersInfoW — Microsoft](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow). O parser de FPS segue a coluna documentada em [PresentMon Capture Application](https://github.com/GameTechDev/PresentMon/blob/main/README-CaptureApplication.md).


## Novos módulos 0.3

power_manager utiliza APIs PowrProf para planos instalados e restauro. profile_engine gera propostas Safe/Balanced/Gaming com estados reais. service_changes limita escritas SCM e eleva somente o helper CLI estrito, antes da inicialização Tauri. scheduled_tasks usa COM e comparação da definição. desktop_commands desloca operações para workers e serializa mutações com o mutex de operações.

game_sessions mantém uma única sessão gerida, acompanha o Child e restaura energia no fim. A migração 003 adiciona o diário de sessões; o restauro não depende do processo sobreviver a uma reinicialização. storage_tools implementa lixeira e análise limitada de ficheiros grandes. isolated_tests contém fixtures exclusivamente para runners GitHub Windows descartáveis.

O workflow valida fonte, testes Rust release, instalador, smoke nativo, fixtures, instalação/desinstalação e navegação WebView2 de leitura. Evidência efetiva em WINDOWS-BUILD.md.

## Fluxos 0.4

As preferências SPI são inventariadas por opção, com erros independentes. O filtro nativo do ensaio de leitura recusa mutações antes do handler de comandos e verifica o diário. Helpers de energia e serviços têm rotas CLI estritas anteriores à inicialização Tauri; não elevam a interface ou o jogo. A contabilidade de limpeza tem um contrato tipado que distingue ausência verificada e espaço livre do volume. Consultar AUDIT-0.4.md para a matriz de testes e as exclusões.
