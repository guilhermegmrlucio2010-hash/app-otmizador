$os=Get-CimInstance Win32_OperatingSystem
$cpu=Get-CimInstance Win32_Processor | Select-Object -First 1
$cs=Get-CimInstance Win32_ComputerSystem
$gpu=@(Get-CimInstance Win32_VideoController | Select-Object Name,DriverVersion,DriverDate,VideoProcessor,Status,CurrentHorizontalResolution,CurrentVerticalResolution)
$page=@(Get-CimInstance Win32_PageFileUsage | Select-Object Name,AllocatedBaseSize,CurrentUsage,PeakUsage)
$disks=@(Get-CimInstance Win32_DiskDrive | Select-Object Model,MediaType,InterfaceType,Size,Status)
$volumes=@(Get-CimInstance Win32_LogicalDisk -Filter 'DriveType=3' | Select-Object DeviceID,FileSystem,Size,FreeSpace,VolumeName)
$memory=@(Get-CimInstance Win32_PhysicalMemory | Select-Object Capacity,Speed,ConfiguredClockSpeed,Manufacturer,PartNumber)
$devices=@(Get-CimInstance Win32_PnPEntity -Filter 'ConfigManagerErrorCode <> 0' | Select-Object Name,ConfigManagerErrorCode)
function ReadValue($path,$name){try{(Get-ItemProperty -LiteralPath $path -Name $name -ErrorAction Stop).$name}catch{$null}}
[pscustomobject]@{
OS=$os.Caption;Version=$os.Version;Build=$os.BuildNumber;Architecture=$os.OSArchitecture;LastBoot=$os.LastBootUpTime.ToString('o');FreePhysicalKB=$os.FreePhysicalMemory;TotalVisibleKB=$os.TotalVisibleMemorySize;FreeVirtualKB=$os.FreeVirtualMemory;TotalVirtualKB=$os.TotalVirtualMemorySize
CPU=$cpu.Name;Cores=$cpu.NumberOfCores;Threads=$cpu.NumberOfLogicalProcessors;MaxClockMHz=$cpu.MaxClockSpeed;CurrentClockMHz=$cpu.CurrentClockSpeed;CPUStatus=$cpu.Status;Socket=$cpu.SocketDesignation;VirtualizationFirmware=$cpu.VirtualizationFirmwareEnabled;L2KB=$cpu.L2CacheSize;L3KB=$cpu.L3CacheSize
Manufacturer=$cs.Manufacturer;Model=$cs.Model;PhysicalMemory=$cs.TotalPhysicalMemory;AutoPagefile=$cs.AutomaticManagedPagefile;GPU=$gpu;Pagefile=$page;Disks=$disks;Volumes=$volumes;MemoryModules=$memory;DeviceErrors=$devices
Transparency=ReadValue 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize' 'EnableTransparency'
TransparencyEffective=$(try{[Windows.UI.ViewManagement.UISettings,Windows.UI.ViewManagement,ContentType=WindowsRuntime]::new().AdvancedEffectsEnabled}catch{$null})
Animations=ReadValue 'HKCU:\Control Panel\Desktop\WindowMetrics' 'MinAnimate'
GameMode=ReadValue 'HKCU:\Software\Microsoft\GameBar' 'AutoGameModeEnabled'
GameDVR=ReadValue 'HKCU:\Software\Microsoft\Windows\CurrentVersion\GameDVR' 'AppCaptureEnabled'
StartupDelay=ReadValue 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize' 'StartupDelayInMSec'
DarkApps=ReadValue 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize' 'AppsUseLightTheme'
TaskbarAnimations=ReadValue 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' 'TaskbarAnimations'
Hibernate=ReadValue 'HKLM:\SYSTEM\CurrentControlSet\Control\Power' 'HibernateEnabled'
LongPaths=ReadValue 'HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem' 'LongPathsEnabled'
}
