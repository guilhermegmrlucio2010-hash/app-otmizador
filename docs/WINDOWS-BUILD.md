# Compilação Windows — 17/11/2026

Foi gerado o instalador LUCIOZ OPTIMIZER_0.2.0_x64-setup.exe e o executável portátil lucioz-optimizer.exe.

- Repositório: https://github.com/guilhermegmrlucio2010-hash/app-otmizador
- Branch: codex/windows-installer
- Commit compilado: 0a62e5988cfd0768fee4c1dd0ac7ec909befe74b
- Execução bem-sucedida: https://github.com/guilhermegmrlucio2010-hash/app-otmizador/actions/runs/44137255553
- Download: https://github.com/guilhermegmrlucio2010-hash/app-otmizador/actions/runs/44137255553/artifacts/14352702952
- Artefacto disponível até 01/12/2026; é necessário iniciar sessão no GitHub para o descarregar.
- Tamanho do ZIP: 7201935 bytes.
- SHA256 do ZIP: 834c290359d32d8acb205ee777bff5a6ed025201da3a657fa210b929f57a1ed3
- Testes TS/SSR, TypeScript, lint, SQLite, PowerShell, Rust release e teste nativo de leitura passaram.
- O self-test executou em Windows Server 2025 Datacenter no runner GitHub, não no PC do utilizador.

No ZIP, o instalador está em src-tauri/target/release/bundle/nsis/LUCIOZ OPTIMIZER_0.2.0_x64-setup.exe. O executável portátil está em src-tauri/target/release/lucioz-optimizer.exe. O ficheiro SHA256SUMS.txt contém o hash do instalador.

É uma versão de testes não assinada. A compilação e o teste de leitura não validam todas as operações de escrita nem completam as funcionalidades pendentes do STATUS.md. Os bloqueios locais de Rust/MSVC foram contornados através de compilação no GitHub Actions; o código permanece na pasta obrigatória.
