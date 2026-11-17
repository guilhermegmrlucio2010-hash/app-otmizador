import type {Row} from '../types';
const families:[RegExp,string,string][]=[
 [/^fax$/i,'Fax','Confirmar utilização de fax e software associado.'],
 [/^mapsbroker$/i,'Mapas offline','Confirmar utilização de mapas offline e triggers antes de alterar.'],
 [/^retaildemo$/i,'Demonstração comercial','Confirmar que o computador não está em modo de demonstração.'],
 [/^lfsvc$/i,'Geolocalização','Aplicações e permissões de localização podem depender deste serviço.'],
 [/^(sensordataservice|sensorservice|sensrsvc)$/i,'Sensores','Pode ser necessário para rotação, luminosidade e outros sensores.'],
 [/^(bthserv|btagservice|bluetoothuserservice(?:_.*)?)$/i,'Bluetooth','Confirmar periféricos e funcionalidades de áudio/dispositivos.'],
 [/^(spooler|printnotify|printworkflowusersvc(?:_.*)?)$/i,'Impressão','Inclui impressão física e virtual; não presumir que está dispensado.'],
 [/^(xblAuthManager|xblGameSave|xboxNetApiSvc|xboxGipSvc)$/i,'Xbox','Pode ser necessário para autenticação, jogos, saves ou comandos.']
];
const critical=/^(windefend|mpssvc|wuauserv|bits|cryptsvc|rpcss|dcomlaunch|eventlog|samss|lsm|profsvc|dhcp|dnscache|nlasvc|netman|audiosrv|audioendpointbuilder|plugplay|power|schedule|winmgmt|bfe|securityhealthservice|trustedinstaller|systemeventsbroker|storsvc)$/i;
export function analyzeService(row:Row):Row{
 const name=String(row.Name??'');if(critical.test(name))return {...row,Family:'Protegido',Review:'Preservar serviço essencial ou de segurança.',CanChange:false};
 const family=families.find(([match])=>match.test(name));return {...row,Family:family?.[1]??'Revisão não classificada',Review:family?.[2]??'Nome/editor não demonstram que seja opcional. Rever dependências e utilização.',Activity:row.State==='Stopped'?'Parado nesta amostra; não há custo de CPU atribuível demonstrado.':'O estado em execução não mede o custo de recursos.',CanChange:false};
}
export function classifyApplication(row:Row):Row{const name=String(row.DisplayName??'');const protectedName=/WindowsStore|DesktopAppInstaller|WebView2|Visual C\+\+|\.NET|VCLibs|UI\.Xaml|WindowsAppRuntime|Microsoft Edge|DirectX|Windows SDK|Microsoft Defender/i.test(name);const protectedFlag=row.NonRemovable===true||row.IsFramework===true||Number(row.SystemComponent)===1;return {...row,Classification:protectedName||protectedFlag?'Componente protegido / partilhado':'Revisão manual',Removal:'Apenas pela interface oficial; sem remoção automática'};}
