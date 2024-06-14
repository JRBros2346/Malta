CREATE TABLE employees (
    employee_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    employee_name TEXT NOT NULL
);


CREATE TABLE employee_salary (
    employee INTEGER NOT NULL,
    change_date DATE NOT NULL DEFAULT CURRENT_DATE,
    work_type INTEGER CHECK(work_type IN (1, 2, 4)) NOT NULL,
    new_salary INTEGER NOT NULL,

    FOREIGN KEY(employee) REFERENCES employees(employee_id),
    PRIMARY KEY(employee, change_date, work_type)
);


CREATE TABLE employee_work (
    employee INTEGER NOT NULL,
    on_date DATE NOT NULL DEFAULT CURRENT_DATE,
    on_project INTEGER NOT NULL,
    work_type INTEGER CHECK(work_type IN (1, 2, 4)) NOT NULL,

    FOREIGN KEY(employee) REFERENCES employees(employee_id),
    FOREIGN KEY(on_project) REFERENCES projects(project_id),
    PRIMARY KEY(employee, on_date, on_project)
);


CREATE TABLE employee_paid (
    employee INTEGER NOT NULL,
    on_date DATE NOT NULL DEFAULT CURRENT_DATE,
    amount INTEGER NOT NULL,

    FOREIGN KEY(employee) REFERENCES employees(employee_id),
    PRIMARY KEY(employee, on_date)
);