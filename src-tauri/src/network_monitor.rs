use serde_json::Value;
pub fn latency(address:&str)->Result<Value,String>{
 let ip:std::net::Ipv4Addr=address.parse().map_err(|_|"Introduza um endereço IPv4 válido; nomes DNS e comandos não são aceites")?;
 if ip.is_unspecified()||ip.is_multicast()||ip.is_broadcast(){return Err("Destino IPv4 não permitido".into())}
 // Only a parsed IPv4 address is interpolated. No paths, commands or scripts from the UI.
 let script=format!(r#"
$ping=New-Object System.Net.NetworkInformation.Ping
$results=@()
try {{
 for($i=0;$i -lt 10;$i++) {{
  try {{$r=$ping.Send('{ip}',1000);$results += [pscustomobject]@{{Sequence=$i+1;Status=$r.Status.ToString();Milliseconds=$(if($r.Status -eq 'Success'){{$r.RoundtripTime}}else{{$null}})}}}}
  catch {{$results += [pscustomobject]@{{Sequence=$i+1;Status='Error';Milliseconds=$null}}}}
  Start-Sleep -Milliseconds 100
 }}
}}finally{{$ping.Dispose()}}
$ok=@($results | Where-Object Status -eq 'Success')
$jitter=$null
if($ok.Count -gt 1){{$sum=0.0;for($i=1;$i -lt $ok.Count;$i++){{$sum += [math]::Abs($ok[$i].Milliseconds-$ok[$i-1].Milliseconds)}};$jitter=$sum/($ok.Count-1)}}
[pscustomobject]@{{Address='{ip}';Timestamp=(Get-Date).ToUniversalTime().ToString('o');Samples=$results;Sent=10;Received=$ok.Count;LossPercent=(10-$ok.Count)*10;AverageMilliseconds=$(if($ok.Count){{($ok | Measure-Object Milliseconds -Average).Average}}else{{$null}});JitterMilliseconds=$jitter;Method='ICMP, 10 pedidos, timeout de 1000 ms; jitter = média das diferenças absolutas entre respostas sucessivas bem-sucedidas'}}
"#);
 crate::windows_inventory::run_fixed(&script)
}
#[cfg(test)]mod tests{use super::*;#[test]fn rejects_command_injection(){assert!(latency("1.1.1.1';Start-Process calc;'").is_err());assert!(latency("example.com").is_err());assert!(latency("0.0.0.0").is_err());}}
