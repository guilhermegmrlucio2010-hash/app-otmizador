$ErrorActionPreference='Stop'
if($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_OS -ne 'Windows'){throw 'Este teste é exclusivo do runner descartável GitHub Windows'}
$command=Get-Command gsudo.exe -ErrorAction SilentlyContinue
$tool=if($command){$command.Source}else{Join-Path $env:ProgramFiles 'gsudo\Current\gsudo.exe'}
if(-not(Test-Path -LiteralPath $tool)){
 winget install --id gerardog.gsudo --exact --silent --accept-source-agreements --accept-package-agreements --disable-interactivity
 if($LASTEXITCODE -ne 0){throw 'Não foi possível instalar o utilitário gratuito de redução de privilégios do teste'}
}
if(-not(Test-Path -LiteralPath $tool)){throw 'gsudo não encontrado'}
$workspace=(Get-Location).Path
if($workspace -ne $env:GITHUB_WORKSPACE){throw 'Diretório diferente do workspace descartável'}
$sid=[Security.Principal.WindowsIdentity]::GetCurrent().User.Value
icacls $workspace /grant ('*'+$sid+':(OI)(CI)M')
if($LASTEXITCODE -ne 0){throw 'Não foi possível conceder acesso ao próprio utilizador do runner'}
& $tool --integrity Medium --wait node scripts/test-ui-native.mjs
if($LASTEXITCODE -ne 0){throw 'O teste WebView2 em privilégios normais falhou'}
& $tool --integrity Medium --wait node scripts/smoke-native.mjs src-tauri/target/release/lucioz-optimizer.exe preferences-smoke.json --isolated-preferences-test
if($LASTEXITCODE -ne 0){throw 'O ensaio de alterações/restauro sem administrador falhou'}
