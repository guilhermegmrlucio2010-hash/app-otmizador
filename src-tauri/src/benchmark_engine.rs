use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use std::sync::{Arc,atomic::{AtomicBool,Ordering}};
use std::time::{Instant,Duration};
use sysinfo::{System,ProcessesToUpdate};
#[derive(Default)]pub struct BenchmarkControl{pub running:AtomicBool,pub cancel:Arc<AtomicBool>}
#[derive(Clone,Serialize,Deserialize)]pub struct FrameStats{pub frames:usize,pub duration_ms:f64,pub average_fps:f64,pub low_1_fps:f64,pub p50_ms:f64,pub p95_ms:f64,pub p99_ms:f64}
pub fn frame_stats(frames:&[f64])->Result<FrameStats,String>{
 if frames.len()<120||frames.len()>250_000||frames.iter().any(|v|!v.is_finite()||*v<=0.0||*v>3_600_000.0){return Err("A captura requer entre 120 e 250000 intervalos válidos em ms".into())}
 let mut sorted=frames.to_vec();sorted.sort_by(f64::total_cmp);let total:f64=frames.iter().sum();let slow=&sorted[(sorted.len() as f64*0.99).floor() as usize..];let percentile=|p:f64|sorted[(sorted.len() as f64*p).ceil() as usize-1];
 Ok(FrameStats{frames:frames.len(),duration_ms:total,average_fps:1000.0/(total/frames.len() as f64),low_1_fps:1000.0/(slow.iter().sum::<f64>()/slow.len() as f64),p50_ms:percentile(0.5),p95_ms:percentile(0.95),p99_ms:percentile(0.99)})
}
pub fn capture(seconds:u64,cancel:Arc<AtomicBool>)->Result<Value,String>{
 if ![15,30,60].contains(&seconds){return Err("Duração não permitida".into())}
 let mut system=System::new_all();let mut samples=Vec::new();let start=Instant::now();let mut next=Duration::from_secs(1);
 loop{
  while start.elapsed()<next{if cancel.load(Ordering::SeqCst){return Err("Benchmark cancelado; não foram guardados resultados parciais".into())}std::thread::sleep(Duration::from_millis(50));}
  if cancel.load(Ordering::SeqCst){return Err("Benchmark cancelado".into())}
  system.refresh_cpu_all();system.refresh_memory();system.refresh_processes(ProcessesToUpdate::All,true);
  samples.push(json!({"elapsed_seconds":start.elapsed().as_secs_f64(),"cpu_percent":system.global_cpu_usage(),"ram_used":system.used_memory(),"ram_available":system.available_memory(),"processes":system.processes().len()}));
  if start.elapsed().as_secs_f64()>=seconds as f64{break}next+=Duration::from_secs(1);
 }
 let mean=|field:&str|samples.iter().filter_map(|s|s[field].as_f64()).sum::<f64>()/samples.len() as f64;
 let summary=json!({"cpu_mean_percent":mean("cpu_percent"),"ram_mean_bytes":mean("ram_used"),"process_mean":mean("processes"),"elapsed_seconds":start.elapsed().as_secs_f64(),"cpu_model":system.cpus().first().map(|c|c.brand()),"ram_total":system.total_memory(),"os":System::long_os_version(),"method":"Amostragem a cada segundo; não gera carga sintética nem presume repouso"});
 Ok(json!({"summary":summary,"samples":samples,"sample_count":samples.len()}))
}
#[cfg(test)]mod tests{use super::*;#[test]fn known_frame_statistics(){let mut frames=vec![10.0;198];frames.extend([20.0,30.0]);let s=frame_stats(&frames).unwrap();assert!((s.average_fps-1000.0/10.15).abs()<0.00001);assert_eq!(s.low_1_fps,40.0);assert_eq!(s.p99_ms,10.0);}#[test]fn rejects_invalid_input(){assert!(frame_stats(&vec![0.0;120]).is_err());assert!(frame_stats(&vec![f64::NAN;120]).is_err());assert!(frame_stats(&vec![10.0;119]).is_err());assert!(capture(1,Arc::new(AtomicBool::new(false))).is_err());}}
