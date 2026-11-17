use serde_json::{json,Value};
use std::{fs::OpenOptions,io::Write,path::Path};
pub fn run(report:&Path)->Result<(),String>{
 let db=crate::storage::Database::open(Path::new(":memory:"))?;db.log("self_test","In-memory migration; no Windows preferences changed")?;
 let mut monitor=crate::system_info::Monitor::new();let snapshot=monitor.snapshot();
 let memory=snapshot["ram_total"].as_u64().unwrap_or(0);
 if memory==0||snapshot["cpu_model"].as_str().unwrap_or("").is_empty(){return Err("O fornecedor de métricas não devolveu hardware válido".into())}
 let hardware=crate::windows_inventory::query("hardware");
 let inventory=match hardware{Ok(value)=>json!({"available":true,"records":value.as_array().map(Vec::len).unwrap_or(0)}),Err(error)=>json!({"available":false,"error":error})};
 let result:Value=json!({"passed":true,"version":env!("CARGO_PKG_VERSION"),"read_only":true,"sqlite":"migration and log verified in memory","cpu_model":snapshot["cpu_model"],"ram_total":memory,"os":snapshot["os"],"processes":snapshot["processes"],"hardware_inventory":inventory,"limitations":"Não exercita escritas SPI, arranque, limpeza, configuração de jogos ou instalação interativa"});
 let mut file=OpenOptions::new().write(true).create_new(true).open(report).map_err(|e|e.to_string())?;
 file.write_all(serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?.as_bytes()).map_err(|e|e.to_string())?;file.sync_all().map_err(|e|e.to_string())?;Ok(())
}
