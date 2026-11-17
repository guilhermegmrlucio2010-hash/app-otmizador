$steam=Get-ItemProperty 'HKCU:\Software\Valve\Steam' -ErrorAction SilentlyContinue
if($steam.SteamPath){
 $roots=@($steam.SteamPath)
 $vdf=Join-Path $steam.SteamPath 'steamapps\libraryfolders.vdf'
 if(Test-Path -LiteralPath $vdf){foreach($line in Get-Content -LiteralPath $vdf){if($line -match '"path"\s+"([^"]+)"'){$roots+=$Matches[1].Replace('\\','\')}}}
 foreach($root in ($roots | Select-Object -Unique)){Get-ChildItem -LiteralPath (Join-Path $root 'steamapps') -Filter 'appmanifest_*.acf' -ErrorAction SilentlyContinue | ForEach-Object {
  $text=Get-Content -LiteralPath $_.FullName -Raw
  if($text -match '"name"\s+"([^"]+)"'){$name=$Matches[1]}else{$name=$_.Name}
  if($text -match '"appid"\s+"(\d+)"'){[pscustomobject]@{Name=$name;Launcher='Steam';AppId=$Matches[1];Manifest=$_.FullName}}
 }}
}
$epic=Join-Path $env:ProgramData 'Epic\EpicGamesLauncher\Data\Manifests'
Get-ChildItem -LiteralPath $epic -Filter '*.item' -ErrorAction SilentlyContinue | ForEach-Object {$m=Get-Content -LiteralPath $_.FullName -Raw | ConvertFrom-Json;[pscustomobject]@{Name=$m.DisplayName;Launcher='Epic';InstallLocation=$m.InstallLocation;Executable=$m.LaunchExecutable}}
