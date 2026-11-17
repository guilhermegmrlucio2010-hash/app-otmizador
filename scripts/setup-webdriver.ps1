$ErrorActionPreference='Stop'
$driverDirectory=Join-Path (Get-Location).Path '.local\webdriver'
New-Item -ItemType Directory -Force -Path $driverDirectory | Out-Null
$runtimeRoots=@((Join-Path ${env:ProgramFiles(x86)} 'Microsoft\EdgeWebView\Application'),(Join-Path $env:ProgramFiles 'Microsoft\EdgeWebView\Application'))
$versions=@(foreach($root in $runtimeRoots){Get-ChildItem -LiteralPath $root -Directory -ErrorAction SilentlyContinue | Where-Object {$_.Name -match '^\d+\.\d+\.\d+\.\d+$'} | ForEach-Object {$_.Name}})
if(-not $versions.Count){throw 'WebView2 Runtime não encontrado para obter um driver compatível'}
$version=$versions | Sort-Object {[version]$_} -Descending | Select-Object -First 1
$zip=Join-Path $driverDirectory 'edgedriver.zip'
Invoke-WebRequest -Uri ('https://msedgedriver.microsoft.com/'+$version+'/edgedriver_win64.zip') -OutFile $zip
Expand-Archive -LiteralPath $zip -DestinationPath $driverDirectory -Force
$signature=Get-AuthenticodeSignature -LiteralPath (Join-Path $driverDirectory 'msedgedriver.exe')
if($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -notmatch 'Microsoft'){throw 'Assinatura do Edge WebDriver inválida'}
Write-Output ('Edge WebDriver verificado: '+$version)
