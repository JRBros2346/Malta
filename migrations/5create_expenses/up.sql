CREATE TABLE expenses (
    on_time DATETIME NOT NULL PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
    project INTEGER NULLABLE,
    reason TEXT NOT NULL,
    amount INTEGER NOT NULL,

    FOREIGN KEY(project) REFERENCES projects(project_id)
);