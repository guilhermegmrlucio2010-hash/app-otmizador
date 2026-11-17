use crate::{storage::Database,benchmark_engine::{self,BenchmarkControl}};
use serde_json::{json,Value};
use std::sync::atomic::Ordering;
use tauri::State;
use rusqlite::params;
#[tauri::command]
pub async fn record_benchmark(db:State<'_,Database>,control:State<'_,BenchmarkControl>,label:String,conditions:String,seconds:u64)->Result<Value,String>{
 if label.trim().is_empty()||label.len()>200||conditions.trim().is_empty()||conditions.len()>4000||![15,30,60].contains(&seconds){return Err("Indique nome, condições e uma duração suportada".into())}
 if control.running.compare_exchange(false,true,Ordering::SeqCst,Ordering::SeqCst).is_err(){return Err("Já existe um benchmark em execução".into())}control.cancel.store(false,Ordering::SeqCst);let cancel=control.cancel.clone();
 let result=tauri::async_runtime::spawn_blocking(move||benchmark_engine::capture(seconds,cancel)).await.map_err(|e|e.to_string()).and_then(|x|x);
 // Keep the running flag set until the durable result has been written.
 let saved=(||->Result<Value,String>{let result=result?;let conn=db.0.lock().map_err(|e|e.to_string())?;conn.execute("INSERT INTO benchmark_runs(label,conditions,duration_seconds,sample_count,summary,samples) VALUES(?1,?2,?3,?4,?5,?6)",params![label,conditions,seconds,result["sample_count"].as_u64().unwrap_or(0),result["summary"].to_string(),result["samples"].to_string()]).map_err(|e|e.to_string())?;let id=conn.last_insert_rowid();drop(conn);db.log("benchmark.record",&json!({"id":id,"seconds":seconds}).to_string())?;Ok(json!({"id":id,"result":result}))})();
 control.running.store(false,Ordering::SeqCst);if let Err(e)=&saved{let _=db.log("benchmark.error",e);}saved
}
#[tauri::command]pub fn cancel_benchmark(control:State<BenchmarkControl>)->Result<(),String>{if !control.running.load(Ordering::SeqCst){return Err("Não existe um benchmark ativo".into())}control.cancel.store(true,Ordering::SeqCst);Ok(())}
#[tauri::command]pub fn save_fps_session(db:State<Database>,label:String,conditions:String,stream:String,frames:Vec<f64>)->Result<Value,String>{
 if label.trim().is_empty()||label.len()>200||conditions.trim().is_empty()||conditions.len()>4000||stream.is_empty()||stream.len()>1000{return Err("Indique identificação, condições e fluxo da captura".into())}let stats=benchmark_engine::frame_stats(&frames)?;let stats=serde_json::to_string(&stats).map_err(|e|e.to_string())?;let conn=db.0.lock().map_err(|e|e.to_string())?;conn.execute("INSERT INTO fps_sessions(label,conditions,stream,source,stats,frames) VALUES(?1,?2,?3,'PresentMon CSV / MsBetweenPresents',?4,?5)",params![label,conditions,stream,stats,serde_json::to_string(&frames).map_err(|e|e.to_string())?]).map_err(|e|e.to_string())?;let id=conn.last_insert_rowid();drop(conn);db.log("fps.import",&id.to_string())?;Ok(json!({"id":id}))
}
#[tauri::command]pub fn get_session(db:State<Database>,kind:String,id:i64)->Result<Value,String>{let sql=match kind.as_str(){"fps"=>"SELECT frames FROM fps_sessions WHERE id=?1","benchmark"=>"SELECT samples FROM benchmark_runs WHERE id=?1",_=>return Err("Fonte de sessão não permitida".into())};let data:String=db.0.lock().map_err(|e|e.to_string())?.query_row(sql,[id],|r|r.get(0)).map_err(|e|e.to_string())?;serde_json::from_str(&data).map_err(|e|e.to_string())}
