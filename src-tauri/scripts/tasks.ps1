$scheduler=New-Object -ComObject Schedule.Service
$scheduler.Connect()
function ReadFolder($folder){
 foreach($task in $folder.GetTasks(1)){
  Describe-LuciozTask $task
 }
 foreach($sub in $folder.GetFolders(0)){ReadFolder $sub}
}
ReadFolder ($scheduler.GetFolder('\'))
