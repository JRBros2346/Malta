CREATE TABLE tools (
    tool_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    tool_name TEXT NOT NULL
);


CREATE TABLE tool_adj (
    tool INTEGER NOT NULL,
    adjective TEXT NOT NULL,

    FOREIGN KEY(tool) REFERENCES tools(tool_id)
);


CREATE TABLE tool_cost (
    tool INTEGER NOT NULL,
    change_date DATE NOT NULL DEFAULT CURRENT_DATE,
    new_cost INTEGER NOT NULL,

    FOREIGN KEY(tool) REFERENCES tools(tool_id),
    PRIMARY KEY(tool, change_date)
);


CREATE TABLE tool_movement (
    tool INTEGER NOT NULL,
    from_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    to_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    from_project INTEGER NULLABLE,
    to_project INTEGER NULLABLE,
    by_employee INTEGER NOT NULL,

    FOREIGN KEY(tool) REFERENCES tools(tool_id),
    FOREIGN KEY(from_project) REFERENCES projects(project_id),
    FOREIGN KEY(to_project) REFERENCES projects(project_id),
    FOREIGN KEY(by_employee) REFERENCES employees(employee_id),
    PRIMARY KEY(tool, from_time, to_time)
    CHECK (from_time < to_time)
);


CREATE TRIGGER tool_movement_check
BEFORE INSERT ON tool_movement
FOR EACH ROW
BEGIN
    SELECT 
    CASE
        WHEN EXISTS (
            SELECT 1 
            FROM tool_movement
            WHERE tool = NEW.tool 
              AND NEW.from_time < to_time 
              AND NEW.to_time > from_time
        )
        THEN RAISE(ABORT, 'Date range overlaps with an existing entry.')
    END;
END;