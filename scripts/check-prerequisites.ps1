$ErrorActionPreference='Continue'
$missing=@()
foreach($name in @('node','npm.cmd','rustc','cargo')){if(Get-Command $name -ErrorAction SilentlyContinue){Write-Output ($name+': disponível')}else{$missing+=$name;Write-Output ($name+': EM FALTA')}}
$vswhere=Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
if(Test-Path -LiteralPath $vswhere){$vs=& $vswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath;if($vs){Write-Output 'MSVC: disponível'}else{$missing+='MSVC'}}else{$missing+='MSVC'}
if($missing.Count){Write-Output ('Instalar os pré-requisitos indicados no README: '+($missing -join ', '));exit 1}
Write-Output 'Pré-requisitos de compilação encontrados. Validar SDK Windows e WebView2 no build.'
