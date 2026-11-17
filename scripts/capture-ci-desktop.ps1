$ErrorActionPreference='Stop'
if($env:GITHUB_ACTIONS -ne 'true'){throw 'Captura limitada ao runner descartável'}
Get-Process lucioz-optimizer,msedgewebview2 -ErrorAction SilentlyContinue | Select-Object Id,ProcessName,MainWindowTitle | Format-Table | Out-String | Write-Output
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$bounds=[Windows.Forms.SystemInformation]::VirtualScreen
$bitmap=[Drawing.Bitmap]::new($bounds.Width,$bounds.Height)
$graphics=[Drawing.Graphics]::FromImage($bitmap)
try{$graphics.CopyFromScreen($bounds.Location,[Drawing.Point]::Empty,$bounds.Size);$bitmap.Save((Join-Path (Get-Location) '.local\ci-desktop.png'))}finally{$graphics.Dispose();$bitmap.Dispose()}
