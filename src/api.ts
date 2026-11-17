import { invoke, isTauri } from '@tauri-apps/api/core';
export const native = isTauri();
export async function call<T>(command: string, args: Record<string, unknown> = {}): Promise<T> { if (!native)
    throw new Error('Disponível na aplicação Windows. Execute npm run desktop. O navegador não tem acesso ao computador.'); return invoke<T>(command, args); }
export const bytes = (n: number) => { if (!Number.isFinite(n))
    return 'Não disponível'; const units = ['B', 'KB', 'MB', 'GB', 'TB']; let i = 0; while (n >= 1024 && i < 4) {
    n /= 1024;
    i++;
} return `${n.toFixed(i ? 1 : 0)} ${units[i]}`; };
export function exportFile(name: string, data: unknown, csv = false) { const content = csv ? String(data) : JSON.stringify(data, null, 2); const url = URL.createObjectURL(new Blob([content], { type: csv ? 'text/csv;charset=utf-8' : 'application/json' })); const a = document.createElement('a'); a.href = url; a.download = name; a.click(); setTimeout(() => URL.revokeObjectURL(url), 1000); }
export function frameStats(frames: number[]) { if (frames.length < 120 || frames.some(x => !Number.isFinite(x) || x <= 0))
    throw new Error('São necessárias pelo menos 120 amostras positivas de frametime em ms.'); const sorted = [...frames].sort((a, b) => a - b); const percentile = (p: number) => sorted[Math.ceil(sorted.length * p) - 1]; const slow = sorted.slice(Math.floor(sorted.length * .99)); return { frames: frames.length, duration_ms: frames.reduce((a, b) => a + b, 0), average_fps: 1000 / (frames.reduce((a, b) => a + b, 0) / frames.length), low_1_fps: 1000 / (slow.reduce((a, b) => a + b, 0) / slow.length), p50_ms: percentile(.5), p95_ms: percentile(.95), p99_ms: percentile(.99) }; }
