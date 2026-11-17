# LUCIOZ OPTIMIZER

Aplicação desktop gratuita para Windows 11 x64, em PT-PT. Tauri 2, Rust, React, TypeScript, Vite, Tailwind, Radix, Lucide, Recharts e SQLite. Implementação original, sem subscrições. A equivalência completa com o Hone ainda não foi atingida.

## Instalar

Consulte [a compilação validada e o download](docs/WINDOWS-BUILD.md). Extraia o ZIP do artefacto Windows e execute LUCIOZ OPTIMIZER_<versão>_x64-setup.exe. O instalador NSIS é por utilizador. Requer Windows x64 e WebView2 Runtime; não requer Node ou Rust para utilizar. Executável sem assinatura comercial.

Abra pelo menu Iniciar, como utilizador normal. Comece pelo Dashboard e pela análise. Alterações exigem pré-visualização e confirmação. O helper UAC é utilizado apenas numa operação administrativa suportada. Não execute toda a aplicação como administrador.

Abrir-LUCIOZ.cmd e http://127.0.0.1:1420 abrem somente o preview do navegador, sem APIs nativas ou diagnóstico do computador.

## Funcionalidades

- Dashboard com CPU, RAM, discos, processos e tráfego agregado reais; GPU NVIDIA opcional com fornecedor instalado e assinado. Dados indisponíveis são identificados.
- Catálogo: 108 entradas, das quais 105 inspeções/propriedades e três alterações visuais via API. Não são 108 otimizações.
- Otimizar PC: análise, propostas Safe/Balanced/Gaming, seleção, confirmação, original persistido, aplicação e verificação. Alterações de energia opcionais e inicialmente desmarcadas.
- Planos de energia instalados, alteração e restauro com deteção de conflitos.
- Serviços SCM: dependências, triggers e revisão conservadora. Escrita limitada a serviços opcionais reconhecidos, parados, automáticos, sem triggers ou dependentes. Apenas transição para Manual, sem parar ou desativar serviços. Confirmação de não utilização e UAC por operação.
- Tarefas COM: inventário e desativação/restauro limitado a tarefas opcionais reconhecidas do próprio utilizador, execução limitada e XML verificado.
- Arranque HKCU Run reconhecido e reversível; processos com identidade por handle, proteção e encerramento confirmado.
- Debloater: inventário Win32/AppX e revisão interativa nas Definições oficiais. Desinstalação integrada não suportada.
- Cleaner: temporários diretos antigos de extensões técnicas permitidas, pré-visualização, validação e bloqueio de reparse points. Lixeira com confirmação dupla; ficheiros grandes apenas leitura. Eliminações não são reversíveis.
- Steam/Epic, executáveis manuais e perfis. Sessões geridas aplicam energia autorizada e restauram quando o processo lançado termina. Se fechar a aplicação, restaure pelo histórico. Launchers podem terminar antes do jogo.
- Fortnite: três opções existentes de ScalabilityGroups com preview, backup integral e restauro, apenas com jogo fechado. Outros jogos têm orientação manual.
- Rede: adaptadores/DNS, tráfego agregado, ICMP com latência, jitter e perda medidos.
- FPS: CSV PresentMon por PID/swapchain, média, 1% low, percentis, histórico e exportação. Captura ao vivo não integrada.
- Benchmark CPU/RAM 15/30/60 segundos, condições declaradas e comparação; sem atribuição automática de ganhos.
- SQLite: configurações, perfis, sessões, originais, histórico e auditoria. Restauro individual/categoria/global para operações reversíveis.

## Desenvolvimento e validação

Requer Node 22.18+, Rust estável MSVC, Visual Studio C++ Build Tools, Windows SDK e WebView2.

```powershell
npm ci
npm run desktop
```

```powershell
npm run typecheck
npm run lint
npm test
npm run test:db
npm run test:windows-syntax
npm run build
cargo test --manifest-path src-tauri/Cargo.toml --release --features tauri/custom-protocol
npm run installer
```

Saída 0.3: src-tauri/target/release/bundle/nsis/LUCIOZ OPTIMIZER_0.3.0_x64-setup.exe. Executável: src-tauri/target/release/lucioz-optimizer.exe. Consulte o registo de compilação antes de assumir que o instalador está disponível localmente.

Os testes de alteração SCM/Task Scheduler e instalação executam exclusivamente num runner descartável GitHub Actions. Não os execute no Windows pessoal. Testes locais de desenvolvimento não alteram configurações Windows.

## Segurança e dados

Não desativa Defender, Firewall, UAC, Windows Update, drivers ou serviços críticos. Não executa comandos arbitrários da interface, scripts remotos, overclock, tweaks BCD/HPET ou RAM cleaners. Não promete FPS sem medição.

A base lucioz.db fica no diretório de dados local Tauri, identificador pt.lucioz.optimizer. Preserve-a e os backups até restaurar as alterações. Faça cópias com a aplicação fechada. Restaure do mais recente para o mais antigo. Eliminações, desinstalações e alterações feitas nas Definições externas não têm rollback. Pontos de restauro não são criados automaticamente.

[Estado e limitações](docs/STATUS.md) · [Arquitetura](docs/ARCHITECTURE.md) · [Utilização](docs/USAGE.md) · [Segurança](docs/SECURITY.md) · [Referência e plano](docs/REFERENCE-AND-PLAN.md)
