# Auditoria e critérios da versão 0.3

O código da versão 0.2 foi revisto por domínio: frontend, comandos Tauri, fornecedores Windows, persistência, testes, scripts e empacotamento. node_modules e ícones gerados são dependências/artefactos, não código de produto para reescrever.

## Problemas identificados

- One-click executa inventário mas não gera um plano final automaticamente.
- Perfis LITE/BALANCED/POWER apenas propõem três preferências visuais; SAFE/CUSTOM/ADVANCED nas Definições não alteram o comportamento do motor.
- Serviços/tarefas só permitem leitura; as opções não suportadas devem ficar explícitas.
- Game Booster guarda metadados e lança o executável sem um ciclo temporário de configurações.
- Debloater encaminha para Definições; não verifica remoção nem preservação de dependências num fluxo próprio.
- Cleaner não tem análise de lixeira nem pesquisa de ficheiros grandes.
- Processos não são atualizados automaticamente ao permanecer na vista.
- Estatísticas/CSV FPS são reais, mas falta integração de captura em direto. Sem fornecedor integrado, a captura deve permanecer não suportada.
- Diálogos de detalhes usam rótulos de confirmação de operações; links externos no WebView não têm um abridor documentado próprio.
- README/STATUS contradizem a compilação Windows bem-sucedida da 0.2.
- Faltam testes interativos de interface e de instalação em ambiente Windows isolado.

## Critérios de entrega

1. Preservar e migrar todas as tabelas existentes sem perda de históricos.
2. Ações com allowlist, original persistido antes de escrita, releitura e conflitos explícitos.
3. Safe/Balanced/Gaming descrevem ações reais. Selecionar perfil não executa mudanças.
4. Serviços críticos, partilhados, com dependências ou triggers inadequados não são modificados. Nenhum serviço é iniciado, parado ou desativado em massa.
5. Elevação limitada ao helper de configuração de serviços permitidos; sem comandos, paths ou scripts administrativos arbitrários.
6. Testes locais não alteram configurações do computador do utilizador. Os testes de escrita usam adaptadores ou fixtures do projeto e os ensaios administrativos decorrem apenas em runner isolado.
7. Lint, TypeScript, testes frontend, migrações, Rust release, build, teste de leitura do binário e empacotamento devem passar.
8. O instalador deve ser disponibilizado com checksum e limitações reais. Uma build bem-sucedida não equivale a todas as funções testadas em todos os PCs Windows 11.

## Resultado da auditoria

Critérios de compilação, testes e empacotamento aprovados no run 44152570438. As falhas de UI intermédias foram investigadas e resolvidas reduzindo os privilégios do processo de teste. One-click, perfis, serviços/tarefas restritos, energia temporária, lixeira, ficheiros grandes, atualização de processos e rótulos/links foram implementados ou corrigidos. Debloater integrado, captura FPS ao vivo e restantes limitações do STATUS.md permanecem explicitamente não suportados.

Foram acrescentados testes reais de substituição atómica/ficheiro bloqueado e proteção da ordem de restauro por alvo, além de fixtures SCM/COM e navegação no WebView2. Nenhuma configuração Windows do utilizador foi alterada durante estes testes.
