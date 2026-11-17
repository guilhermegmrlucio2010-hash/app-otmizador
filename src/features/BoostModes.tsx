import {useState} from 'react';
import {ShieldCheck,Zap,SlidersHorizontal} from 'lucide-react';
import {Button,Notice,CheckList} from '../components';
import {modes,modeActions,type BoostMode} from './models';
import {OptimizationWorkflow} from './OptimizationWorkflow';
export function BoostModes({busy,notify,onChanged}:{busy:boolean;notify:(text:string,error?:boolean)=>void;onChanged:()=>Promise<void>}){
 const[selected,setSelected]=useState<BoostMode>('LITE');
 const profile=selected==='LITE'?'Safe':selected==='BALANCED'?'Balanced':'Gaming';
 return <><Notice>Safe, Balanced e Gaming selecionam propostas suportadas. LITE e POWER são os nomes anteriores conservados como aliases. Não representam níveis de ganho de FPS; mudar de perfil não executa alterações.</Notice><div className="mode-grid">{modes.map((m,i)=>{const Icon=[ShieldCheck,SlidersHorizontal,Zap][i];const name=m.id==='LITE'?'Safe':m.id==='BALANCED'?'Balanced':'Gaming';return <article className={`card mode-card ${selected===m.id?'mode-selected':''}`} key={m.id}><div className="icon-box"><Icon/></div><div className="section-title"><h2>{name}</h2><span className="badge">{m.id}</span></div><p>{m.description}</p><CheckList items={modeActions(m.id).map(c=>c.name)}/>{m.id!=='LITE'&&<p>Analisa também o plano de energia associado, se estiver instalado. Requer seleção individual.</p>}<Button disabled={busy} variant={selected===m.id?'default':'secondary'} onClick={()=>setSelected(m.id)}>{selected===m.id?'Modo selecionado':'Selecionar modo'}</Button></article>;})}</div><OptimizationWorkflow key={selected} initialMode={profile} notify={notify} onChanged={onChanged}/></>;
}
