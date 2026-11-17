use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use std::{path::{Path,PathBuf},fs::{self,OpenOptions},os::windows::{fs::{MetadataExt,OpenOptionsExt},io::AsRawHandle}};
use windows_sys::Win32::Storage::FileSystem::{SetFileInformationByHandle,FileDispositionInfo,FILE_DISPOSITION_INFO,GetDiskFreeSpaceExW};

#[derive(Clone,Serialize,Deserialize)]pub struct Candidate{pub path:String,pub size:u64,pub modified:u64}
fn supported_name(path:&Path)->bool{path.extension().and_then(|e|e.to_str()).is_some_and(|e|["tmp","temp","log","etl","dmp"].contains(&e.to_ascii_lowercase().as_str()))}
fn root()->Result<PathBuf,String>{let local=std::env::var_os("LOCALAPPDATA").ok_or("LOCALAPPDATA ausente")?;let p=PathBuf::from(local).join("Temp");if fs::symlink_metadata(&p).map_err(|e|e.to_string())?.file_attributes()&0x400!=0{return Err("A pasta temporária é um reparse point; limpeza bloqueada".into())}p.canonicalize().map_err(|e|e.to_string())}
fn safe_metadata(p:&Path)->Option<fs::Metadata>{if !supported_name(p){return None}let m=fs::symlink_metadata(p).ok()?;if !m.is_file()||m.file_attributes()&0x400!=0{return None}Some(m)}
pub fn scan()->Result<Vec<Candidate>,String>{scan_at(&root()?)}
fn scan_at(root:&Path)->Result<Vec<Candidate>,String>{let mut rows=Vec::new();for entry in fs::read_dir(root).map_err(|e|e.to_string())?.take(10000).flatten(){let p=entry.path();if let Some(m)=safe_metadata(&p){let age=m.modified().ok().and_then(|t|t.elapsed().ok()).map(|d|d.as_secs()).unwrap_or(0);if age>=7*86400{rows.push(Candidate{path:p.to_string_lossy().to_string(),size:m.len(),modified:m.last_write_time()})}}}Ok(rows)}
fn free_bytes(root:&Path)->Option<u64>{let wide:Vec<u16>=root.as_os_str().to_string_lossy().encode_utf16().chain(Some(0)).collect();let mut available=0;let ok=unsafe{GetDiskFreeSpaceExW(wide.as_ptr(),&mut available,std::ptr::null_mut(),std::ptr::null_mut())};if ok==0{None}else{Some(available)}}
pub fn delete(selected:Vec<Candidate>)->Result<Value,String>{delete_at(&root()?,selected)}
fn delete_at(root:&Path,selected:Vec<Candidate>)->Result<Value,String>{
 if selected.len()>10000{return Err("Seleção demasiado grande".into())}
 let before=free_bytes(root);let mut results=Vec::new();let mut verified_bytes=0u64;let mut seen=std::collections::HashSet::new();
 for file in selected{let p=Path::new(&file.path);let result=(||->Result<(),String>{
  if !seen.insert(file.path.to_ascii_lowercase()){return Err("Ficheiro repetido na seleção".into())}
  if p.parent()!=Some(root){return Err("Caminho fora da pasta permitida".into())}
  let m=safe_metadata(p).ok_or("Ficheiro irregular, extensão não permitida ou reparse point")?;
  if m.len()!=file.size||m.last_write_time()!=file.modified{return Err("O ficheiro foi alterado após a análise".into())}
  if m.modified().map_err(|e|e.to_string())?.elapsed().map_err(|e|e.to_string())?.as_secs()<7*86400{return Err("Ficheiro recente protegido".into())}
  let h=OpenOptions::new().access_mode(0x10000|0x80).share_mode(0).custom_flags(0x00200000).open(p).map_err(|e|format!("Em utilização ou sem permissão: {e}"))?;
  let hm=h.metadata().map_err(|e|e.to_string())?;
  if hm.file_attributes()&0x400!=0||hm.len()!=file.size||hm.last_write_time()!=file.modified{return Err("A identidade do ficheiro mudou".into())}
  let info=FILE_DISPOSITION_INFO{DeleteFile:1};let ok=unsafe{SetFileInformationByHandle(h.as_raw_handle() as _,FileDispositionInfo,&info as *const _ as _,std::mem::size_of::<FILE_DISPOSITION_INFO>() as u32)};
  if ok==0{return Err(std::io::Error::last_os_error().to_string())}drop(h);
  match fs::symlink_metadata(p){Err(e)if e.kind()==std::io::ErrorKind::NotFound=>Ok(()),_=>Err("A ausência do ficheiro não foi verificada; o caminho pode ter sido recriado".into())}
 })();if result.is_ok(){verified_bytes=verified_bytes.saturating_add(file.size)}results.push(json!({"path":file.path,"size":file.size,"deleted":result.is_ok(),"verified":result.is_ok(),"error":result.err()}));}
 let after=free_bytes(root);let delta=before.zip(after).map(|(a,b)|i128::from(b)-i128::from(a));
 Ok(json!({"files":results,"verified_deleted_bytes":verified_bytes,"volume_free_before":before,"volume_free_after":after,"volume_free_delta":delta.map(|v|v.clamp(i64::MIN as i128,i64::MAX as i128) as i64),"measurement_note":"Bytes lógicos dos ficheiros cuja ausência foi verificada. A variação livre do volume pode incluir atividade concorrente, compressão e hard links; não é atribuída exclusivamente à limpeza.","reversible":false}))
}

#[cfg(test)]mod tests{
 use super::*;
 fn old_file(root:&Path,name:&str)->PathBuf{let path=root.join(name);fs::write(&path,b"test-data").unwrap();let f=OpenOptions::new().write(true).open(&path).unwrap();let time=(std::time::SystemTime::now()-std::time::Duration::from_secs(9*86400)).duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()*10_000_000+116444736000000000;let ft=windows_sys::Win32::Foundation::FILETIME{dwLowDateTime:time as u32,dwHighDateTime:(time>>32) as u32};assert_ne!(unsafe{windows_sys::Win32::Storage::FileSystem::SetFileTime(f.as_raw_handle() as _,std::ptr::null(),std::ptr::null(),&ft)},0);path}
 #[test]fn preview_and_verified_deletion_use_only_old_technical_files(){let dir=tempfile::tempdir().unwrap();let root=dir.path().canonicalize().unwrap();let old=old_file(&root,"cache.tmp");let personal=old_file(&root,"document.txt");fs::write(root.join("fresh.tmp"),b"new").unwrap();let rows=scan_at(&root).unwrap();assert_eq!(rows.len(),1);let report=delete_at(&root,rows).unwrap();assert_eq!(report["verified_deleted_bytes"],9);assert_eq!(report["files"][0]["verified"],true);assert!(!old.exists());assert!(personal.exists());assert!(root.join("fresh.tmp").exists());}
 #[test]fn stale_locked_and_outside_files_are_not_deleted(){let dir=tempfile::tempdir().unwrap();let root=dir.path().canonicalize().unwrap();let path=old_file(&root,"locked.tmp");let rows=scan_at(&root).unwrap();let lock=OpenOptions::new().read(true).share_mode(0).open(&path).unwrap();assert_eq!(delete_at(&root,rows.clone()).unwrap()["verified_deleted_bytes"],0);drop(lock);fs::write(&path,b"changed").unwrap();assert_eq!(delete_at(&root,rows).unwrap()["verified_deleted_bytes"],0);assert!(path.exists());let other=tempfile::tempdir().unwrap();let outside=old_file(other.path(),"outside.tmp");let m=fs::metadata(&outside).unwrap();let candidate=Candidate{path:outside.to_string_lossy().to_string(),size:m.len(),modified:m.last_write_time()};assert_eq!(delete_at(&root,vec![candidate]).unwrap()["verified_deleted_bytes"],0);assert!(outside.exists());}
}
