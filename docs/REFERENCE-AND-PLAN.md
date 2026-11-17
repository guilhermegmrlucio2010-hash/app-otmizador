# LUCIOZ OPTIMIZER — referência pública e plano

Consulta: 17 de novembro de 2026. Apenas páginas públicas; nenhum ficheiro da aplicação Hone foi utilizado. Identidade, textos de produto, ícones e código são próprios do projeto ou das dependências declaradas.

## Referências consultadas

- [Site público em português](https://hone.gg/pt): categorias gerais de sistema, GPU, jogos, atividade de fundo e restauro. As alegações de desempenho da referência não são metas verificadas do LUCIOZ.
- [Using Hone](https://support.hone.gg/hc/en-gb/articles/4758186993183-Using-Hone): painel, filtros por finalidade, ferramentas de manutenção, biblioteca de jogos e revisão de opções por jogo fechado.
- [Game Mode](https://support.hone.gg/hc/en-gb/articles/6031122574367-Game-Mode-What-It-Is-How-to-Use-It): gestão temporária de aplicações escolhidas e perfis de sessão. O LUCIOZ exige autorização explícita por aplicação e não assume que todos os processos de fundo prejudicam o jogo.
- [Atualização 1.4](https://hone.gg/blog/hone-1-4-update-boost-ups-and-merged-optimizations/): organização em grupos, perfis editáveis, pré-visualizações e progresso de manutenção.
- [Restauro de opções de jogos](https://support.hone.gg/hc/en-gb/articles/4758246456607-Revert-Game-Settings-Changes): necessidade de recuperar configurações. O LUCIOZ preservará os ficheiros originais em vez de os eliminar para “repor”.

## Equivalência funcional planeada

| Área | Fluxo original LUCIOZ | Critério para disponibilizar escrita |
|---|---|---|
| Dashboard | Métricas e leituras com origem, idade e indisponibilidade explícitas | Leitura apenas |
| Otimizações | Catálogo por categoria, evidência, risco, seleção e comparação de valores | API documentada, estado verificável e original persistido |
| Serviços | Inventário SCM/CIM, dependências, triggers e revisão por finalidade | Validação de uso/dependências, allowlist e elevação isolada antes de habilitar alterações |
| Debloater | Inventário, distinção entre aplicações e dependências, remoção interativa | Método de desinstalação validado; nunca executar comandos arbitrários do inventário |
| Game Booster | Checklist, aplicações autorizadas e perfil por sessão | Identidade de processos e restauro de cada alteração temporária |
| Game Optimizer | Perfis locais para jogos reconhecidos e opções suportadas | Documentação da opção, jogo fechado, backup byte a byte e confirmação de diferenças |
| FPS | CSV real por processo/swapchain, estatísticas, histórico e condições | Captura ao vivo só após integração validada com fornecedor documentado |
| Rede | Adaptadores, DNS, tráfego e testes ICMP explícitos | Sem tweaks TCP/IP experimentais |
| Arranque | Entradas do utilizador reconhecidas, desativação e restauro | Chave Run suportada, aplicação opcional reconhecida, verificação de conflitos |
| Processos | Recursos medidos e encerramento manual | Processo reconhecido, identidade verificada e aviso de trabalho não guardado |
| Cleaner | Pré-visualização de ficheiros, tamanho e resultado por ficheiro | Âmbito limitado, verificação de identidade, reparse points e bloqueios |
| Boost Modes | LITE/BALANCED/POWER selecionam propostas; nunca aplicam ao selecionar | Só ações realmente implementadas e confirmação individual |
| Restauro | Individual, por categoria e global, ordem inversa e conflitos | Original durável antes de qualquer escrita |
| Benchmarks | Sessões de duração conhecida, condições e comparação explícita | Sem atribuição causal automática de diferenças |
| Definições | Preferências locais, acessibilidade, logs e capacidades | Atualizações só com mecanismo de assinatura e origem definidos |

## Ordem de execução

1. Preservar a base e testes existentes; atualizar a identidade e navegação.
2. Isolar os novos módulos e criar migrações SQLite versionadas.
3. Arranque reversível e router de restauro; modos de propostas com allowlist.
4. Benchmarks de duração fixa, cancelamento e histórico de FPS com condições.
5. Expandir serviços, debloater e opções de jogos apenas com documentação e testes apropriados. Não apresentar revisão manual como escrita integrada.
6. Executar TypeScript, lint, testes e build; compilar/testar Rust e gerar NSIS quando a toolchain estiver disponível.

Não serão replicadas opções da referência que conflituem com os requisitos, incluindo desativar mitigations/VBS, modificar MSI/afinidades, temporizadores ou aplicar afinações experimentais da rede. O número de entradas nunca será usado como prova de melhorias de desempenho.
