$scheduler=New-Object -ComObject Schedule.Service
$scheduler.Connect()
function ReadFolder($folder){
 foreach($task in $folder.GetTasks(1)){
  [pscustomobject]@{Name=$task.Name;Path=$task.Path;Publisher=$task.Definition.RegistrationInfo.Author;Enabled=$task.Enabled;State=$task.State;LastRun=$task.LastRunTime.ToString('o');NextRun=$task.NextRunTime.ToString('o');LastResult=$task.LastTaskResult;Triggers=@($task.Definition.Triggers | ForEach-Object {$_.Type});Action=@($task.Definition.Actions | ForEach-Object {$_.Path})}
 }
 foreach($sub in $folder.GetFolders(0)){ReadFolder $sub}
}
ReadFolder ($scheduler.GetFolder('\'))
