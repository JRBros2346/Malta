use diesel::prelude::*;
use super::schema::*;

#[derive(Queryable)]
pub struct Company {
    pub company_id: i32,
    pub company_name: String,
}

#[derive(Queryable)]
pub struct Project {
    pub project_id: i32,
    pub parent_company: i32,
    pub project_name: String,
    pub estimate: Option<i32>,
}

#[derive(Queryable)]
pub struct Employee {
    pub employee_id: i32,
    pub employee_name: String,
}

#[derive(Queryable)]
pub struct Tool {
    pub tool_id: i32,
    pub tool_name: String,
}

#[derive(Queryable)]
pub struct ToolAdj {
    pub tool: i32,
    pub adjective: String,
}

#[derive(Queryable)]
pub struct ToolMovement {
    pub tool: i32,
    pub from_time: i32,
    pub to_time: i32,
    pub from_project: Option<i32>,
    pub to_project: Option<i32>,
    pub by_employee: i32,
}

#[derive(Queryable)]
pub struct ToolCost {
    pub tool: i32,
    pub change_date: i32,
    pub new_cost: i32,
}

#[derive(Queryable)]
pub struct EmployeePaid {
    pub employee: i32,
    pub on_date: i32,
    pub amount: i32,
}

#[derive(Queryable)]
pub struct EmployeeSalary {
    pub employee: i32,
    pub change_date: i32,
    pub work_type: i32,
    pub new_salary: i32,
}

#[derive(Queryable)]
pub struct EmployeeWork {
    pub employee: i32,
    pub on_date: i32,
    pub on_project: i32,
    pub work_type: i32,
}

#[derive(Queryable)]
pub struct Expense {
    pub on_time: i32,
    pub project: Option<i32>,
    pub reason: String,
}
