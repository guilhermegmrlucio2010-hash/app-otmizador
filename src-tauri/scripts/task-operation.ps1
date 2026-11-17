$scheduler=New-Object -ComObject Schedule.Service
$scheduler.Connect()
if(-not $Request.path.StartsWith('\') -or $Request.path.Length -gt 512){throw 'Caminho de tarefa inválido'}
$task=$scheduler.GetFolder('\').GetTask($Request.path)
$before=Describe-LuciozTask $task
if($Request.mode -eq 'inspect'){$before;return}
if(-not $before.CanDisable){throw $before.Reason}
if($before.Fingerprint -ne $Request.fingerprint -or $before.Enabled -ne [bool]$Request.expected){throw 'A definição ou o estado da tarefa mudou desde a análise'}
$task.Enabled=[bool]$Request.enabled
$after=Describe-LuciozTask ($scheduler.GetFolder('\').GetTask($Request.path))
if($after.Enabled -ne [bool]$Request.enabled -or $after.Fingerprint -ne $before.Fingerprint){throw 'Verificação da tarefa falhou'}
$after
