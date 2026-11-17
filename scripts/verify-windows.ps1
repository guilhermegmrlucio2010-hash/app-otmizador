param([switch]$SyntaxOnly)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$scripts=Join-Path $root 'src-tauri\scripts'
foreach($name in @('hardware','tasks','games','gpu')){
 $path=Join-Path $scripts ($name+'.ps1')
 $errors=$null
 $tokens=$null
 [void][System.Management.Automation.Language.Parser]::ParseFile($path,[ref]$tokens,[ref]$errors)
 if($errors.Count -gt 0){throw ($errors | Out-String)}
 Write-Output ($name+': sintaxe PowerShell válida')
}
if($SyntaxOnly){exit 0}
try {
 $data=@(& (Join-Path $scripts 'hardware.ps1'))
 if($data.Count -ne 1 -or -not $data[0].CPU -or -not $data[0].Build){throw 'Inventário incompleto'}
 Write-Output 'hardware: inventário real obtido; CPU e compilação presentes'
} catch {Write-Error ('hardware: '+$_.Exception.Message);exit 1}
