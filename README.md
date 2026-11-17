# LUCIOZ OPTIMIZER

Aplicação desktop para Windows 11, em PT-PT, com Tauri 2, Rust, React, TypeScript, Vite, Tailwind, componentes locais com Radix e Recharts.

**Versão inicial em desenvolvimento. A suíte pedida ainda não está completa. O frontend foi compilado e testado; o backend Rust ainda não foi compilado neste ambiente e não deve ser tratado como uma versão validada para distribuição. Não existe instalador gerado.** Consulte [o estado detalhado](docs/STATUS.md).

## Evolução 0.2.0

A base existente foi preservada e a referência pública Hone foi estudada, sem reutilizar código, imagens ou textos do produto. O [plano de equivalência](docs/REFERENCE-AND-PLAN.md) distingue funcionalidades, limites e critérios para permitir escritas.

Novos módulos implementados em código:

- Modos LITE/BALANCED/POWER que selecionam propostas e mantêm confirmação obrigatória. Atualmente abrangem apenas as três preferências visuais suportadas; não representam níveis de ganho de FPS.
- Gestão reversível de entradas opcionais reconhecidas em HKCU Run, sem mexer em drivers, HKLM, RunOnce ou mecanismos de segurança.
- Router de restauro para preferências, arranque e configurações de jogo; restauro por categoria na interface.
- Sessões CPU/RAM de 15/30/60 segundos, cancelamento, amostras persistidas e comparação de condições declaradas.
- Histórico SQLite de importações FPS, estatísticas recalculadas no backend e comparação lado a lado.
- Game Optimizer: três opções existentes de ScalabilityGroups do Fortnite, backup completo, pré-visualização e verificação de conteúdo. GTA V, VALORANT e CS2 permanecem em orientação manual.
- Fornecedor opcional NVIDIA SMI, com assinatura verificada, para utilização, VRAM e temperatura. Dados indisponíveis não são preenchidos.
- Classificação conservadora de serviços, proteção de dependências partilhadas no inventário de aplicações e leitura de triggers SCM.
- Migração SQLite v1 → v2, notificações configuráveis e abertura da ferramenta oficial Proteção do Sistema.

Estas integrações nativas continuam **pendentes de compilação e ensaios Windows**. Os 25 testes TS/SSR, testes SQLite e sintaxe PowerShell passaram; não substituem testes nativos.

## O que existe no código

- 18 vistas, pesquisa, filtros, tabelas ordenáveis, gráficos, notificações e confirmação de operações.
- Catálogo com **108 entradas: 105 inspeções de propriedades/condições e 3 alterações de preferências visuais**. Não são 108 otimizações. Parte das inspeções é inventário, sem interpretação automática.
- Inventário Windows por CIM, API SCM para detalhe dos serviços e COM para tarefas agendadas.
- Monitorização CPU, RAM, processos, volumes, uptime e tráfego agregado com `sysinfo`.
- SQLite local: alterações, auditoria, inventários guardados, amostras de benchmark, definições e perfis de jogos.
- Preferências de animação via `SystemParametersInfoW`, com intenção persistida antes da alteração, verificação, deteção de alterações concorrentes e restauro.
- Limpeza restrita a ficheiros temporários diretos com pelo menos sete dias; confirmação, validação de tamanho/data, rejeição de reparse points e abertura exclusiva antes de eliminação pelo handle.
- Inventário Win32/AppX, revisão nas Definições oficiais; não executa desinstaladores recebidos do inventário.
- Deteção Steam/Epic, perfis locais e lançamento explícito de um executável selecionado.
- Teste ICMP de 10 pedidos a um IPv4 introduzido pelo utilizador; média, perda e jitter com método indicado.
- Análise de CSV PresentMon (`MsBetweenPresents`), sem misturar processos ou swapchains; exportação CSV/JSON.

As funcionalidades nativas acima estão **implementadas em código e pendentes de compilação e validação em Windows fora da sandbox**. Não há métricas de exemplo, estimativas de ganho de FPS, desligamento de segurança, RAM boosters ou ajustes de temporizadores.

## Instalação para desenvolvimento

Pré-requisitos:

1. Windows 11 x64 e Microsoft Edge WebView2 Runtime.
2. Node.js **22.18 ou superior**. O carregador nativo de configuração usa suporte a TypeScript do Node.
3. Rust estável com o host `x86_64-pc-windows-msvc` e Cargo.
4. Visual Studio Build Tools, carga **Desktop development with C++**, MSVC e Windows SDK.

Referências: [pré-requisitos Tauri](https://v2.tauri.app/start/prerequisites/) e [instaladores Windows](https://v2.tauri.app/distribute/windows-installer/).

```powershell
npm ci
npm run check:prerequisites
npm run desktop
```

Execute como utilizador normal. Esta versão não contém um helper de elevação; operações administrativas não são disponibilizadas. Não execute toda a aplicação como administrador.

Para consultar apenas a interface:

Faça duplo clique em **`Abrir-LUCIOZ.cmd`**, ou execute `npm run open`. O atalho inicia/verifica o servidor local e abre o navegador. Não fornece acesso às APIs Windows.

```powershell
npm run build
npm run preview
```

Abra `http://127.0.0.1:1420`. O navegador mostra indisponibilidade das métricas, porque não tem a ponte Tauri. Não é uma forma de diagnosticar o computador. `npm run dev` disponibiliza desenvolvimento com HMR; nesta sandbox, a pré-compilação esbuild de dependências falhou por falta de acesso a pastas ascendentes. O build de produção e o preview não necessitam dessa pré-compilação.

## Validação

```powershell
npm run typecheck
npm run lint
npm test
npm run test:db
npm run test:windows
npm run test:windows-syntax
cargo test --manifest-path src-tauri/Cargo.toml
npm run build
```

Os testes Rust usam um adaptador simulado para verificar aplicação, restauro, falhas de permissão, verificação e recuperação de intenções interrompidas. Não alteram preferências reais. A integração efetiva das APIs Win32 requer ensaios numa VM Windows e revisão antes de publicar.

## Criar o instalador

```powershell
npm run installer
```

A configuração gera NSIS por utilizador em `src-tauri/target/release/bundle/nsis/`, depois de compilar Rust, frontend e descarregar as ferramentas de empacotamento necessárias. Os ícones estão incluídos. O executável não é assinado; não é apresentado como assinado. O workflow Windows disponibilizado em `.github/workflows/windows.yml` compila e guarda o instalador como artefacto quando executado num repositório GitHub; não foi executado nesta sessão.

## Dados e restauro

A base de dados é criada no diretório local da aplicação fornecido pelo Tauri, com identificador `pt.lucioz.optimizer` e nome `lucioz.db`. Não a elimine enquanto houver alterações por restaurar. Guarde uma cópia com a aplicação fechada para incluir o conteúdo WAL corretamente.

No Centro de restauro, restaure por ordem da alteração mais recente para a mais antiga. Entradas `pending` e `failed` continuam a guardar o original. O restauro recusa conflitos e alterações mais antigas quando existe uma alteração posterior da mesma preferência. Ficheiros eliminados, desinstalações e alterações feitas nas Definições do Windows não são revertidos pelo programa. Não são criados pontos de restauro Windows.

Os inventários guardados são evidência de diagnóstico; não são backups executáveis de serviços ou tarefas. Os perfis do Game Booster são metadados e executáveis, sem aplicação automática de definições temporárias. O Game Optimizer tem um fluxo separado para preferências locais do Fortnite. O Centro de benchmarks distingue amostras instantâneas de sessões CPU/RAM com duração controlada. Nenhum destes fluxos mede automaticamente o tempo de arranque ou captura FPS em direto.

## Documentação

- [Arquitetura](docs/ARCHITECTURE.md)
- [Guia de utilização](docs/USAGE.md)
- [Referência pública e plano de equivalência](docs/REFERENCE-AND-PLAN.md)
- [Estado por fase e limitações](docs/STATUS.md)
- [Segurança e validação necessária](docs/SECURITY.md)

