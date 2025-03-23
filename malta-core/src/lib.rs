pub mod models;

use std::path::PathBuf;

use chrono::{DateTime, Utc};
pub use models::{
    CreateEmployee, CreateProject, CreateTool, Employee, FakeID, GeneralExpense, GeneralIncome,
    Project, Tool,
};
use once_cell::sync::Lazy;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use surrealdb::{engine::local::Db, Datetime, RecordIdKey, Result, Surreal};

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
    pub async fn add_project(&self, create_info: CreateProject) -> std::result::Result<(), String> {
        self.0
            .create::<Option<FakeID>>("project")
            .content(create_info)
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }
    pub async fn add_project_income(
        &self,
        project: RecordIdKey,
        on_date: Option<DateTime<Utc>>,
        amount: Decimal,
    ) -> std::result::Result<(), String> {
        self.0
            .query(include_str!("../queries/create_income.surql"))
            .bind(("source", project))
            .bind((
                "on_date",
                on_date.map(Datetime::from).unwrap_or(Utc::now().into()),
            ))
            .bind(("amount", amount))
            .await
            .map_err(|e| format!("{e}"))?
            .take::<Option<FakeID>>(0)
            .map_err(|e| format!("{e}"))
            .map(|_| ())
    }
    pub async fn add_general_income(
        &self,
        source: String,
        on_date: Option<DateTime<Utc>>,
        amount: Decimal,
    ) -> std::result::Result<(), String> {
        self.0
            .query(include_str!("../queries/create_income.surql"))
            .bind(("source", source))
            .bind(("on_date", on_date.map(Datetime::from)))
            .bind(("amount", amount))
            .await
            .map_err(|e| format!("{e}"))?
            .take::<Option<FakeID>>(0)
            .map_err(|e| format!("{e}"))
            .map(|_| ())
    }
    pub async fn add_project_expense(
        &self,
        project: RecordIdKey,
        on_date: Option<DateTime<Utc>>,
        reason: String,
        amount: Decimal,
    ) -> std::result::Result<(), String> {
        self.0
            .query(include_str!("../queries/create_income.surql"))
            .bind(("project", project))
            .bind(("on_date", on_date.map(Datetime::from)))
            .bind(("reason", reason))
            .bind(("amount", amount))
            .await
            .map_err(|e| format!("{e}"))?
            .take::<Option<FakeID>>(0)
            .map_err(|e| format!("{e}"))
            .map(|_| ())
    }
    pub async fn add_general_expense(
        &self,
        on_date: Option<DateTime<Utc>>,
        reason: String,
        amount: Decimal,
    ) -> std::result::Result<(), String> {
        self.0
            .query(include_str!("../queries/create_expense.surql"))
            .bind(("project", None::<RecordIdKey>))
            .bind(("on_date", on_date.map(Datetime::from)))
            .bind(("reason", reason))
            .bind(("amount", amount))
            .await
            .map_err(|e| format!("{e}"))?
            .take::<Option<FakeID>>(0)
            .map_err(|e| format!("{e}"))
            .map(|_| ())
    }
    pub async fn get_income(&self) -> Result<Decimal> {
        Ok(self
            .0
            .query(include_str!("../queries/get_income.surql"))
            .await?
            .take::<Option<Decimal>>(0)?
            .unwrap_or(dec!(0)))
    }
    pub async fn get_expense(&self) -> Result<Decimal> {
        Ok(self
            .0
            .query(include_str!("../queries/get_expense.surql"))
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
    pub async fn add_employee(
        &self,
        create_info: CreateEmployee,
    ) -> std::result::Result<(), String> {
        self.0
            .create::<Option<FakeID>>("employee")
            .content(create_info)
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }
    pub async fn add_tool(&self, create_info: CreateTool) -> std::result::Result<(), String> {
        self.0
            .create::<Option<FakeID>>("tool")
            .content(create_info)
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }
    pub async fn get_project(&self, record: RecordIdKey) -> Result<Option<Project>> {
        self.0.select(("project", record)).await
    }
    pub async fn get_all_projects(&self) -> Result<Vec<Project>> {
        self.0.select("project").await
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
    // pub async fn project_stream<F, Fut>(&self, mut callback: F)
    // where
    //     F: FnMut(Notification<Project>) -> Fut,
    //     Fut: Future<Output = ()>,
    // {
    //     use futures::StreamExt as _;
    //     if let Ok(mut stream) = self.0.select::<Vec<Project>>("project").live().await {
    //         while let Some(result) = stream.next().await {
    //             if let Ok(notification) = result {
    //                 callback(notification).await;
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    // }
}
