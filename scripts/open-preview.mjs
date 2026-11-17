import {spawn} from 'node:child_process';
import {existsSync,mkdirSync,openSync,closeSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import path from 'node:path';
const root=fileURLToPath(new URL('../',import.meta.url));
const url='http://127.0.0.1:1420/';
async function ready(){try{const response=await fetch(url,{signal:AbortSignal.timeout(700)});return response.ok&&(await response.text()).includes('LUCIOZ');}catch{return false;}}
try{
 if(!await ready()){
  if(!existsSync(path.join(root,'dist/index.html')))throw new Error('Falta o build. Execute npm run build na pasta do projeto.');
  if(!existsSync(path.join(root,'node_modules/vite/bin/vite.js')))throw new Error('Faltam dependências. Execute npm ci.');
  mkdirSync(path.join(root,'.local'),{recursive:true});const log=openSync(path.join(root,'.local/preview-launch.log'),'a');
  const child=spawn(process.execPath,[path.join(root,'node_modules/vite/bin/vite.js'),'preview','--host','127.0.0.1','--port','1420','--strictPort','--configLoader','native'],{cwd:root,detached:true,windowsHide:true,stdio:['ignore',log,log]});
  await new Promise((resolve,reject)=>{child.once('spawn',resolve);child.once('error',reject);});child.unref();closeSync(log);
  let started=false;for(let n=0;n<20;n++){if(await ready()){started=true;break;}await new Promise(resolve=>setTimeout(resolve,250));}
  if(!started)throw new Error('O servidor não respondeu. Consulte .local/preview-launch.log; a porta 1420 pode estar ocupada.');
 }
 console.log(`LUCIOZ disponível em ${url}\nEsta pré-visualização não tem acesso às APIs Windows. Para o desktop: npm run desktop.`);
 const browser=spawn('explorer.exe',[url],{detached:true,windowsHide:true,stdio:'ignore'});browser.on('error',()=>console.log('Abra o endereço acima no seu navegador.'));browser.unref();
}catch(error){console.error(error.message);process.exitCode=1;}
