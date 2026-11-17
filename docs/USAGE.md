# Utilização — LUCIOZ OPTIMIZER

## Abrir

Com Node e as dependências instalados, faça duplo clique em `Abrir-LUCIOZ.cmd` para consultar a interface no navegador. O atalho verifica o servidor antes de abrir `http://127.0.0.1:1420/` e guarda erros em `.local/preview-launch.log`.

A pré-visualização não tem APIs Windows. Para usar inventário e operações, compile/inicie o desktop com `npm run desktop` depois de instalar os pré-requisitos do README. Consulte WINDOWS-BUILD.md para descarregar o instalador efetivamente validado.

## Otimizar com confirmação

1. No painel ou no Otimizador automático, execute a análise com **OTIMIZAR PC**.
2. Leia o estado e os detalhes de cada entrada. Inspeções não são alterações.
3. Selecione apenas as preferências que pretende alterar. Os modos LITE/BALANCED/POWER ajudam a escolher propostas; não as aplicam automaticamente.
4. Pré-visualize os valores originais e propostos e confirme a execução.
5. Consulte o resultado e o Centro de restauro.

Nesta versão os modos abrangem três preferências de animação. Não reduzem clocks nem serviços e não representam níveis de aumento de FPS.

## Aplicações de arranque

O painel de gestão direta só mostra `HKCU Run`. A desativação está limitada a executáveis opcionais reconhecidos e guarda o valor bruto e o tipo antes da remoção. Não termina a aplicação. A alteração só impede o arranque através dessa entrada; a própria aplicação pode criá-la novamente.

Reative através do Centro de restauro, categoria Arranque. Se existir uma nova entrada com o mesmo nome e conteúdo diferente, o restauro recusa sobrescrevê-la.

O inventário alargado abaixo mostra outras origens, que continuam em revisão manual.

## Gráficos do Fortnite

Feche o Fortnite. No Game Optimizer, selecione Sombras, Pós-processamento e Efeitos e peça a pré-visualização. O programa só admite valores entre 0 e 3 em opções já existentes de uma única secção `ScalabilityGroups` do ficheiro local `GameUserSettings.ini`.

Leia o conteúdo original e proposto antes de confirmar. O original completo é guardado no SQLite antes da substituição, e a API de substituição recebe um nome de backup adicional junto ao ficheiro. As cópias `.lucioz-original-*.bak` não são limpas automaticamente. Se o jogo substituir as definições, o restauro recusa sobrescrever o novo conteúdo sem o identificar.

Não são alterados executáveis, instalação ou anti-cheat. A verificação de bytes não prova que uma versão do jogo irá respeitar uma opção. GTA V, VALORANT e CS2 têm orientação manual nesta versão, sem escrita integrada.

## Medições antes/depois

Para CPU/RAM, use sessões de 15, 30 ou 60 segundos. Descreva plano de energia, carga, aplicações e condições. A recolha não presume que o computador está em repouso nem gera carga sintética. Pode cancelar; sessões canceladas não são guardadas como resultados completos.

Selecione duas sessões. O programa compara a identidade do hardware/sistema, a duração e as condições declaradas. Mesmo quando coincidem, isso não demonstra que temperatura, carga e cenário foram iguais, nem atribui diferenças a uma otimização.

Para FPS, importe um CSV PresentMon com `MsBetweenPresents`, `ProcessID` e `SwapChainAddress`. Escolha o fluxo correto e descreva as condições antes de o guardar no histórico. O backend recalcula as estatísticas. São intervalos entre chamadas Present, não necessariamente frames visíveis no ecrã. A captura ao vivo ainda não está integrada.

## Restauro

Restaure alterações recentes primeiro, individualmente, por categoria ou globalmente. Não elimine `lucioz.db` enquanto houver alterações ativas. As ferramentas oficiais de Proteção do Sistema podem ser abertas no Centro de restauro; o Windows pode pedir autorização para essa ferramenta. O LUCIOZ não marca um ponto como criado só por abrir o painel.

Eliminar ficheiros e desinstalar aplicações não são operações reversíveis pelo Centro de restauro. Alterações feitas fora do LUCIOZ também não são registadas automaticamente.

## Fluxos 0.3

Safe/Balanced/Gaming abrem propostas nativas; selecionar um modo não aplica configurações. Reveja cada proposta, deixe opções desnecessárias desmarcadas e confirme. Opções de energia podem aumentar consumo e temperatura; não garantem FPS.

Em Processador, pode selecionar um plano instalado com confirmação e restauro. Em Serviços, reveja o serviço reconhecido, dependências e motivo de elegibilidade. Só confirme não utilização se conhecer a funcionalidade; o helper UAC altera apenas o tipo de arranque permitido. Serviços não elegíveis ficam disponíveis apenas para inspeção.

As tarefas agendadas estão na vista de Serviços. Só tarefas opcionais reconhecidas do próprio utilizador podem ser desativadas; o Centro de restauro guarda o estado original.

No Game Booster, guarde um executável e prepare uma sessão. Safe não altera energia. Balanced/Gaming exigem um plano já instalado. Mantenha o LUCIOZ aberto até terminar. Se fechar, consulte o Centro de restauro; o histórico não prova que o jogo ainda esteja em execução. Um launcher que termine cedo também termina a monitorização.

A análise de ficheiros grandes não elimina ficheiros. A limpeza de temporários e da lixeira é permanente e requer confirmação. Não são incluídos documentos, Downloads ou saves.

## Fluxos 0.4

As seis preferências visuais são propostas reais: área de cliente, menus, sugestões, caixas de seleção, deslocamento de listas e desvanecimento de seleção. Não são apresentadas como ganho de FPS. Abra Otimizações, aguarde a leitura dos estados, selecione propostas e reveja a confirmação. Após aplicar/restaurar, os estados são relidos.

Se o Windows recusar uma mudança de energia sem administrador, o LUCIOZ pede UAC apenas para a operação. A interface e os jogos mantêm privilégios normais. Cancelar/recusar ou uma política que impeça a mudança produz erro, não sucesso; o original permanece registado. Um restauro de energia também pode precisar de UAC.

O relatório Cleaner distingue bytes lógicos de ficheiros cuja ausência foi verificada e a variação observada do espaço livre do volume. Outras aplicações, compressão e hard links podem afetar a diferença; não se atribui automaticamente toda a variação à limpeza. Ficheiros bloqueados, alterados, recentes ou fora da raiz permitida não são contabilizados como eliminados.

Em Game Booster, o painel de estado lê preferências Game Mode/captura e processos. Uma preferência registada não prova que o modo/captura esteja ativo. A presença de Discord, OBS ou outro processo também não prova que um overlay esteja ativo. Use os botões para rever nas Definições oficiais e leia novamente; o LUCIOZ não finge ter alterado essas opções.
