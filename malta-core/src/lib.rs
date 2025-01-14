pub mod models;

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use models::SalaryTracking;
pub use models::{Employee, Project, Record, WorkType};
use once_cell::sync::Lazy;
use rust_decimal::prelude::*;
use serde::Deserialize;
use surrealdb::{
    engine::local::{Db, SpeeDb},
    sql::{Datetime, Thing},
    Result, Surreal,
};

static DB_PATH: Lazy<PathBuf> = Lazy::new(|| {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("malta.db")
});

pub struct Malta(Surreal<Db>);
impl Malta {
    pub async fn open() -> Result<Self> {
        let db = Self(Surreal::new::<SpeeDb>(DB_PATH.as_path()).await?);
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
    pub async fn get<T: for<'a> Deserialize<'a>>(&self, record: &Record) -> Result<Option<T>> {
        self.0.select(record.id.clone()).await
    }
    pub async fn remove<T: for<'a> Deserialize<'a>>(&self, record: &Record) -> Result<Option<T>> {
        self.0.delete(record.id.clone()).await
    }
    pub async fn add_project(
        &self,
        name: String,
        estimate: Option<Decimal>,
    ) -> Result<Option<Record>> {
        self.0
            .create("project")
            .content(Project { name, estimate })
            .await
            .map(|mut v| v.pop())
    }
    pub async fn projects(&self) -> Result<Vec<Project>> {
        self.0.select("project").await
    }
    pub async fn add_employee(&self, name: String) -> Result<Option<Record>> {
        self.0
            .create("employee")
            .content(Employee { name })
            .await
            .map(|mut v| v.pop())
    }
    pub async fn employees(&self) -> Result<Vec<Employee>> {
        self.0.select("employee").await
    }
    pub async fn change_salary(
        &self,
        employee: Thing,
        change_date: DateTime<Utc>,
        new_salary: Decimal,
        work_type: WorkType,
    ) -> Result<Option<Record>> {
        self.0
            .create::<Vec<Record>>("salary_tracking")
            .content(SalaryTracking {
                employee,
                change_date: change_date.into(),
                new_salary,
                work_type,
            })
            .await
            .map(|mut v| v.pop())
    }
    pub async fn get_salary_today(&self, employee: Thing, work_type: WorkType) -> Option<Decimal> {
        self.get_salary_on(employee, work_type, Utc::now()).await
    }
    pub async fn get_salary_on(
        &self,
        employee: Thing,
        work_type: WorkType,
        on_date: DateTime<Utc>,
    ) -> Option<Decimal> {
        let mut res = self
            .0
            .query(include_str!("../queries/query_salary.surql"))
            .bind(("employee", employee))
            .bind(("work", work_type))
            .bind(("date", Datetime::from(on_date)))
            .await
            .unwrap();
        res.take(0).unwrap()
    }
}

#[cfg(test)]
impl Drop for Malta {
    fn drop(&mut self) {
        std::fs::remove_dir_all(DB_PATH.as_path()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use once_cell::sync::Lazy;
    use rust_decimal_macros::dec;
    use surrealdb::Result;
    use tokio::sync::OnceCell;

    static MALTA: Lazy<OnceCell<Malta>> = Lazy::new(OnceCell::new);

    async fn get_db() -> &'static Malta {
        async fn init_db() -> Malta {
            Malta::open().await.unwrap()
        }
        MALTA.get_or_init(init_db).await
    }
    #[tokio::test]
    async fn test_add_project() -> Result<()> {
        let db = get_db().await;

        // Add a project
        let project_name = "New Project".to_string();
        let project_estimate = Some(dec!(5000.00));
        let record = db
            .add_project(project_name.clone(), project_estimate)
            .await?;

        // Ensure the project was added successfully
        assert!(record.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_add_employee() -> Result<()> {
        let db = get_db().await;

        // Add an employee
        let employee_name = "John Doe".to_string();
        let record = db.add_employee(employee_name.clone()).await?;

        // Ensure the employee was added successfully
        assert!(record.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_change_salary() -> Result<()> {
        let db = get_db().await;

        // Add an employee
        let employee_name = "Jane Smith".to_string();
        let employee_record = db.add_employee(employee_name.clone()).await?.unwrap();

        // Change salary for the employee
        let change_date = Utc::now();
        let new_salary = dec!(7500.00);
        let work_type = WorkType::Full;
        let salary_record = db
            .change_salary(
                employee_record.id,
                change_date.into(),
                new_salary,
                work_type,
            )
            .await?;

        // Ensure the salary tracking record was added successfully
        assert!(salary_record.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_project() -> Result<()> {
        let db = get_db().await;

        // Add a project
        let project_name = "Project to Remove".to_string();
        let project_estimate = Some(dec!(1000.00));
        let record = db
            .add_project(project_name.clone(), project_estimate)
            .await?;

        // Ensure the project was added
        assert!(record.is_some());

        // Remove the project
        let removed_project = db
            .remove::<Project>(&Record {
                id: record.unwrap().id,
            })
            .await?;
        assert!(removed_project.is_some());

        Ok(())
    }
}
