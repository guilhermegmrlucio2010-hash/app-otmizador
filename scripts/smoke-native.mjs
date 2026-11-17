import {spawn} from 'node:child_process';
import {existsSync,readFileSync,mkdirSync} from 'node:fs';
import path from 'node:path';
const args=process.argv.slice(2);
const executable=path.resolve(args[0]??'src-tauri/target/release/lucioz-optimizer.exe');
const report=path.resolve(args[1]??'.local/native-smoke.json');
if(!existsSync(executable))throw new Error(`Executável não encontrado: ${executable}`);
if(existsSync(report))throw new Error(`O relatório já existe; indique um nome novo: ${report}`);
mkdirSync(path.dirname(report),{recursive:true});
await new Promise((resolve,reject)=>{
 const child=spawn(executable,['--self-test',report],{windowsHide:true,stdio:'pipe',timeout:90000});let errors='';
 child.stdout.on('data',data=>process.stdout.write(data));child.stderr.on('data',data=>{errors+=String(data);});
 child.on('error',reject);child.on('close',(code,signal)=>code===0?resolve():reject(new Error(`Self-test falhou: código ${code}, sinal ${signal??'nenhum'}. ${errors}`)));
});
const result=JSON.parse(readFileSync(report,'utf8'));
if(result.passed!==true||result.read_only!==true||!(result.ram_total>0))throw new Error('Relatório nativo inválido');
console.log(`Self-test nativo passou: ${result.cpu_model}; ${result.os}. Relatório: ${report}`);
