$binary=Join-Path $env:SystemRoot 'System32\nvidia-smi.exe'
if(-not (Test-Path -LiteralPath $binary -PathType Leaf)){
 [pscustomobject]@{Available=$false;Provider='NVIDIA SMI';Reason='Fornecedor não instalado no caminho suportado';GPUs=@()}
 return
}
$signature=Get-AuthenticodeSignature -LiteralPath $binary
if($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -notmatch 'NVIDIA'){
 throw 'O fornecedor NVIDIA não tem uma assinatura válida reconhecida. Não foi executado.'
}
function NumberOrNull($value){
 $number=0.0
 if([double]::TryParse($value.Trim(),[System.Globalization.NumberStyles]::Float,[System.Globalization.CultureInfo]::InvariantCulture,[ref]$number)){return $number}
 return $null
}
$csv=& $binary '--query-gpu=name,utilization.gpu,memory.used,memory.total,temperature.gpu,driver_version' '--format=csv,noheader,nounits'
if($LASTEXITCODE -ne 0){throw 'O fornecedor NVIDIA recusou a consulta'}
$gpus=@($csv | ConvertFrom-Csv -Header Name,Utilization,MemoryUsedMiB,MemoryTotalMiB,Temperature,Driver | ForEach-Object {
 [pscustomobject]@{Name=$_.Name.Trim();Utilization=NumberOrNull $_.Utilization;MemoryUsedMiB=NumberOrNull $_.MemoryUsedMiB;MemoryTotalMiB=NumberOrNull $_.MemoryTotalMiB;TemperatureC=NumberOrNull $_.Temperature;Driver=$_.Driver.Trim()}
})
[pscustomobject]@{Available=($gpus.Count -gt 0);Provider='NVIDIA SMI / assinatura validada';Timestamp=(Get-Date).ToUniversalTime().ToString('o');GPUs=$gpus}
