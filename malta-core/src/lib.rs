pub mod models;

use std::path::PathBuf;

pub use models::{
    CreateEmployee, CreateProject, CreateTool, Employee, FakeID, GeneralExpense, GeneralIncome,
    Project, Tool,
};
use once_cell::sync::Lazy;
use rust_decimal_macros::dec;
use surrealdb::{engine::local::Db, Datetime, RecordId, RecordIdKey, Result, Surreal};

pub use chrono::{DateTime, Utc};
pub use rust_decimal::Decimal;
pub use serde;
pub use surrealdb;
pub use tracing;

static DB_PATH: Lazy<PathBuf> = Lazy::new(|| {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("malta.db")
});

#[derive(Debug, Clone)]
pub struct Malta(Surreal<Db>);
impl Malta {
    pub async fn open() -> Result<Self> {
        use surrealdb::engine::local::RocksDb;
        let db = Self(Surreal::new::<RocksDb>(DB_PATH.as_path()).await?);
        db.0.use_ns("malta").use_db("malta").await?;
        db.setup().await?;
        Ok(db)
    }
    #[cfg(test)]
    pub async fn test_open() -> Result<Self> {
        use surrealdb::engine::local::Mem;
        let db = Self(Surreal::new::<Mem>(()).await?);
        db.0.use_ns("malta").use_db("malta").await?;
        db.setup().await?;
        Ok(db)
    }
    pub async fn setup(&self) -> Result<()> {
        self.0
            .query(include_str!("../queries/setup_database.surql"))
            .await?;
        Ok(())
    }
    pub async fn add_project(&self, create_info: CreateProject) -> Result<Option<FakeID>> {
        self.0
            .create::<Option<FakeID>>("project")
            .content(create_info)
            .await
    }
    pub async fn add_project_income(
        &self,
        project: RecordIdKey,
        on_date: Option<DateTime<Utc>>,
        amount: Decimal,
    ) -> Result<()> {
        self.0
            .query(include_str!("../queries/create_income.surql"))
            .bind(("source", project))
            .bind((
                "on_date",
                on_date.map(Datetime::from).unwrap_or(Utc::now().into()),
            ))
            .bind(("amount", amount))
            .await?
            .take::<Option<FakeID>>(0)
            .map(|_| ())
    }
    pub async fn add_general_income(
        &self,
        source: String,
        on_date: Option<DateTime<Utc>>,
        amount: Decimal,
    ) -> Result<()> {
        self.0
            .query(include_str!("../queries/create_income.surql"))
            .bind(("source", source))
            .bind(("on_date", on_date.map(Datetime::from)))
            .bind(("amount", amount))
            .await?
            .take::<Option<FakeID>>(0)
            .map(|_| ())
    }
    pub async fn add_project_expense(
        &self,
        project: RecordIdKey,
        on_date: Option<DateTime<Utc>>,
        reason: String,
        amount: Decimal,
    ) -> Result<()> {
        self.0
            .query(include_str!("../queries/create_income.surql"))
            .bind(("project", project))
            .bind(("on_date", on_date.map(Datetime::from)))
            .bind(("reason", reason))
            .bind(("amount", amount))
            .await?
            .take::<Option<FakeID>>(0)
            .map(|_| ())
    }
    pub async fn add_general_expense(
        &self,
        on_date: Option<DateTime<Utc>>,
        reason: String,
        amount: Decimal,
    ) -> Result<()> {
        self.0
            .query(include_str!("../queries/create_expense.surql"))
            .bind(("project", None::<RecordIdKey>))
            .bind(("on_date", on_date.map(Datetime::from)))
            .bind(("reason", reason))
            .bind(("amount", amount))
            .await?
            .take::<Option<FakeID>>(0)
            .map(|_| ())
    }
    pub async fn get_total_income(&self) -> Result<Decimal> {
        Ok(self
            .0
            .query(include_str!("../queries/get_total_income.surql"))
            .await?
            .take::<Option<Decimal>>(0)?
            .unwrap_or(dec!(0)))
    }
    pub async fn get_total_expense(&self) -> Result<Decimal> {
        Ok(self
            .0
            .query(include_str!("../queries/get_total_expense.surql"))
            .await?
            .take::<Option<Decimal>>(0)?
            .unwrap_or(dec!(0)))
    }
    pub async fn get_general_incomes(&self) -> Result<Vec<GeneralIncome>> {
        self.0
            .query(include_str!("../queries/get_general_incomes.surql"))
            .await?
            .take(0)
    }
    pub async fn get_general_expenses(&self) -> Result<Vec<GeneralExpense>> {
        self.0
            .query(include_str!("../queries/get_general_expenses.surql"))
            .await?
            .take(0)
    }

    pub async fn get_project_incomes(&self, project: RecordIdKey) -> Result<Vec<GeneralIncome>> {
        self.0
            .query(include_str!("../queries/get_project_incomes.surql"))
            .bind(("source", project))
            .await?
            .take(0)
    }
    pub async fn get_project_expenses(&self, project: RecordIdKey) -> Result<Vec<GeneralExpense>> {
        self.0
            .query(include_str!("../queries/get_project_expenses.surql"))
            .bind(("source", project))
            .await?
            .take(0)
    }

    pub async fn add_employee(&self, create_info: CreateEmployee) -> Result<()> {
        self.0
            .create::<Option<FakeID>>("employee")
            .content(create_info)
            .await?;
        Ok(())
    }
    pub async fn add_tool(&self, create_info: CreateTool) -> Result<()> {
        self.0
            .create::<Option<FakeID>>("tool")
            .content(create_info)
            .await?;
        Ok(())
    }
    pub async fn get_project(&self, record: String) -> Result<Option<Project>> {
        self.0
            .query(include_str!("../queries/get_project.surql"))
            .bind(("record", record))
            .await?
            .take(0)
    }
    pub async fn get_all_projects(&self) -> Result<Vec<Project>> {
        self.0
            .query(include_str!("../queries/get_all_projects.surql"))
            .await?
            .take(0)
    }
    pub async fn get_employee(&self, record: RecordIdKey) -> Result<Option<Employee>> {
        self.0.select(("employee", record)).await
    }
    pub async fn get_all_employees(&self) -> Result<Vec<Employee>> {
        self.0.select("employee").await
    }
    pub async fn get_tool(&self, record: RecordIdKey) -> Result<Option<Tool>> {
        self.0.select(("tool", record)).await
    }
    pub async fn get_all_tools(&self) -> Result<Vec<Tool>> {
        self.0.select("tool").await
    }
    pub async fn remove_project(&self, record: String) -> Result<bool> {
        self.0
            .delete::<Option<FakeID>>(("project", RecordIdKey::from(record)))
            .await
            .map(|b| b.is_some())
    }
    pub async fn remove_employee(&self, record: String) -> Result<bool> {
        self.0
            .delete::<Option<FakeID>>(("employee", RecordIdKey::from(record)))
            .await
            .map(|b| b.is_some())
    }
    pub async fn remove_tool(&self, record: String) -> Result<bool> {
        self.0
            .delete::<Option<FakeID>>(("tool", RecordIdKey::from(record)))
            .await
            .map(|b| b.is_some())
    }
}
