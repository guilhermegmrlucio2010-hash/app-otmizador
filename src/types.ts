export type Row = Record<string, unknown>;
export interface Snapshot {
    timestamp: number;
    cpu_model: string;
    cpu_percent: number;
    cores: number;
    ram_total: number;
    ram_used: number;
    uptime: number;
    os: string;
    processes: number;
    disks: {
        name: string;
        total: number;
        free: number;
    }[];
    network_rx: number;
    network_tx: number;
    per_core: number[];
}
export interface Check {
    id: string;
    name: string;
    category: string;
    description: string;
    kind: 'DIAGNOSTIC' | 'SAFE CHANGE' | 'MANUAL REVIEW' | 'UNSUPPORTED';
    risk: 'Baixo' | 'Moderado';
    source: string;
    field: string;
    recommendation: string;
    benefit: string;
    confidence: string;
    admin: boolean;
    restart: boolean;
    dependencies: string[];
    action?: string;
}
export interface CheckResult {
    id: string;
    state: string;
    evidence: string;
    available: boolean;
}
export interface Change {
    category: string;
    id: number;
    optimization_id: string;
    previous_value: string;
    new_value: string;
    status: string;
    created_at: string;
    error: string | null;
}
export interface Benchmark {
    id: number;
    created_at: string;
    label: string;
    conditions: string;
    snapshot: string;
}
export interface GameProfile {
    id: number;
    name: string;
    executable: string;
    profile: string;
}
