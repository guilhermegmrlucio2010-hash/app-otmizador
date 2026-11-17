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

## Validação

Local: TypeScript, ESLint, 25 testes frontend, migrações SQLite v1/v2/v3, sintaxe PowerShell e build frontend passaram. Rust/MSVC não estão disponíveis localmente; compilação nativa e instalador são produzidos no GitHub Windows.

A versão 0.2 teve compilação/instalador e 15 testes Rust aprovados. A versão 0.3 acrescenta fixtures Windows descartáveis, testes de substituição atómica de ficheiros, instalação/desinstalação e navegação WebView2. Consulte WINDOWS-BUILD.md para a execução efetivamente concluída. Uma compilação no Windows Server CI não prova compatibilidade com todos os PCs Windows 11.
