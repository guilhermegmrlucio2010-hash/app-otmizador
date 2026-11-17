use std::{path::Path,os::windows::ffi::OsStringExt};
use windows_sys::Win32::{Foundation::{CloseHandle,HANDLE,FILETIME},System::Threading::{OpenProcess,GetProcessTimes,QueryFullProcessImageNameW,IsProcessCritical,TerminateProcess,PROCESS_QUERY_LIMITED_INFORMATION,PROCESS_TERMINATE}};
struct ProcessHandle(HANDLE);
impl Drop for ProcessHandle{fn drop(&mut self){unsafe{CloseHandle(self.0);}}}
fn allowed_image(path:&Path)->bool{
 let leaf=path.file_name().and_then(|x|x.to_str()).unwrap_or("").to_ascii_lowercase();
 path.is_absolute()&&["chrome.exe","firefox.exe","msedge.exe","discord.exe","spotify.exe","steam.exe","epicgameslauncher.exe","notepad.exe"].contains(&leaf.as_str())
}
fn unix_seconds(time:FILETIME)->Option<u64>{let ticks=((time.dwHighDateTime as u64)<<32)|time.dwLowDateTime as u64;ticks.checked_sub(116444736000000000).map(|x|x/10_000_000)}
pub fn terminate(pid:u32,start:u64)->Result<(),String>{
 if pid<=4||pid==std::process::id()||start==0{return Err("Processo protegido ou identidade incompleta".into())}
 // Open once: all identity checks and termination use this same kernel object.
 let raw=unsafe{OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION|PROCESS_TERMINATE,0,pid)};
 if raw.is_null(){return Err(format!("O Windows recusou abrir o processo: {}",std::io::Error::last_os_error()))}
 let handle=ProcessHandle(raw);
 let mut critical=0i32;if unsafe{IsProcessCritical(handle.0,&mut critical)}==0{return Err(format!("Não foi possível verificar se o processo é crítico: {}",std::io::Error::last_os_error()))}if critical!=0{return Err("Processo crítico protegido pelo Windows".into())}
 let mut created=FILETIME{dwLowDateTime:0,dwHighDateTime:0};let mut exit=created;let mut kernel=created;let mut user=created;
 if unsafe{GetProcessTimes(handle.0,&mut created,&mut exit,&mut kernel,&mut user)}==0{return Err(std::io::Error::last_os_error().to_string())}
 if unix_seconds(created)!=Some(start){return Err("A identidade do processo mudou; o pedido foi cancelado".into())}
 let mut image=vec![0u16;32768];let mut length=image.len() as u32;
 if unsafe{QueryFullProcessImageNameW(handle.0,0,image.as_mut_ptr(),&mut length)}==0{return Err(std::io::Error::last_os_error().to_string())}
 let image=std::ffi::OsString::from_wide(&image[..length as usize]);
 if !allowed_image(Path::new(&image)){return Err("Terminação bloqueada: apenas aplicações opcionais reconhecidas podem ser terminadas".into())}
 if unsafe{TerminateProcess(handle.0,1)}==0{return Err(format!("O Windows recusou terminar o processo: {}",std::io::Error::last_os_error()))}Ok(())
}
#[cfg(test)]mod tests{use super::*;#[test]fn protected_images_are_not_allowed(){for name in ["C:\\Windows\\System32\\lsass.exe","C:\\Windows\\System32\\svchost.exe","chrome.exe","C:\\Tools\\chrome.exe.bat"]{assert!(!allowed_image(Path::new(name)));}assert!(allowed_image(Path::new("C:\\Program Files\\Browser\\chrome.exe")));}#[test]fn filetime_conversion_is_checked(){assert_eq!(unix_seconds(FILETIME{dwLowDateTime:0,dwHighDateTime:0}),None);let ticks=116444736000000000u64+42*10_000_000;assert_eq!(unix_seconds(FILETIME{dwLowDateTime:ticks as u32,dwHighDateTime:(ticks>>32) as u32}),Some(42));}}

