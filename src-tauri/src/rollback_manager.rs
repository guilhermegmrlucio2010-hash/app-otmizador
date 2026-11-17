use crate::storage::Database;
use serde_json::Value;
pub fn restore(db:&Database,id:i64)->Result<Value,String>{let kind:String=db.0.lock().map_err(|e|e.to_string())?.query_row("SELECT optimization_id FROM changes WHERE id=?1",[id],|r|r.get(0)).map_err(|e|e.to_string())?;ensure_latest_target(db,id,&kind)?;match kind.as_str(){"power.scheme"=>crate::power_manager::restore(db,id),"services.manual"=>crate::service_changes::restore(db,id),"tasks.optional"=>crate::scheduled_tasks::restore(db,id),"game.fortnite.scalability"=>crate::game_optimizer::restore(db,id),"startup.user_run"=>crate::startup_manager::restore(db,id),_=>crate::optimization_engine::restore(db,id)}}

fn ensure_latest_target(db:&Database,id:i64,kind:&str)->Result<(),String>{
 let key=match kind{"services.manual"|"startup.user_run"=>"name","tasks.optional"=>"Path",_=>return Ok(())};
 let conn=db.0.lock().map_err(|e|e.to_string())?;
 let (previous,status):(String,String)=conn.query_row("SELECT previous_value,status FROM changes WHERE id=?1",[id],|r|Ok((r.get(0)?,r.get(1)?))).map_err(|e|e.to_string())?;
 if status=="restored"{return Ok(())}
 let previous:Value=serde_json::from_str(&previous).map_err(|e|e.to_string())?;
 let target=previous[key].as_str().ok_or("Backup sem identidade; restauro bloqueado")?;
 let mut query=conn.prepare("SELECT previous_value FROM changes WHERE optimization_id=?1 AND id>?2 AND status!='restored'").map_err(|e|e.to_string())?;
 let rows=query.query_map(rusqlite::params![kind,id],|r|r.get::<_,String>(0)).map_err(|e|e.to_string())?;
 for row in rows{let value:Value=serde_json::from_str(&row.map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;let newer=value[key].as_str().ok_or("Backup posterior sem identidade; restauro bloqueado")?;if newer.eq_ignore_ascii_case(target){return Err("Restaure primeiro a alteração mais recente deste serviço, tarefa ou entrada de arranque".into())}}
 Ok(())
}

#[cfg(test)]mod tests{use super::*;#[test]fn restore_order_is_scoped_to_target_and_survives_failures(){let db=Database::open(std::path::Path::new(":memory:")).unwrap();for(name,status)in [("Steam","applied"),("Discord","applied"),("steam","failed")]{db.0.lock().unwrap().execute("INSERT INTO changes(optimization_id,previous_value,new_value,status) VALUES('startup.user_run',?1,'absent',?2)",rusqlite::params![serde_json::json!({"name":name}).to_string(),status]).unwrap();}assert!(ensure_latest_target(&db,1,"startup.user_run").is_err());assert!(ensure_latest_target(&db,2,"startup.user_run").is_ok());db.0.lock().unwrap().execute("UPDATE changes SET status='restored' WHERE id=3",[]).unwrap();assert!(ensure_latest_target(&db,1,"startup.user_run").is_ok());}}
