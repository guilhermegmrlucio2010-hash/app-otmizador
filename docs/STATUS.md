# Estado de implementação — LUCIOZ OPTIMIZER 0.2.0

A versão anterior foi preservada e evoluída. Este documento substitui o estado da 0.1, arquivado em STATUS-v0.1.md. A equivalência completa com o Hone ainda não foi atingida.

## Verificações atuais

| Verificação | Resultado |
|---|---|
| TypeScript | Passou |
| ESLint | Passou |
| Vitest | 25 testes passaram: cálculo, CSV, proveniência, modos, comparabilidade, classificação e renderização de vistas |
| SQLite | Passou: migração inicial idempotente, atualização v1 para v2 preservando dados, diário original, unicidade e sessões |
| PowerShell | Sintaxe validada para hardware, tarefas, jogos e fornecedor GPU |
| Rust/Cargo/MSVC | Ausentes; backend e testes Rust não compilados/executados |
| Integração Windows real | Pendente; a consulta CIM anterior foi recusada pela sandbox |
| Instalador NSIS | Tentado novamente; bloqueado em cargo metadata: programa não encontrado |
| Build frontend | Passou; dist contém a interface de produção |
| QA visual/WebView2 | Pendente; os testes SSR não substituem cliques nem execução desktop |

## Estado por fase

| Fase | Código disponível | Ainda em falta |
|---|---|---|
| Referência e plano | Site e documentação pública Hone estudados; plano original em REFERENCE-AND-PLAN.md | Não existe declaração de equivalência total |
| Interface/dashboard | Identidade LUCIOZ OPTIMIZER, navegação com todos os módulos pedidos e vistas de diagnóstico adicionais, PT-PT, métricas sysinfo e GPU NVIDIA opcional | QA visual, validação WebView2, sensores CPU e fornecedores AMD/Intel |
| Motor/restauro | Três preferências SPI, intenção antes da escrita, verificação, conflitos, router por domínio, restauro individual/categoria/global | Ensaios Win32 de falhas, robustez de disco e helper de elevação por operação |
| Arranque/processos | Inventário alargado, desativação/restauro de HKCU Run reconhecido, terminação limitada e confirmada | Assinaturas/impacto medido do arranque, outras origens, identidade de processo mantida por handle |
| Serviços/tarefas | Inventário, detalhe SCM, dependências, triggers e classificação por família sem desativação automática; COM para tarefas | Alterações e restauro integrados de serviços/tarefas, grafo visual e elevação isolada |
| Debloater/cleaner | Inventário Win32/AppX, classificação conservadora, desinstalação interativa nas Definições; limpeza Temp restrita com pré-visualização e verificação | Desinstalação integrada com dependências, lixeira, outros caches e procura segura de ficheiros grandes |
| Gaming | Steam/Epic, executáveis manuais, perfis de lançamento, checklist; Game Optimizer com três preferências locais do Fortnite e backup integral | Ciclo de alterações temporárias por jogo, mais jogos com opções validadas; GTA V/VALORANT/CS2 são revisão manual |
| Rede/GPU/memória | ICMP, adaptadores/DNS, tráfego agregado, RAM/pagefile, NVIDIA SMI com assinatura verificada | Tráfego por aplicação, sensores universais, throttling e análise longitudinal de fugas |
| FPS/benchmarks | CSV PresentMon separado por fluxo, médias/1% low/percentis, histórico SQLite, estatísticas recalculadas, exportação; sessões CPU/RAM 15/30/60 s canceláveis e comparação de condições | Captura FPS ao vivo, tempo de arranque e controlo automático de condições térmicas/cenário |
| Modos/definições | LITE/BALANCED/POWER selecionam propostas reais; notificações, movimento reduzido, frequência de recolha, idioma/tema identificados | Atualizações automáticas assinadas, mais idiomas/temas e mais ações validadas nos modos |
| Distribuição | Configuração NSIS por utilizador, ícones, scripts, CI, documentação e atalho persistente de preview | Compilar, executar testes nativos, gerar e testar instalador e assinar a distribuição |

## Catálogo e ações

O catálogo mantém 108 entradas: 105 propriedades/inspeções e três alterações visuais. Isso não equivale a 108 otimizações. Gestão de entradas de arranque e configuração local do Fortnite são operações adicionais fora do motor de preferências visuais, com os seus próprios fluxos e backups.

Parte das entradas é inventário, não diagnóstico automático de um problema. O nome/estado de um serviço não demonstra que pode ser desativado. A versão de um controlador não prova que está desatualizado. O estado WMI do disco não substitui SMART.

Os modos LITE/BALANCED/POWER abrangem somente as três ações visuais do catálogo nesta versão. A palavra POWER não representa uma promessa de desempenho ou autorização para alterações de risco.

## Condições de entrega

A interface e os testes que não dependem do Windows nativo estão disponíveis e passaram. Nenhum componente nativo novo é apresentado como validado em produção. Não existe um instalador EXE/MSI gerado nesta sessão. A restrição de rede e a ausência de toolchain impedem concluir esse passo aqui.

Para continuar a validação: instalar Rust estável MSVC e Visual Studio C++ Build Tools/Windows SDK, executar cargo test e npm run installer, corrigir quaisquer erros de bindings e ensaiar arranque, SPI, GPU e ficheiros numa VM Windows antes de distribuir. As funcionalidades ainda em falta acima permanecem trabalho de implementação, não são apenas bloqueios de compilação.
