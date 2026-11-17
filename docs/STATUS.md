# Estado de implementação — 0.3.0

Aplicação gratuita. A equivalência completa com o Hone permanece um objetivo.

## Implementado em código

Dashboard/inventário reais; catálogo 108 entradas (105 inspeções/propriedades e três ações SPI); propostas Safe/Balanced/Gaming; planos instalados com restauro; gestão extremamente restrita de serviços opcionais com UAC por operação; tarefas opcionais reconhecidas via COM; arranque HKCU Run; processos protegidos; inventário de aplicações; limpeza Temp restrita; lixeira confirmada; ficheiros grandes de leitura; rede ICMP; Steam/Epic; sessões com energia temporária; três opções Fortnite com backup; FPS PresentMon importado; benchmarks CPU/RAM; SQLite e restauro.

## Não suportado ou pendente

- Não existem 100 ações de otimização: grande parte do catálogo é inventário.
- Serviços partilhados, ativos, essenciais, com triggers/dependentes não são alterados; o nome opcional não demonstra elegibilidade.
- Desinstalação integrada AppX/Win32: utilizar revisão nas Definições oficiais.
- FPS em direto, sensores universais, temperatura CPU, confirmação de throttling, tráfego por aplicação, impacto de arranque medido e análise longitudinal de fugas.
- GTA V, VALORANT e CS2: orientação manual, sem alterações a anti-cheat.
- Sessões monitorizam apenas o processo lançado. Fecho da app ou launchers que terminem cedo exigem atenção ao histórico/restauro.
- Atualizador automático assinado, assinatura comercial e pontos de restauro automáticos.
- Não há rollback de eliminações/desinstalações ou alterações em aplicações externas.
- Não são limpos Downloads, documentos, saves, perfis, dados de navegador ou caches desconhecidas.

## Validação aprovada

Execução Windows: https://github.com/guilhermegmrlucio2010-hash/app-otmizador/actions/runs/44152570438

Passaram TypeScript, ESLint, 25 testes frontend, migrações SQLite v1/v2/v3, sintaxe PowerShell, build de produção, 24 testes Rust release, Tauri/NSIS, smoke nativo, fixtures SCM/Task Scheduler, instalação/desinstalação e 21 verificações de navegação WebView2.

O instalador 0.3.0 está disponível; consulte [WINDOWS-BUILD.md](WINDOWS-BUILD.md) para download, checksum, caminho e instruções. Código compilado: 153c269a2067ff0521e3f41aae55121a72c245bb.

Os testes nativos correram no Windows Server 2025 do GitHub, com UI em privilégios normais. Nenhuma configuração do Windows pessoal do utilizador foi alterada. Testes de preferências/arranque usam adaptadores; testes de ficheiros usam fixtures; serviços/tarefas são criados e removidos exclusivamente no runner descartável. A validação não cobre todas as configurações Windows 11, jogos ou falhas de hardware.