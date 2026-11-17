import * as Dialog from '@radix-ui/react-dialog';
import { Activity, ArrowDownToLine, Check, ChevronRight, ShieldCheck, X, Search, Info } from 'lucide-react';
import { AreaChart, Area, ResponsiveContainer, Tooltip, XAxis, YAxis, CartesianGrid } from 'recharts';
import { useMemo, useState, type ReactNode } from 'react';
import { Button } from './ui/button';
import type { Check as CheckType, Row } from '../types';
export { Button };
export function RiskIndicator({ risk = 'Baixo' }: {
    risk?: string;
}) { return <span className={`badge ${risk === 'Baixo' ? 'green' : 'amber'}`}><ShieldCheck size={12}/>{risk}</span>; }
export function MetricCard({ label, value, detail, icon, percent }: {
    label: string;
    value: string;
    detail: string;
    icon: ReactNode;
    percent?: number;
}) { return <div className="card metric"><div className="metric-label">{label}<span className="metric-icon">{icon}</span></div><strong>{value}</strong><p title={detail}>{detail}</p><div className="meter"><i style={{ width: `${Math.max(0, Math.min(100, percent ?? 0))}%` }}/></div></div>; }
export function Empty({ title = 'Sem dados disponíveis', children }: {
    title?: string;
    children?: ReactNode;
}) { return <div className="empty"><Activity size={30}/><h3>{title}</h3><p>{children ?? 'Execute uma análise na aplicação Windows para consultar dados reais.'}</p></div>; }
export function Notice({ children }: {
    children: ReactNode;
}) { return <div className="notice"><Info size={17}/><div>{children}</div></div>; }
export function PerformanceChart({ data, field = 'cpu', color = '#4285ff', unit = '%', height = 230 }: {
    data: Row[];
    field?: string;
    color?: string;
    unit?: string;
    height?: number;
}) { if (data.length < 2)
    return <Empty title="A aguardar amostras">O gráfico aparece após duas leituras reais do sistema.</Empty>; return <div style={{ width: '100%', height }}><ResponsiveContainer><AreaChart data={data}><defs><linearGradient id={`fill-${field}`} x1="0" y1="0" x2="0" y2="1"><stop offset="0%" stopColor={color} stopOpacity={.3}/><stop offset="100%" stopColor={color} stopOpacity={0}/></linearGradient></defs><CartesianGrid stroke="#202937" vertical={false}/><XAxis dataKey="time" tick={{ fill: '#788799', fontSize: 10 }} axisLine={false} tickLine={false} minTickGap={40}/><YAxis tick={{ fill: '#788799', fontSize: 10 }} axisLine={false} tickLine={false} width={35} unit={unit}/><Tooltip contentStyle={{ background: '#121a27', border: '1px solid #2c3b50', borderRadius: 8 }}/><Area type="monotone" dataKey={field} stroke={color} fill={`url(#fill-${field})`} strokeWidth={2} isAnimationActive={false}/></AreaChart></ResponsiveContainer></div>; }
export function ConfirmationDialog({ open, title, children, onCancel, onConfirm, busy = false, confirmLabel='Confirmar e executar', description='Leia as consequências antes de confirmar a operação.' }: {
    open: boolean;
    title: string;
    children: ReactNode;
    onCancel: () => void;
    onConfirm: () => void;
    busy?: boolean;
    confirmLabel?: string;
    description?: string;
}) { return <Dialog.Root open={open} onOpenChange={v => { if (!v && !busy)
    onCancel(); }}><Dialog.Portal><Dialog.Overlay className="dialog-overlay"/><Dialog.Content className="dialog-content" onEscapeKeyDown={e => { if (busy)
    e.preventDefault(); }} onInteractOutside={e => { if (busy)
    e.preventDefault(); }}><Dialog.Title>{title}</Dialog.Title><Dialog.Description>{description}</Dialog.Description><div className="dialog-body">{children}</div><div className="actions"><Button variant="secondary" disabled={busy} onClick={onCancel}>Cancelar</Button><Button disabled={busy} onClick={onConfirm}>{busy ? 'A executar…' : confirmLabel}</Button></div></Dialog.Content></Dialog.Portal></Dialog.Root>; }
function cell(value: unknown) { return value === undefined || value === null ? 'Não disponível' : typeof value === 'object' ? JSON.stringify(value) : String(value); }
export function DataTable({ rows, columns, onRow, action, filter = '' }: {
    rows: Row[];
    columns?: string[];
    onRow?: (row: Row) => void;
    action?: string;
    filter?: string;
}) { const [sort, setSort] = useState<{
    key: string;
    asc: boolean;
} | null>(null); const [limit, setLimit] = useState(100); const cols = columns ?? Object.keys(rows[0] ?? {}); const filtered = useMemo(() => { const result = rows.filter(r => JSON.stringify(r).toLocaleLowerCase().includes(filter.toLocaleLowerCase())); if (sort)
    result.sort((a, b) => { const x = a[sort.key], y = b[sort.key]; return (typeof x === 'number' && typeof y === 'number' ? x - y : cell(x).localeCompare(cell(y))) * (sort.asc ? 1 : -1); }); return result; }, [rows, filter, sort]); if (!rows.length)
    return <Empty />; return <><div className="table-wrap"><table><thead><tr>{cols.map(c => <th key={c}><button onClick={() => setSort({ key: c, asc: sort?.key === c ? !sort.asc : true })}>{c} {sort?.key === c ? (sort.asc ? '↑' : '↓') : ''}</button></th>)}{onRow && <th>Operação</th>}</tr></thead><tbody>{filtered.slice(0, limit).map((r, i) => <tr key={i}>{cols.map(c => <td title={cell(r[c])} key={c}>{cell(r[c])}</td>)}{onRow && <td><Button variant="ghost" onClick={() => onRow(r)}>{action ?? 'Inspecionar'}<ChevronRight size={13}/></Button></td>}</tr>)}</tbody></table></div><div className="table-footer">{Math.min(limit, filtered.length)} de {filtered.length} registos{limit < filtered.length && <Button variant="ghost" onClick={() => setLimit(x => x + 100)}>Mostrar mais</Button>}</div></>; }
export const ServiceTable = DataTable;
export const ProcessTable = DataTable;
export function OptimizationToggle({ checked, onChange, label, disabled=false }: {
    checked: boolean;
    onChange: (value: boolean) => void;
    label: string;
    disabled?: boolean;
}) { return <label className="check-label"><input type="checkbox" checked={checked} disabled={disabled} onChange={e => onChange(e.target.checked)}/>{label}</label>; }
export function OptimizationCard({ check, state, onReview, selected, onSelect }: {
    check: CheckType;
    state: {
        available: boolean;
        text: string;
    };
    onReview: () => void;
    selected: boolean;
    onSelect: () => void;
}) { return <article className="card optimization"><div className="optimization-top"><span className="category">{check.category}</span><span className={`badge ${check.kind === 'SAFE CHANGE' ? 'blue' : ''}`}>{check.kind === 'DIAGNOSTIC' ? 'DIAGNÓSTICO' : check.kind === 'SAFE CHANGE' ? 'ALTERAÇÃO REVERSÍVEL' : check.kind === 'MANUAL REVIEW' ? 'REVISÃO MANUAL' : 'NÃO SUPORTADO'}</span></div><h3>{check.name}</h3><p>{check.description}</p><div className="detected"><span className={`dot ${state.available ? 'online' : ''}`}/><span>{state.available ? 'Detetado' : 'Não disponível'}</span><code title={state.text}>{state.text}</code></div><div className="optimization-bottom"><RiskIndicator risk={check.risk}/>{check.action && <OptimizationToggle label="Selecionar proposta" checked={selected} disabled={!state.available} onChange={onSelect}/>}<Button variant="ghost" onClick={onReview}>Detalhes <ChevronRight size={13}/></Button></div></article>; }
export function Toolbar({ search, onSearch, children }: {
    search: string;
    onSearch: (s: string) => void;
    children?: ReactNode;
}) { return <div className="toolbar"><label className="search"><Search size={16}/><input placeholder="Pesquisar nesta vista…" value={search} onChange={e => onSearch(e.target.value)}/>{search && <button aria-label="Limpar pesquisa" onClick={() => onSearch('')}><X size={14}/></button>}</label>{children}</div>; }
export function SectionTitle({ title, subtitle, children }: {
    title: string;
    subtitle?: string;
    children?: ReactNode;
}) { return <div className="section-title"><div><h2>{title}</h2>{subtitle && <p>{subtitle}</p>}</div><div className="actions">{children}</div></div>; }
export function DownloadButton({ onClick }: {
    onClick: () => void;
}) { return <Button variant="secondary" onClick={onClick}><ArrowDownToLine size={15}/>Exportar JSON</Button>; }
export function CheckList({ items }: {
    items: string[];
}) { return <ul className="checklist">{items.map(i => <li key={i}><Check size={14}/>{i}</li>)}</ul>; }
