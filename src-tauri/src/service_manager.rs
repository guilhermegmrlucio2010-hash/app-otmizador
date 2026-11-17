use serde_json::{json,Value};
use windows_service::{service::ServiceAccess,service_manager::{ServiceManager,ServiceManagerAccess}};
use windows_sys::Win32::System::Services::{EnumDependentServicesW,ENUM_SERVICE_STATUSW,SERVICE_STATE_ALL,QueryServiceConfig2W,SERVICE_CONFIG_TRIGGER_INFO,SERVICE_TRIGGER_INFO,SERVICE_ENUMERATE_DEPENDENTS};
unsafe fn wide_string(p:*const u16)->String {if p.is_null(){return String::new()}let mut length=0;while length<1024&&unsafe{*p.add(length)}!=0{length+=1;}String::from_utf16_lossy(unsafe{std::slice::from_raw_parts(p,length)})}
pub fn inspect(name:&str)->Result<Value,String>{
 if name.is_empty()||name.len()>256||name.contains('\0'){return Err("Nome de serviço inválido".into())}
 let manager=ServiceManager::local_computer(None::<&str>,ServiceManagerAccess::CONNECT).map_err(|e|e.to_string())?;
 let service=manager.open_service(name,ServiceAccess::QUERY_CONFIG|ServiceAccess::QUERY_STATUS|ServiceAccess::from_bits_retain(SERVICE_ENUMERATE_DEPENDENTS)).map_err(|e|e.to_string())?;
 let config=service.query_config().map_err(|e|e.to_string())?;let status=service.query_status().map_err(|e|e.to_string())?;
 let mut trigger_size=0u32;let mut trigger_count:Option<u32>=None;
 unsafe{QueryServiceConfig2W(service.raw_handle() as _,SERVICE_CONFIG_TRIGGER_INFO,std::ptr::null_mut(),0,&mut trigger_size);}
 if trigger_size>=std::mem::size_of::<SERVICE_TRIGGER_INFO>() as u32&&trigger_size<1_000_000{let mut buffer=vec![0u64;(trigger_size as usize+7)/8];let ok=unsafe{QueryServiceConfig2W(service.raw_handle() as _,SERVICE_CONFIG_TRIGGER_INFO,buffer.as_mut_ptr() as *mut u8,(buffer.len()*8) as u32,&mut trigger_size)};if ok!=0{trigger_count=Some(unsafe{(*(buffer.as_ptr() as *const SERVICE_TRIGGER_INFO)).cTriggers});}}
 let mut needed=0u32;let mut count=0u32;
 let first=unsafe{EnumDependentServicesW(service.raw_handle() as _,SERVICE_STATE_ALL,std::ptr::null_mut(),0,&mut needed,&mut count)};
 let mut dependents=Vec::new();let mut dependency_error=None;
 if first==0&&needed==0{dependency_error=Some(std::io::Error::last_os_error().to_string());}
 if needed>0&&needed<4_000_000{let mut buffer=vec![0u64;(needed as usize+7)/8];let ptr=buffer.as_mut_ptr() as *mut ENUM_SERVICE_STATUSW;let ok=unsafe{EnumDependentServicesW(service.raw_handle() as _,SERVICE_STATE_ALL,ptr,(buffer.len()*8) as u32,&mut needed,&mut count)};if ok!=0{for i in 0..count as usize{let entry=unsafe{&*ptr.add(i)};dependents.push(unsafe{wide_string(entry.lpServiceName)});}}else{dependency_error=Some(std::io::Error::last_os_error().to_string());}}
 Ok(json!({"name":name,"display_name":config.display_name.to_string_lossy(),"start_type":format!("{:?}",config.start_type),"state":format!("{:?}",status.current_state),"executable":config.executable_path.to_string_lossy(),"dependencies":format!("{:?}",config.dependencies),"dependents":dependents,"dependency_error":dependency_error,"recommendation":"Revisão manual. A configuração de início e as dependências não demonstram utilização nem custo de recursos.","signature":"Não disponível","trigger_count":trigger_count,"trigger_start":trigger_count.map(|n|n>0),"changes_supported":false}))
}
