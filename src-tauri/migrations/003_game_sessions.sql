BEGIN IMMEDIATE;
CREATE TABLE game_sessions(
 id INTEGER PRIMARY KEY, profile_id INTEGER NOT NULL, game_name TEXT NOT NULL,
 pid INTEGER NOT NULL, mode TEXT NOT NULL, power_change INTEGER,
 status TEXT NOT NULL, started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
 ended_at TEXT, error TEXT,
 FOREIGN KEY(power_change) REFERENCES changes(id)
);
PRAGMA user_version=3;
COMMIT;
