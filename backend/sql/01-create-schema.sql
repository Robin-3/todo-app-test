CREATE TABLE todo_status_enum (
    id_status INTEGER PRIMARY KEY AUTOINCREMENT,
    status TEXT NOT NULL UNIQUE
);

INSERT INTO todo_status_enum (id_status, status) VALUES (1, 'open');
INSERT INTO todo_status_enum (id_status, status) VALUES (2, 'close');

CREATE TABLE todo (
    id_todo INTEGER PRIMARY KEY AUTOINCREMENT,
    id_creator INTEGER NOT NULL,
    created_at DATE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%S', 'now')),
    id_modified INTEGER,
    time_modified DATE,
    title TEXT NOT NULL,
    id_status INTEGER NOT NULL REFERENCES todo_status_enum (id_status) DEFAULT (1)
);

-- UPDATE sqlite_sequence SET seq = 1000 WHERE name = 'todo';
INSERT INTO sqlite_sequence (name, seq) VALUES ('todo', 1000);
