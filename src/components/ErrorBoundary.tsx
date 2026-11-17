import { Component, type ErrorInfo, type ReactNode } from 'react';
export class ErrorBoundary extends Component<{
    children: ReactNode;
}, {
    error: string | null;
}> {
    state: {
        error: string | null;
    } = { error: null };
    static getDerivedStateFromError(error: Error) { return { error: error.message }; }
    componentDidCatch(error: Error, info: ErrorInfo) { console.error('Falha na interface', error, info.componentStack); }
    render() { if (this.state.error)
        return <div className="card padded" style={{ margin: 40 }} role="alert"><h1>Não foi possível apresentar esta vista</h1><p>A operação pode ter terminado no backend. Consulte o histórico antes de repetir uma alteração.</p><pre>{this.state.error}</pre><button className="button primary" onClick={() => window.location.reload()}>Reabrir aplicação</button></div>; return this.props.children; }
}
