import { describe, it, expect } from 'vitest';
import { renderToString } from 'react-dom/server';
import App from './App';
describe('interface sem ponte nativa', () => {
    it('mostra o painel e indisponibilidade sem simular hardware', () => { const html = renderToString(<App />); expect(html).toContain('Conheça o seu PC.'); expect(html).toContain('Sem ligação nativa'); expect(html).toContain('Não disponível'); expect(html).not.toContain('i5-12400F'); expect(html).not.toContain('RTX 3070'); expect(html).toContain('Centro de restauro'); });
    it('inclui todos os módulos e as proteções visíveis', () => { const html = renderToString(<App />); for (const label of ['Serviços do Windows', 'Gestor de arranque', 'Gestor de processos', 'Centro de benchmarks', 'Definições', 'Sem alterações silenciosas.'])
        expect(html).toContain(label); expect(html.match(/<nav>/g)).toHaveLength(1); });
    it('apresenta os modos como propostas sem prometer ganhos',()=>{const html=renderToString(<App initialPage="boostmodes"/>);for(const label of ['LITE','BALANCED','POWER','Safe','Balanced','Gaming','OTIMIZAR PC','Não representam níveis de ganho de FPS'])expect(html).toContain(label);});
    it('distingue benchmark temporizado de amostra instantânea',()=>{const html=renderToString(<App initialPage="benchmarks"/>);expect(html).toContain('Benchmark de duração controlada');expect(html).toContain('Registar estado atual');expect(html).toContain('Medir durante');});
    it('mostra o histórico de FPS mesmo sem importação',()=>{const html=renderToString(<App initialPage="fps"/>);expect(html).toContain('Histórico FPS local');expect(html).toContain('NÃO DISPONÍVEL');expect(html).toContain('Guardar captura selecionada');});
    it('distingue integração Fortnite de perfis manuais',()=>{const html=renderToString(<App initialPage="gameoptimizer"/>);for(const label of ['Fortnite','GTA V','VALORANT','Counter-Strike 2','Analisar e pré-visualizar'])expect(html).toContain(label);expect(html).toContain('Não são tocados ficheiros de instalação');});
    it('mantém gestão de arranque restrita e sem impacto fictício',()=>{const html=renderToString(<App initialPage="startup"/>);expect(html).toContain('gestão reversível');expect(html).toContain('O impacto não é estimado sem medições.');});
});
