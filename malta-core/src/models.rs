use std::collections::HashSet;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

// Models
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub estimate: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FakeID {
    pub id: RecordId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: RecordId,
    pub name: String,
    pub estimate: Option<Decimal>,
    pub incomes: Option<Decimal>,
    pub expenses: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralIncome {
    pub source: String,
    pub on_date: DateTime<Utc>,
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralExpense {
    pub reason: String,
    pub on_date: DateTime<Utc>,
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmployee {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub id: RecordId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTool {
    pub name: String,
    pub adjectives: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    pub id: RecordId,
    pub name: String,
    pub adjectives: HashSet<String>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SalaryTracking {
//     pub employee: RecordId,
//     pub change_date: Datetime,
//     pub work_type: WorkType,
//     pub new_salary: Decimal,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Record {
//     pub id: RecordId,
// }
