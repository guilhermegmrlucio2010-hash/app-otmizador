import { describe, it, expect } from 'vitest';
import { frameStats, bytes, call } from './api';
import { catalog, valuesAt, evidence } from './catalog';
import { parseCsv, importPresentMon } from './fps';
describe('frametime e proveniência', () => {
    it('calcula FPS pela média temporal e nearest rank', () => { const s = frameStats([...Array<number>(198).fill(10), 20, 30]); expect(s.average_fps).toBeCloseTo(1000 / 10.15); expect(s.low_1_fps).toBe(40); expect(s.p99_ms).toBe(10); expect(s.duration_ms).toBe(2030); });
    it('recusa medições insuficientes ou inválidas', () => { for (const data of [[], Array(119).fill(10), [...Array(120).fill(10), NaN], [...Array(120).fill(10), 0], [...Array(120).fill(10), -1]])
        expect(() => frameStats(data)).toThrow(); });
    it('não mistura processos ou swapchains', () => { const rows = ['Application,ProcessID,SwapChainAddress,MsBetweenPresents', ...Array(120).fill('a.exe,1,0xaa,10'), ...Array(120).fill('a.exe,1,0xbb,20'), ...Array(120).fill('b.exe,2,0xcc,40')]; const sessions = importPresentMon(rows.join('\r\n')); expect(sessions.map(s => s.stats.average_fps)).toEqual([100, 50, 25]); });
    it('regista as amostras inválidas excluídas', () => { const text = ['Application,ProcessID,SwapChainAddress,MsBetweenPresents', ...Array(120).fill('game.exe,10,0xa,10'), 'game.exe,10,0xa,NA', 'game.exe,10,0xa,0'].join('\n'); expect(importPresentMon(text)[0].excluded).toBe(2); });
    it('recusa CSV incompatível e ficheiros incompletos', () => { expect(() => importPresentMon('FPS,Frame\n100,1')).toThrow(); expect(() => parseCsv('Name\n"incompleto')).toThrow(); expect(parseCsv('"A,B",C\r\n"A""B",D')).toEqual([['A,B', 'C'], ['A"B', 'D']]); });
});
describe('catálogo e fontes reais', () => {
    it('mantém identificadores únicos e todos os metadados obrigatórios', () => { expect(catalog.length).toBeGreaterThanOrEqual(100); expect(new Set(catalog.map(x => x.id)).size).toBe(catalog.length); for (const c of catalog) {
        expect(c.description.length).toBeGreaterThan(30);
        expect(c.recommendation).toBeTruthy();
        expect(c.confidence).toBeTruthy();
        expect(c.benefit).toBeTruthy();
        if (c.kind === 'DIAGNOSTIC')
            expect(c.action).toBeUndefined();
    } });
    it('limita as alterações às seis APIs de preferências visuais', () => { expect(catalog.filter(x => x.action).map(x => x.id)).toEqual(['visual.combobox_animation', 'visual.listbox_scrolling', 'visual.selection_fade', 'visual.client_animation', 'visual.menu_animation', 'visual.tooltip_animation']); });
    it('não oferece ações para proteções, afinidades ou tweaks experimentais',()=>{for(const id of ['security.defender','security.smartscreen','security.mitigations','security.vbs','security.browser_updates','interrupts.msi','interrupts.affinity','graphics.latency','memory.standby','network.tweaks','nvidia.profiles']){const check=catalog.find(c=>c.id===id);expect(check?.kind).toBe('UNSUPPORTED');expect(check?.action).toBeUndefined();}});
    it('preserva zero, false e ausência de fonte', () => { expect(valuesAt([{ nested: [{ value: 0 }, { value: false }] }], 'nested.value')).toEqual([0, false]); expect(evidence(catalog[0], {}, null).available).toBe(false); expect(valuesAt({ value: null }, 'value')).toEqual([]); });
    it('não devolve métricas fictícias no navegador', async () => { await expect(call('snapshot')).rejects.toThrow('aplicação Windows'); });
    it('formata unidades binárias', () => { expect(bytes(1024 ** 3)).toBe('1.0 GB'); expect(bytes(0)).toBe('0 B'); });
});
