import { frameStats } from './api';
export function parseCsv(text: string): string[][] { if (text.length > 20000000)
    throw new Error('O CSV excede 20 MB.'); const rows: string[][] = []; let row: string[] = [], field = '', quoted = false; for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (c === '"') {
        if (quoted && text[i + 1] === '"') {
            field += '"';
            i++;
        }
        else
            quoted = !quoted;
    }
    else if (c === ',' && !quoted) {
        row.push(field);
        field = '';
    }
    else if ((c === '\r' || c === '\n') && !quoted) {
        if (c === '\r' && text[i + 1] === '\n')
            i++;
        row.push(field);
        if (row.some(Boolean))
            rows.push(row);
        row = [];
        field = '';
    }
    else
        field += c;
} if (quoted)
    throw new Error('CSV inválido: aspas incompletas.'); row.push(field); if (row.some(Boolean))
    rows.push(row); return rows; }
export interface FrameSession {
    key: string;
    frames: number[];
    stats: ReturnType<typeof frameStats>;
    excluded: number;
}
export function importPresentMon(text: string): FrameSession[] { const [header, ...rows] = parseCsv(text.replace(/^\uFEFF/, '')); if (!header)
    throw new Error('CSV vazio'); const index = header.findIndex(h => h.toLowerCase() === 'msbetweenpresents'); const process = header.findIndex(h => h.toLowerCase() === 'processid'); const app = header.findIndex(h => h.toLowerCase() === 'application'); const swap = header.findIndex(h => h.toLowerCase() === 'swapchainaddress'); if (index < 0 || process < 0 || swap < 0)
    throw new Error('Formato não suportado. Requer MsBetweenPresents, ProcessID e SwapChainAddress do PresentMon.'); const groups = new Map<string, {
    frames: number[];
    excluded: number;
}>(); for (const row of rows) {
    const key = `${app >= 0 ? row[app] : 'Aplicação'} · PID ${row[process]} · ${row[swap]}`;
    const group = groups.get(key) ?? { frames: [], excluded: 0 };
    const raw = row[index]?.trim();
    const n = raw ? Number(raw) : NaN;
    if (Number.isFinite(n) && n > 0)
        group.frames.push(n);
    else
        group.excluded++;
    groups.set(key, group);
} const valid = [...groups.entries()].filter(([, v]) => v.frames.length >= 120); if (!valid.length)
    throw new Error('Nenhum processo/swapchain contém 120 amostras positivas. Não são misturadas aplicações.'); return valid.map(([key, g]) => ({ key, ...g, stats: frameStats(g.frames) })); }
