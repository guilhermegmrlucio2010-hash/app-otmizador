# Segurança e fronteiras atuais

## Proteções implementadas em código

- Sem plugin shell, filesystem ou HTTP exposto ao frontend; CSP local.
- Sem scripts remotos, comandos de texto vindos da interface ou bypass da política PowerShell.
- Comandos de alteração específicos e identificadores fixos; parâmetros e queries SQLite validados.
- Aplicação sem elevação e instalador por utilizador. Não é instalado um serviço privilegiado.
- Nenhuma alteração de Defender, Firewall, UAC, Windows Update, autenticação, drivers, recuperação, BIOS, voltagem, clocks, pagefile ou temporizadores.
- As escritas SPI exigem confirmação e valor esperado; originais persistidos antes de qualquer tentativa de escrita.
- Limpeza limitada à pasta Temp do utilizador; sem recursão, reparse points ou ficheiros recentes; verifica bloqueios e usa um handle exclusivo para eliminar o objeto aberto.
- Terminação de processos limitada a uma allowlist, com proteção de PID do sistema e próprio processo e confirmação de perda de trabalho.
- Os executáveis dos perfis não recebem argumentos arbitrários nem passam por uma shell. A seleção de um executável não prova que é um jogo ou que é seguro: o utilizador confirma a origem.
- Desinstalação, gestão de arranque e energia encaminham para interfaces oficiais. Alterações externas não são apresentadas como restauráveis pela aplicação.

## Limitações materiais

Na versão 0.2 há duas escritas adicionais delimitadas: remoção/restauro de valores reconhecidos em HKCU Run e preferências locais do Fortnite. Não são tweaks genéricos de registo. A configuração do jogo tem backup integral no SQLite e pode criar `.lucioz-original-*.bak` junto ao ficheiro; estes backups não são apagados automaticamente. As ferramentas oficiais de Proteção do Sistema podem exigir elevação própria, sem elevar o LUCIOZ.

A classificação de arranque é baseada no caminho/nome do executável, não numa assinatura Authenticode verificada; não deve ser apresentada como prova de confiança. A validação da assinatura implementada nesta versão aplica-se ao fornecedor NVIDIA SMI. A aplicação não executa os comandos armazenados no arranque.

Para Fortnite, testar especificamente todos os estados de erro de `ReplaceFileW`, ficheiros em falta, substituição concorrente de caminhos e recuperação a partir dos backups. O patch é estrito e não altera anti-cheat, mas a integração nativa ainda não foi validada.

Este código nativo ainda não foi compilado ou executado. A revisão de código e os testes simulados não demonstram que a integração é segura em todas as configurações Windows. As seguintes validações continuam obrigatórias antes de uma release:

- Validar bindings, flags SPI e comportamento em sessões locais/remotas e com políticas de grupo.
- Verificar ACLs da base SQLite na pasta local da aplicação e recuperação de disco cheio/corrupção/WAL.
- Exercitar substituição concorrente de caminhos, reparse points ancestrais, ficheiros bloqueados, hardlinks e falhas de logging.
- Avaliar a corrida entre a consulta de início/PID e abertura do processo para terminação. Esta versão usa `sysinfo`; uma futura implementação deve manter um handle verificado durante todo o ciclo.
- Verificar assinatura Authenticode de executáveis, editores, dependências de aplicações e classificação de componentes. O backend não executa desinstaladores não verificados.
- Limitar concorrência de consultas IPC recebidas fora da interface e verificar custo da recolha de processos/discos.
- Testar WebView2, CSP, cancelamento, duplo clique e erros parciais.
- Assinar executável/instalador e efetuar revisão independente antes de distribuir.

Não existe helper de elevação por operação. Funcionalidades que precisariam dele permanecem indisponíveis. Não contornar esta limitação executando a aplicação inteira como administrador.

