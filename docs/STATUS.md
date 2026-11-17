# Estado de implementação — 0.4.0

Aplicação gratuita e instalável. A equivalência completa com o Hone e a correspondência exata com as 38 opções das capturas não são declaradas: faltam as capturas/nomes exatos.

## Implementado e validado

- Seis preferências SPI com estado real, pré-visualização, confirmação, original SQLite, verificação e restauro: área de cliente, menus, sugestões, caixas de seleção, listas e desvanecimento de seleção.
- One-click e perfis Safe/Balanced/Gaming. Os interruptores selecionam propostas; não representam estado aplicado. Preferências são relidas após alterações/restauro.
- Planos instalados e restauro. Helper UAC estrito somente se a API recusar acesso normal; não eleva a interface ou o jogo.
- Serviços opcionais extremamente restritos via SCM, dependências/triggers, helper por operação e restauro; tarefas opcionais reconhecidas via COM.
- HKCU Run reconhecido, com restauro exato de bytes/tipo. Processos protegidos com confirmação e saída verificada.
- Cleaner restrito: ausência de cada ficheiro verificada, bytes lógicos e espaço livre observado separados; sem alegar que toda a variação do volume veio da limpeza.
- Dashboard e inventários reais, GPU NVIDIA opcional, rede ICMP, SQLite, auditoria, Steam/Epic, sessões geridas, Fortnite com backups, PresentMon CSV e benchmarks CPU/RAM preservados.
- Game Mode/captura/transparência e possíveis overlays são leituras/revisões explicitamente identificadas; não simulam atividade ou ganhos.

Validação aprovada no run 44165559878: 27 testes frontend, 31 Rust release, 28 verificações WebView2, TypeScript, lint, SQLite, PowerShell, build/NSIS, instalação/desinstalação e roundtrips nativos. Ver WINDOWS-BUILD.md para download e evidência.

## Não suportado ou pendente

- Identificar e mapear exatamente as 38 opções: aguarda nomes/capturas. O catálogo não é preenchido artificialmente para atingir esse número.
- Não existem 100 ações de otimização: grande parte do catálogo é inventário/diagnóstico, e exclusões não são otimizações.
- Escrita automática de transparência, Game Mode/capturas e políticas globais de background: revisão nas Definições oficiais. Valores registados não provam atividade.
- Serviços essenciais, partilhados, ativos, com triggers/dependentes ou fora da allowlist não são alterados.
- MSI, afinidades de dispositivos, latência gráfica genérica, NVAPI/Reflex não integrado, RAM cleaners e tweaks TCP globais: sem ação.
- Defender, SmartScreen, mitigações, VBS e atualizações de navegadores são preservados, sem ação de desativação.
- Desinstalação integrada AppX/Win32; utilizar revisão nas Definições oficiais.
- FPS em direto, sensores universais, temperatura CPU, throttling confirmado, tráfego por aplicação, impacto de arranque medido e análise longitudinal de fugas.
- GTA V, VALORANT e CS2: orientação manual, sem alterações a anti-cheat.
- Sessões acompanham apenas o processo lançado. Fecho da app/launchers que terminem cedo exigem consulta do histórico/restauro. Restauro de energia pode exigir UAC.
- Atualizador assinado, assinatura comercial e pontos de restauro automáticos.
- Sem rollback de eliminações, trabalho perdido por terminar processos, desinstalações ou alterações manuais externas. Não são limpos Downloads, documentos, saves, perfis ou dados de navegador.

Os ensaios nativos foram feitos num runner descartável Windows Server 2025, não no PC do utilizador nem em todas as versões Windows 11. A interação UAC não foi automatizada. Nenhuma configuração do Windows principal foi alterada durante o desenvolvimento.
