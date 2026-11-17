# Compilação Windows — LUCIOZ OPTIMIZER 0.4.0

Instalador gerado e validação concluída em 17/11/2026.

- Commit testado: 88e18bd006fd0c259c41d86732b73e6f09baeb8f.
- Branch: codex/windows-installer.
- Execução aprovada: https://github.com/guilhermegmrlucio2010-hash/app-otmizador/actions/runs/44165559878
- Download: https://github.com/guilhermegmrlucio2010-hash/app-otmizador/actions/runs/44165559878/artifacts/14365108357
- Requer sessão GitHub. Artefacto disponível até 01/12/2026, 20:11 UTC.
- ZIP: 7584893 bytes.
- SHA256 do ZIP: 832db7c695d43ad4baa0727d785f0660f21afc2868ebc17f3f0a7edcd38a59dd.

## Instalar

Descarregue e extraia o ZIP. Execute:

```
src-tauri/target/release/bundle/nsis/LUCIOZ OPTIMIZER_0.4.0_x64-setup.exe
```

Abra LUCIOZ OPTIMIZER pelo menu Iniciar, como utilizador normal. Requer Windows x64 e WebView2 Runtime. Não precisa de Node, Rust, Vite ou localhost:1420 para usar o instalador. A instalação é por utilizador e o EXE não tem assinatura comercial.

O binário foi gerado no GitHub, não compilado nesta pasta local. O portátil está em src-tauri/target/release/lucioz-optimizer.exe dentro do ZIP. SHA256SUMS.txt contém o hash individual do instalador.

## Resultados

| Verificação | Resultado |
|---|---|
| TypeScript / ESLint / build frontend | Passou |
| Frontend | 27 testes passaram |
| SQLite | Migrações v1/v2/v3 e integridade passaram |
| PowerShell | Sintaxe passou |
| Rust release | 31 testes passaram |
| Tauri / NSIS | Compilação e instalador passaram |
| Binário nativo / instalado | Métricas reais, SQLite em memória e inventário CIM passaram |
| Serviços / tarefas | Fixtures SCM Auto/Manual/Auto e COM Enabled/Disabled/Enabled passaram, sem alterar serviços/tarefas existentes |
| Energia com permissão | Dois planos instalados alterados e restaurados; helper estrito verificado |
| Energia sem administrador | Acesso negado (código 5) e plano intacto verificados; sem tentar abrir UAC no teste |
| Seis preferências SPI sem administrador | Cada opção foi ativada para o teste, aplicada/desativada, relida, restaurada pelo diário e reposta no estado inicial do runner |
| Arranque sem administrador | Valor descartável HKCU Run removido/restaurado com bytes/tipo exatos e eliminado; comando nunca executado |
| Processo sem administrador | Filho próprio descartável, identidade obsoleta recusada e terminação/saída confirmadas |
| Cleaner | Ficheiros temporários próprios: proteção de recentes/pessoais, bloqueios, metadados alterados, caminhos externos e ausência verificada |
| Instalação | Instalação por utilizador, self-test e desinstalação passaram |
| WebView2 | 28 verificações passaram; barreira Rust recusou uma escrita; diário de alterações permaneceu igual |

O ZIP inclui native-smoke.json, isolated-fixtures.json, installed-smoke.json, ui-native-report.json e preferences-smoke.json. A captura da interface está no artefacto lucioz-ui-diagnostics da mesma execução.

Os testes correram no Windows Server 2025 do GitHub. Não alteraram configurações do Windows pessoal do utilizador. Isto não demonstra compatibilidade com todas as configurações Windows 11. A interação com a caixa UAC real não foi automatizada; foram verificados os métodos estritos do helper e o comportamento com/sem permissão.

Não é declarada equivalência completa com o Hone. A lista/capturas das 38 opções não foi recebida. Consulte AUDIT-0.4.md para a matriz individual e STATUS.md para limites funcionais.
