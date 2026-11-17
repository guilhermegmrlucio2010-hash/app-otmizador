import {describe,it,expect} from 'vitest';
import {modes,modeActions,compatibility,parseRecord} from './models';
import {analyzeService,classifyApplication} from './serviceAnalysis';
describe('modos de propostas',()=>{
 it('define os três modos sem ações inventadas',()=>{expect(modes.map(m=>m.id)).toEqual(['LITE','BALANCED','POWER']);for(const m of modes)for(const c of modeActions(m.id)){expect(c.action).toBe(c.id);expect(c.kind).toBe('SAFE CHANGE');expect(c.admin).toBe(false);}});
 it('mantém a seleção distinta da aplicação',()=>{const ids=modeActions('POWER').map(c=>c.id);expect(new Set(ids).size).toBe(ids.length);expect(ids.every(id=>id.startsWith('visual.'))).toBe(true);});
});
describe('comparabilidade de medições',()=>{
 const base={conditions:'Repouso, mesmo plano',duration_seconds:30,cpu_model:'CPU medida',ram_total:32_000,os:'Windows medido'};
 it('exige hardware, sistema, duração e condições iguais',()=>{expect(compatibility(base,{...base}).compatible).toBe(true);for(const key of Object.keys(base))expect(compatibility(base,{...base,[key]:'diferente'}).compatible).toBe(false);});
 it('não assume compatibilidade com informação ausente',()=>{expect(compatibility({},{}).compatible).toBe(false);expect(compatibility({...base,cpu_model:null},{...base,cpu_model:null}).compatible).toBe(false);expect(compatibility({...base,conditions:''},{...base,conditions:''}).compatible).toBe(false);expect(parseRecord('{corrompido')).toEqual({});expect(parseRecord('[]')).toEqual({});expect(parseRecord('{"cpu":0}')).toEqual({cpu:0});});
});
describe('serviços e aplicações protegidas',()=>{
 it('preserva segurança, rede e atualizações',()=>{for(const name of ['WinDefend','MpsSvc','wuauserv','Dhcp','Audiosrv','BFE','Schedule']){const r=analyzeService({Name:name,State:'Running'});expect(r.Family).toBe('Protegido');expect(r.CanChange).toBe(false);}});
 it('classifica famílias opcionais sem as declarar dispensáveis',()=>{for(const name of ['Fax','MapsBroker','lfsvc','bthserv','Spooler','XblGameSave']){const r=analyzeService({Name:name,State:'Stopped'});expect(r.Family).not.toBe('Revisão não classificada');expect(r.CanChange).toBe(false);expect(r.Activity).toContain('Parado');}});
 it('não confunde prefixos com serviços conhecidos',()=>{expect(analyzeService({Name:'FaxImportantSecurity'}).Family).toBe('Revisão não classificada');});
 it('protege frameworks, Store, WebView2 e runtimes',()=>{for(const name of ['Microsoft.WindowsStore','Microsoft.DesktopAppInstaller','Microsoft Edge WebView2 Runtime','Microsoft Visual C++ 2022','Microsoft.WindowsAppRuntime.1.6'])expect(classifyApplication({DisplayName:name}).Classification).toBe('Componente protegido / partilhado');expect(classifyApplication({DisplayName:'Qualquer',IsFramework:true}).Classification).toBe('Componente protegido / partilhado');expect(classifyApplication({DisplayName:'Outro'}).Classification).toBe('Revisão manual');});
});
