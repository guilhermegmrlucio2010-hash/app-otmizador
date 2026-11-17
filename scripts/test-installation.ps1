$ErrorActionPreference='Stop'
if($env:GITHUB_ACTIONS -ne 'true'){throw 'Teste de instalação limitado ao runner descartável GitHub Actions'}
$installer=Get-ChildItem -LiteralPath 'src-tauri\target\release\bundle\nsis' -Filter '*-setup.exe' | Select-Object -First 1
if(-not $installer){throw 'Instalador não encontrado'}
$installDirectory=Join-Path (Get-Location).Path '.local\installed-lucioz'
if(Test-Path -LiteralPath $installDirectory){throw 'Diretório de teste já existe; não é sobrescrito'}
$process=Start-Process -FilePath $installer.FullName -ArgumentList @('/S',('/D='+$installDirectory)) -Wait -PassThru -WindowStyle Hidden
if($process.ExitCode -ne 0){throw ('Instalação falhou: '+$process.ExitCode)}
$binary=Join-Path $installDirectory 'lucioz-optimizer.exe'
if(-not (Test-Path -LiteralPath $binary)){throw 'O instalador não produziu o executável esperado'}
try {
 node scripts/smoke-native.mjs $binary installed-smoke.json
 if($LASTEXITCODE -ne 0){throw 'O executável instalado falhou o self-test'}
} finally {
 $uninstaller=Get-ChildItem -LiteralPath $installDirectory -Filter '*uninstall*.exe' | Select-Object -First 1
 if($uninstaller){$removed=Start-Process -FilePath $uninstaller.FullName -ArgumentList '/S' -Wait -PassThru -WindowStyle Hidden;if($removed.ExitCode -ne 0){throw 'O desinstalador de teste falhou'}}
}
Write-Output 'Instalação por utilizador, leitura nativa e desinstalação passaram no runner isolado.'
