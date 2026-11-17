function Describe-LuciozTask($task){
 $definition=$task.Definition
 $currentSid=[System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value
 $owner=$definition.Principal.UserId
 $ownerSid=$null
 try {
  if($owner -match '^S-1-'){$ownerSid=$owner}else{$ownerSid=([System.Security.Principal.NTAccount]::new($owner)).Translate([System.Security.Principal.SecurityIdentifier]).Value}
 }catch{}
 $actions=@($definition.Actions)
 $optional=$false
 if($actions.Count -eq 1 -and $actions[0].Type -eq 0){
  $exe=$actions[0].Path
  if([System.IO.Path]::IsPathRooted($exe)){
   $optional=@('steam.exe','spotify.exe','discord.exe','epicgameslauncher.exe') -contains [System.IO.Path]::GetFileName($exe).ToLowerInvariant()
  }
 }
 $protected=$task.Path.StartsWith('\Microsoft\',[StringComparison]::OrdinalIgnoreCase) -or $task.Path -match '(?i)update|security|defender|backup|recovery|maintenance|driver'
 $allowed=($ownerSid -eq $currentSid) -and ($definition.Principal.RunLevel -eq 0) -and $optional -and -not $protected
 [xml]$document=$task.Xml
 $namespace=[System.Xml.XmlNamespaceManager]::new($document.NameTable)
 $namespace.AddNamespace('t',$document.DocumentElement.NamespaceURI)
 $enabled=$document.SelectSingleNode('/t:Task/t:Settings/t:Enabled',$namespace)
 if($enabled){[void]$enabled.ParentNode.RemoveChild($enabled)}
 [pscustomobject]@{Name=$task.Name;Path=$task.Path;Publisher=$definition.RegistrationInfo.Author;Enabled=$task.Enabled;State=$task.State;Owner=$ownerSid;RunLevel=$definition.Principal.RunLevel;LastRun=$task.LastRunTime.ToString('o');NextRun=$task.NextRunTime.ToString('o');LastResult=$task.LastTaskResult;Triggers=@($definition.Triggers | ForEach-Object {$_.Type});Action=@($actions | ForEach-Object {$_.Path});CanDisable=$allowed;Reason=$(if($allowed){'Aplicação opcional do utilizador, execução limitada e fora das tarefas protegidas'}else{'Não suportado: tarefa protegida, identidade/privilégios ou ação fora da allowlist'});Fingerprint=$(if($allowed){$document.OuterXml}else{$null})}
}
