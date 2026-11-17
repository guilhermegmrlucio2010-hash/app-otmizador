# Segurança — LUCIOZ OPTIMIZER 0.3

## Fronteiras

A interface chama comandos tipados Tauri. Não recebe uma shell, scripts remotos, caminhos de desinstaladores para executar nem comandos arbitrários. A aplicação principal funciona como utilizador normal. Conteúdo remoto não é carregado no WebView; a política CSP limita origens.

Alterações exigem confirmação. O estado é relido antes de escrever e o original persistido. Falhas e intenções pendentes permanecem no SQLite para recuperação. O restauro bloqueia conflitos; não tenta adivinhar o que terceiros alteraram.

- SPI: três preferências de animação documentadas, sem tweaks genéricos de registo.
- Arranque: somente valores HKCU Run de aplicações opcionais reconhecidas; bytes e tipo originais preservados.
- Energia: apenas planos instalados, sem criação de planos ou alterações a parâmetros de CPU.
- Serviços: helper próprio com argumentos estritos, elevação UAC por operação, SCM e allowlist mínima. Não executa scripts como administrador. Verifica tipo, estado, triggers, dependentes e imagem reconhecida. Apenas Automático para Manual e respetivo restauro; nunca Disabled ou paragem.
- Tarefas: COM, proprietário atual, execução limitada, ação reconhecida e XML verificado. Namespace Microsoft e tarefas de atualização/segurança/manutenção protegidos.
- Processos: identidade e criticidade verificadas pelo mesmo handle; allowlist de aplicações, confirmação e aviso de trabalho não guardado.
- Temporários: extensão técnica permitida, antiguidade mínima, ficheiros diretos, metadados verificados, sem reparse points, abertura exclusiva e eliminação pelo handle. A lixeira mantém confirmação Windows. Eliminações não são reversíveis.
- Fortnite: apenas três valores existentes, jogo fechado, patch estrito, backup integral SQLite e cópia .bak, substituição atómica e verificação. Anti-cheat e ficheiros protegidos não são alterados.

Defender, Firewall, UAC, Windows Update, drivers, autenticação, armazenamento e serviços críticos são preservados. Não há overclock, RAM cleaner, desativação de pagefile, prioridade real-time ou tweaks BCD/HPET.

## Validação e limites

Consulte WINDOWS-BUILD.md para evidência efetiva de compilação/testes. Testes de escrita de serviços/tarefas criam exclusivamente fixtures num runner descartável GitHub Actions; nunca alteram serviços existentes. Os testes locais não modificam o Windows do utilizador. Testes de ficheiros usam pastas temporárias próprias.

Há testes simulados do motor para permissões, verificação, idempotência e restauro, testes de ficheiros reais e testes de integração restritos. Isto não demonstra compatibilidade com todos os sistemas Windows. Persistem riscos de interrupção de energia, disco cheio, políticas empresariais, alterações externas e launchers que terminam antes do jogo.

Não há assinatura comercial ou atualizador automático. Não são garantidos ganhos. A revisão independente e ensaios adicionais Windows 11 continuam recomendáveis antes de distribuição alargada. Guarde a base de dados e backups enquanto existirem alterações por restaurar.
