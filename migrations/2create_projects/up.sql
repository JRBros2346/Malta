CREATE TABLE projects (
    project_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    parent_company TEXT NOT NULL,
    project_name TEXT NOT NULL,
    estimate INTEGER NULLABLE,

    FOREIGN KEY(parent_company) REFERENCES companies(company)
);