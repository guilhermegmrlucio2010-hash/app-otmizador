# Compilação Windows — LUCIOZ OPTIMIZER 0.3.0

Instalador gerado e validação concluída em 17/11/2026.

- Código compilado: 153c269a2067ff0521e3f41aae55121a72c245bb.
- Branch: codex/windows-installer.
- Execução aprovada: https://github.com/guilhermegmrlucio2010-hash/app-otmizador/actions/runs/44152570438
- Download do ZIP: https://github.com/guilhermegmrlucio2010-hash/app-otmizador/actions/runs/44152570438/artifacts/14360266539
- É necessário iniciar sessão no GitHub. Artefacto disponível até 01/12/2026, 18:58 UTC.
- ZIP: 7552718 bytes.
- SHA256 do ZIP: 68de9fb3ff8d8de7ab967b20079ea9b42605ea807864177b48ea6f100f0d2739.
- Captura da interface: artefacto lucioz-ui-diagnostics na mesma execução.

## Onde está o EXE

Dentro do ZIP descarregado:

```
src-tauri/target/release/bundle/nsis/LUCIOZ OPTIMIZER_0.3.0_x64-setup.exe
```

Executável sem instalação:

```
src-tauri/target/release/lucioz-optimizer.exe
```

SHA256SUMS.txt contém o hash individual do instalador. O artefacto inclui relatórios nativos, fixtures, instalação e UI. O instalador foi produzido no runner GitHub; não é afirmado que exista um EXE compilado nesta pasta local sem descarregar o ZIP.

## Instalação

1. Abra o link de download, inicie sessão no GitHub e guarde o ZIP.
2. Extraia todos os ficheiros.
3. Execute o ficheiro x64-setup.exe indicado acima. Instala por utilizador.
4. Abra LUCIOZ OPTIMIZER no menu Iniciar, como utilizador normal.
5. Consulte o Dashboard e analise antes de autorizar alterações. O Centro de restauro conserva os originais das operações reversíveis.

Não precisa de abrir localhost:1420, instalar Node/Rust ou executar o servidor Vite. Requer Windows x64 e WebView2 Runtime. O instalador não tem assinatura comercial.

## Resultados aprovados

| Verificação | Resultado |
|---|---|
| TypeScript / ESLint | Passou |
| Frontend | 25 testes passaram |
| SQLite | Migrações v1/v2/v3 e integridade passaram |
| PowerShell | Sintaxe passou |
| Frontend produção | Passou |
| Rust release | 24 testes passaram |
| Tauri / NSIS | Compilação e instalador passaram |
| Binário nativo | SQLite em memória, métricas reais e inventário CIM passaram |
| Fixtures de serviços | SCM Auto → Manual → Auto passou; fixture eliminada; nenhum serviço existente alterado |
| Fixtures de tarefas | COM Enabled → Disabled → Enabled passou; tarefa nunca executada e removida |
| Instalação | Instalação por utilizador, self-test do EXE instalado e desinstalação passaram |
| WebView2 | 21 verificações passaram: IPC/métricas, 19 abas adicionais e preview de Otimizar PC; zero comandos de escrita Windows |

Os testes correram em Windows Server 2025 Datacenter no GitHub, incluindo UI com privilégios normais. Não correram no PC do utilizador e não modificaram o seu Windows. Os ensaios de preferências/arranque usam adaptadores e os de ficheiros usam fixtures temporárias; nem todas as combinações de hardware, jogos, permissões e interrupções foram exercitadas.

As primeiras tentativas de UI ficaram bloqueadas porque o runner elevado não permitia os argumentos de depuração WebView2. A execução final reduz privilégios apenas para os testes UI, seguindo a abordagem pública do Tauri. A aplicação distribuída não inclui gsudo nem drivers de automação.

Consulte STATUS.md para funcionalidades ainda não suportadas. Build aprovada não significa equivalência completa com o Hone ou garantia de ganhos de FPS.
