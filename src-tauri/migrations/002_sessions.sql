BEGIN IMMEDIATE;
ALTER TABLE changes ADD COLUMN category TEXT NOT NULL DEFAULT 'Efeitos visuais';
CREATE TABLE fps_sessions (
 id INTEGER PRIMARY KEY, created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
 label TEXT NOT NULL, conditions TEXT NOT NULL, stream TEXT NOT NULL,
 source TEXT NOT NULL CHECK(source='PresentMon CSV / MsBetweenPresents'),
 stats TEXT NOT NULL, frames TEXT NOT NULL
);
CREATE TABLE benchmark_runs (
 id INTEGER PRIMARY KEY, created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
 label TEXT NOT NULL, conditions TEXT NOT NULL,
 duration_seconds INTEGER NOT NULL, sample_count INTEGER NOT NULL,
 summary TEXT NOT NULL, samples TEXT NOT NULL
);
PRAGMA user_version=2;
COMMIT;
