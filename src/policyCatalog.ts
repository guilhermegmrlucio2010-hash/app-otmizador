import type {Check} from './types';
// These are requested subjects, not a reconstruction of the missing Hone screenshots.
const topics:[string,string,string,string][]=[
 ['interrupts.msi','Sistema','Modo MSI de dispositivos','A ativação MSI pertence à instalação INF e aos requisitos do controlador. Não existe alteração universal validada para aplicar aqui.'],
 ['interrupts.affinity','CPU','Afinidades e prioridade de interrupções','A política é gerida pelo Windows e pelo controlador. Sem medições de ISR/DPC e validação específica do dispositivo, não são modificadas afinidades.'],
 ['graphics.latency','GPU','Latência gráfica e fila de renderização','Depende do jogo, API gráfica, VSync e controlador. Não é inferida a partir de FPS; rever menus oficiais e medir com fornecedor compatível.'],
 ['nvidia.profiles','GPU','Perfis avançados NVIDIA / Reflex','Não há integração NVAPI DRS testada nesta versão. As opções de Reflex pertencem a jogos compatíveis; não são simuladas por alterações de registo.'],
 ['memory.standby','Memória','Limpeza de standby e redução forçada de RAM','Não suportada como otimização geral. Reduzir memória residente não prova melhoria e pode aumentar falhas de página.'],
 ['network.tweaks','Rede','Tweaks globais TCP/IP e DNS','Sem evidência e ensaios por ligação, não se alteram parâmetros globais. Use os testes ICMP, inventário DNS e medições comparáveis.'],
 ['security.defender','Segurança','Microsoft Defender','Proteção essencial preservada. Não existe ação para desativar o Defender.'],
 ['security.smartscreen','Segurança','Microsoft SmartScreen','Proteção de reputação preservada; não existe ação para desativar o SmartScreen.'],
 ['security.mitigations','Segurança','Mitigações de exploração','Não são reduzidas mitigações de segurança como otimização. Estado e políticas devem ser revistos nas ferramentas oficiais.'],
 ['security.vbs','Segurança','VBS e integridade da memória','Proteções essenciais preservadas; nenhuma ação de desativação é disponibilizada.'],
 ['security.browser_updates','Segurança','Atualizações de segurança dos navegadores','Tarefas e serviços de atualização não são desativados. Atualizações de segurança não são tratadas como aplicações dispensáveis.'],
 ['background.policy','Sistema','Permissões de aplicações em segundo plano','Sem uma interface universal segura para todas as aplicações Win32 e Store. Rever permissões por aplicação nas Definições; não aplica uma política global.']
];
export const policyCatalog:Check[]=topics.map(([id,category,name,reason])=>({id,category,name,description:reason,kind:'UNSUPPORTED',risk:'Moderado',source:'unsupported',field:'',recommendation:reason,benefit:'Nenhuma alteração aplicada ou ganho presumido.',confidence:'Sem implementação de escrita validada; exclusão explícita.',admin:false,restart:false,dependencies:['Revisão específica do dispositivo, aplicação ou política necessária']}));
