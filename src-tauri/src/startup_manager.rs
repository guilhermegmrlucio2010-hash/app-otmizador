use crate::storage::Database;
use rusqlite::params;
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use winreg::{RegKey,RegValue,enums::*};
const RUN:&str="Software\\Microsoft\\Windows\\CurrentVersion\\Run";
#[derive(Serialize,Deserialize)]struct SavedEntry{name:String,bytes:Vec<u8>,kind:String}
fn optional(command:&str)->bool{
 let command=command.trim();
 let exe=if let Some(rest)=command.strip_prefix('"'){rest.split('"').next().unwrap_or("")}else{command.split_whitespace().next().unwrap_or("")};
 let path=std::path::Path::new(exe);
 if !path.is_absolute(){return false}
 let leaf=path.file_name().and_then(|x|x.to_str()).unwrap_or("").to_ascii_lowercase();
 // Removing startup registration does not terminate or execute the application.
 ["steam.exe","spotify.exe","discord.exe","epicgameslauncher.exe","chrome.exe","msedge.exe"].contains(&leaf.as_str())
}
fn decode(value:&RegValue)->Result<String,String>{if ![REG_SZ,REG_EXPAND_SZ].contains(&value.vtype)||value.bytes.len()>32768||value.bytes.len()%2!=0{return Err("Tipo de entrada não suportado".into())}let words:Vec<u16>=value.bytes.chunks_exact(2).map(|x|u16::from_le_bytes([x[0],x[1]])).collect();let text=String::from_utf16(&words).map_err(|e|e.to_string())?;Ok(text.trim_end_matches('\0').to_string())}
pub fn inventory()->Result<Value,String>{let key=match RegKey::predef(HKEY_CURRENT_USER).open_subkey(RUN){Ok(k)=>k,Err(e)if e.kind()==std::io::ErrorKind::NotFound=>return Ok(json!([])),Err(e)=>return Err(e.to_string())};let mut rows=Vec::new();for entry in key.enum_values(){let(name,value)=entry.map_err(|e|e.to_string())?;let command=decode(&value).unwrap_or_else(|_|"Tipo não suportado".into());rows.push(json!({"Name":name,"Command":command,"Location":"HKCU Run","Status":"Registada no arranque","Impact":"Não medido","CanDisable":optional(&command),"Reason":if optional(&command){"Aplicação opcional reconhecida; exige confirmação"}else{"Revisão manual; gestão integrada bloqueada"}}));}Ok(json!(rows))}
pub fn disable(db:&Database,name:&str,expected:&str)->Result<Value,String>{
 if name.is_empty()||name.len()>256||name.contains('\0'){return Err("Nome inválido".into())}
 let key=RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(RUN,KEY_READ|KEY_SET_VALUE).map_err(|e|e.to_string())?;
 let raw=key.get_raw_value(name).map_err(|e|e.to_string())?;let command=decode(&raw)?;
 if command!=expected{return Err("A entrada mudou desde a análise".into())}if !optional(&command){return Err("Apenas aplicações opcionais reconhecidas em HKCU Run podem ser geridas".into())}
 let previous=SavedEntry{name:name.into(),bytes:raw.bytes,kind:if raw.vtype==REG_SZ{"REG_SZ"}else{"REG_EXPAND_SZ"}.into()};
 let conn=db.0.lock().map_err(|e|e.to_string())?;
 conn.execute("INSERT INTO changes(optimization_id,previous_value,new_value,status,category) VALUES('startup.user_run',?1,'absent','pending','Arranque')",[serde_json::to_string(&previous).map_err(|e|e.to_string())?]).map_err(|e|e.to_string())?;let id=conn.last_insert_rowid();drop(conn);
 let result=(||{key.delete_value(name).map_err(|e|e.to_string())?;match key.get_raw_value(name){Err(e)if e.kind()==std::io::ErrorKind::NotFound=>Ok(()),_=>Err("Não foi possível verificar a desativação".to_string())}})();
 db.0.lock().map_err(|e|e.to_string())?.execute("UPDATE changes SET status=?1,error=?2 WHERE id=?3",params![if result.is_ok(){"applied"}else{"failed"},result.as_ref().err(),id]).map_err(|e|e.to_string())?;
 db.log("startup.disable",&json!({"id":id,"name":name,"success":result.is_ok()}).to_string())?;result?;Ok(json!({"id":id,"verified":true}))
}
pub fn restore(db:&Database,id:i64)->Result<Value,String>{
 let conn=db.0.lock().map_err(|e|e.to_string())?;let(previous,status):(String,String)=conn.query_row("SELECT previous_value,status FROM changes WHERE id=?1 AND optimization_id='startup.user_run'",[id],|r|Ok((r.get(0)?,r.get(1)?))).map_err(|e|e.to_string())?;drop(conn);
 if status=="restored"{return Ok(json!({"status":"restored"}))}let old:SavedEntry=serde_json::from_str(&previous).map_err(|e|e.to_string())?;
 let vtype=match old.kind.as_str(){"REG_SZ"=>REG_SZ,"REG_EXPAND_SZ"=>REG_EXPAND_SZ,_=>return Err("Backup não suportado".into())};let raw=RegValue{bytes:old.bytes,vtype};let command=decode(&raw)?;
 if !optional(&command)||old.name.is_empty()||old.name.len()>256||old.name.contains('\0'){return Err("O backup não corresponde a uma entrada opcional suportada".into())}
 let(key,_)=RegKey::predef(HKEY_CURRENT_USER).create_subkey(RUN).map_err(|e|e.to_string())?;
 match key.get_raw_value(&old.name){Ok(current)if current.bytes==raw.bytes&&current.vtype==raw.vtype=>{},Ok(_)=>return Err("Conflito: uma nova entrada ocupa este nome. Nada foi sobrescrito.".into()),Err(e)if e.kind()==std::io::ErrorKind::NotFound=>{key.set_raw_value(&old.name,&raw).map_err(|e|e.to_string())?;},Err(e)=>return Err(e.to_string())}
 let verified=key.get_raw_value(&old.name).map_err(|e|e.to_string())?;if verified.bytes!=raw.bytes||verified.vtype!=raw.vtype{return Err("Restauro não verificado".into())}
 db.0.lock().map_err(|e|e.to_string())?.execute("UPDATE changes SET status='restored',error=NULL WHERE id=?1",[id]).map_err(|e|e.to_string())?;db.log("startup.restore",&id.to_string())?;Ok(json!({"status":"restored","verified":true}))
}
#[cfg(test)]mod tests{use super::*;#[test]fn protects_system_and_scripts(){assert!(!optional("C:\\Windows\\System32\\cmd.exe /c anything"));assert!(!optional("powershell.exe"));assert!(!optional("C:\\Windows\\System32\\SecurityHealthSystray.exe"));assert!(!optional("C:\\Program Files\\Steam\\steam.exe"));assert!(optional("\"C:\\Program Files (x86)\\Steam\\steam.exe\" -silent"));}}

pub fn isolated_roundtrip()->Result<Value,String>{
 if std::env::var("GITHUB_ACTIONS").as_deref()!=Ok("true")||std::env::var("RUNNER_OS").as_deref()!=Ok("Windows"){return Err("Ensaio exclusivo do runner descartável".into())}
 let name=format!("LuciozDisposableFixture{}",std::process::id());let(key,_)=RegKey::predef(HKEY_CURRENT_USER).create_subkey(RUN).map_err(|e|e.to_string())?;
 match key.get_raw_value(&name){Err(e)if e.kind()==std::io::ErrorKind::NotFound=>{},_=>return Err("Não foi possível provar a ausência do valor de fixture".into())}
 struct Guard(String);impl Drop for Guard{fn drop(&mut self){if let Ok(k)=RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(RUN,KEY_SET_VALUE){let _=k.delete_value(&self.0);}}}
 let _guard=Guard(name.clone());let command="\"C:\\LuciozNonexistentFixture\\steam.exe\" -silent";
 let bytes:Vec<u8>=command.encode_utf16().chain(Some(0)).flat_map(u16::to_le_bytes).collect();let value=RegValue{bytes,vtype:REG_SZ};key.set_raw_value(&name,&value).map_err(|e|e.to_string())?;
 let db=Database::open(std::path::Path::new(":memory:"))?;let change=disable(&db,&name,command)?;
 if !matches!(key.get_raw_value(&name),Err(e)if e.kind()==std::io::ErrorKind::NotFound){return Err("A entrada de fixture ainda existe".into())}
 restore(&db,change["id"].as_i64().ok_or("ID de arranque ausente")?)?;let current=key.get_raw_value(&name).map_err(|e|e.to_string())?;if current.bytes!=value.bytes||current.vtype!=value.vtype{return Err("O restauro não preservou bytes/tipo".into())}
 key.delete_value(&name).map_err(|e|e.to_string())?;
 Ok(json!({"status":"passed","method":"HKCU Run fixture disable/restore; exact raw bytes and type; fixture removed","executed":false}))
}
