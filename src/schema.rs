// @generated automatically by Diesel CLI.

diesel::table! {
    companies (company) {
        company -> Text,
    }
}

diesel::table! {
    employee_paid (employee, on_date) {
        employee -> Integer,
        on_date -> Date,
        amount -> Integer,
    }
}

diesel::table! {
    employee_salary (employee, change_date, work_type) {
        employee -> Integer,
        change_date -> Date,
        work_type -> Integer,
        new_salary -> Integer,
    }
}

diesel::table! {
    employee_work (employee, on_date, on_project) {
        employee -> Integer,
        on_date -> Date,
        on_project -> Integer,
        work_type -> Integer,
    }
}

diesel::table! {
    employees (employee_id) {
        employee_id -> Integer,
        employee_name -> Text,
    }
}

diesel::table! {
    expenses (on_time) {
        on_time -> Timestamp,
        project -> Nullable<Integer>,
        reason -> Text,
        amount -> Integer,
    }
}

diesel::table! {
    projects (project_id) {
        project_id -> Integer,
        parent_company -> Text,
        project_name -> Text,
        estimate -> Nullable<Integer>,
    }
}

diesel::table! {
    tool_adj (rowid) {
        rowid -> Integer,
        tool -> Integer,
        adjective -> Text,
    }
}

diesel::table! {
    tool_cost (tool, change_date) {
        tool -> Integer,
        change_date -> Date,
        new_cost -> Integer,
    }
}

diesel::table! {
    tool_movement (tool, from_time, to_time) {
        tool -> Integer,
        from_time -> Timestamp,
        to_time -> Timestamp,
        from_project -> Nullable<Integer>,
        to_project -> Nullable<Integer>,
        by_employee -> Integer,
    }
}

diesel::table! {
    tools (tool_id) {
        tool_id -> Integer,
        tool_name -> Text,
    }
}

diesel::joinable!(employee_paid -> employees (employee));
diesel::joinable!(employee_salary -> employees (employee));
diesel::joinable!(employee_work -> employees (employee));
diesel::joinable!(employee_work -> projects (on_project));
diesel::joinable!(expenses -> projects (project));
diesel::joinable!(projects -> companies (parent_company));
diesel::joinable!(tool_adj -> tools (tool));
diesel::joinable!(tool_cost -> tools (tool));
diesel::joinable!(tool_movement -> employees (by_employee));
diesel::joinable!(tool_movement -> tools (tool));

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    employee_paid,
    employee_salary,
    employee_work,
    employees,
    expenses,
    projects,
    tool_adj,
    tool_cost,
    tool_movement,
    tools,
);
