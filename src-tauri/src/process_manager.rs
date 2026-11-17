use std::{path::Path,os::windows::ffi::OsStringExt};
use windows_sys::Win32::{Foundation::{CloseHandle,HANDLE,FILETIME},System::Threading::{OpenProcess,GetProcessTimes,QueryFullProcessImageNameW,IsProcessCritical,TerminateProcess,WaitForSingleObject,PROCESS_QUERY_LIMITED_INFORMATION,PROCESS_TERMINATE}};
struct ProcessHandle(HANDLE);
impl Drop for ProcessHandle{fn drop(&mut self){unsafe{CloseHandle(self.0);}}}
fn allowed_image(path:&Path)->bool{
 let leaf=path.file_name().and_then(|x|x.to_str()).unwrap_or("").to_ascii_lowercase();
 path.is_absolute()&&["chrome.exe","firefox.exe","msedge.exe","discord.exe","spotify.exe","steam.exe","epicgameslauncher.exe","notepad.exe"].contains(&leaf.as_str())
}
fn unix_seconds(time:FILETIME)->Option<u64>{let ticks=((time.dwHighDateTime as u64)<<32)|time.dwLowDateTime as u64;ticks.checked_sub(116444736000000000).map(|x|x/10_000_000)}
pub fn isolated_roundtrip()->Result<serde_json::Value,String>{
 if std::env::var("GITHUB_ACTIONS").as_deref()!=Ok("true")||std::env::var("RUNNER_OS").as_deref()!=Ok("Windows"){return Err("Ensaio exclusivo do runner descartável".into())}
 let dir=std::env::temp_dir().join(format!("lucioz-process-fixture-{}",std::process::id()));std::fs::create_dir(&dir).map_err(|e|e.to_string())?;
 struct Files(std::path::PathBuf);impl Drop for Files{fn drop(&mut self){let _=std::fs::remove_file(self.0.join("notepad.exe"));let _=std::fs::remove_dir(&self.0);}}
 let _files=Files(dir.clone());let path=dir.join("notepad.exe");std::fs::copy(std::env::current_exe().map_err(|e|e.to_string())?,&path).map_err(|e|e.to_string())?;
 struct Child(std::process::Child);impl Drop for Child{fn drop(&mut self){let _=self.0.kill();let _=self.0.wait();}}
 let child=std::process::Command::new(&path).arg("--process-fixture-wait").spawn().map_err(|e|e.to_string())?;let mut child=Child(child);
 let raw=unsafe{OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION,0,child.0.id())};if raw.is_null(){return Err(std::io::Error::last_os_error().to_string())}let handle=ProcessHandle(raw);let mut created=FILETIME{dwLowDateTime:0,dwHighDateTime:0};let mut exited=created;let mut kernel=created;let mut user=created;
 if unsafe{GetProcessTimes(handle.0,&mut created,&mut exited,&mut kernel,&mut user)}==0{return Err(std::io::Error::last_os_error().to_string())}let start=unix_seconds(created).ok_or("Hora da fixture inválida")?;
 if terminate(child.0.id(),start+1).is_ok(){return Err("Uma identidade incorreta foi aceite".into())}terminate(child.0.id(),start)?;child.0.wait().map_err(|e|e.to_string())?;
 Ok(serde_json::json!({"status":"passed","method":"Owned disposable child; stale identity rejected; TerminateProcess and signalled handle verified","reversible":false}))
}
pub fn terminate(pid:u32,start:u64)->Result<(),String>{
 if pid<=4||pid==std::process::id()||start==0{return Err("Processo protegido ou identidade incompleta".into())}
 // Open once: all identity checks and termination use this same kernel object.
 // SYNCHRONIZE is required to verify the process handle becomes signalled.
 let raw=unsafe{OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION|PROCESS_TERMINATE|0x00100000,0,pid)};
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
 if unsafe{TerminateProcess(handle.0,1)}==0{return Err(format!("O Windows recusou terminar o processo: {}",std::io::Error::last_os_error()))}if unsafe{WaitForSingleObject(handle.0,5000)}!=0{return Err("O pedido foi enviado, mas a saída do processo não foi verificada em cinco segundos".into())}Ok(())
}
#[cfg(test)]mod tests{use super::*;#[test]fn protected_images_are_not_allowed(){for name in ["C:\\Windows\\System32\\lsass.exe","C:\\Windows\\System32\\svchost.exe","chrome.exe","C:\\Tools\\chrome.exe.bat"]{assert!(!allowed_image(Path::new(name)));}assert!(allowed_image(Path::new("C:\\Program Files\\Browser\\chrome.exe")));}#[test]fn filetime_conversion_is_checked(){assert_eq!(unix_seconds(FILETIME{dwLowDateTime:0,dwHighDateTime:0}),None);let ticks=116444736000000000u64+42*10_000_000;assert_eq!(unix_seconds(FILETIME{dwLowDateTime:ticks as u32,dwHighDateTime:(ticks>>32) as u32}),Some(42));}}

