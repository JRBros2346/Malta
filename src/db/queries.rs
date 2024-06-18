use gluesql::core::ast_builder::*;
use gluesql::core::error::SelectError;
use gluesql::prelude::*;

use crate::types::Money;

impl super::Database {
    pub(super) async fn create_companies(&mut self) -> Result<()> {
        assert_eq!(
            table("companies")
                .create_table_if_not_exists()
                .add_column("company TEXT PRIMARY KEY")
                .execute(&mut self.0)
                .await?,
            Payload::Create
        );
        Ok(())
    }
    pub(super) async fn create_projects(&mut self) -> Result<()> {
        assert_eq!(
            table("projects")
                .create_table_if_not_exists()
                .add_column("id INT PRIMARY KEY")
                .add_column("company TEXT NOT NULL")
                .add_column("project TEXT NOT NULL")
                .add_column("estimate DECIMAL")
                .execute(&mut self.0)
                .await?,
            Payload::Create
        );
        Ok(())
    }
    pub(super) async fn create_employees(&mut self) -> Result<()> {
        assert_eq!(
            table("employees")
                .create_table_if_not_exists()
                .add_column("id INT PRIMARY KEY")
                .add_column("employee TEXT NOT NULL")
                .execute(&mut self.0)
                .await?,
            Payload::Create
        );
        Ok(())
    }
    pub(super) async fn create_salaries(&mut self) -> Result<()> {
        assert_eq!(
            table("salaries")
                .create_table_if_not_exists()
                .add_column("employee INT NOT NULL")
                .add_column("change_date DATE NOT NULL DEFAULT NOW()")
                .add_column("work_type UINT8 NOT NULL")
                .add_column("new_salary DECIMAL NOT NULL")
                .execute(&mut self.0)
                .await?,
            Payload::Create
        );
        Ok(())
    }
    pub(super) async fn create_worked(&mut self) -> Result<()> {
        assert_eq!(
            table("worked")
                .create_table_if_not_exists()
                .add_column("employee INT NOT NULL")
                .add_column("on_date DATE NOT NULL DEFAULT NOW()")
                .add_column("on_project INT NOT NULL")
                .add_column("work_type UINT8 NOT NULL")
                .execute(&mut self.0)
                .await?,
            Payload::Create
        );
        Ok(())
    }
    pub(super) async fn create_payment(&mut self) -> Result<()> {
        assert_eq!(
            table("payment")
                .create_table_if_not_exists()
                .add_column("employee INT NOT NULL")
                .add_column("on_date DATE NOT NULL DEFAULT NOW()")
                .add_column("amount DECIMAL NOT NULL")
                .execute(&mut self.0)
                .await?,
            Payload::Create
        );
        Ok(())
    }
    pub(super) async fn create_tools(&mut self) -> Result<()> {
        assert_eq!(
            table("tools")
                .create_table_if_not_exists()
                .add_column("id INT PRIMARY KEY")
                .add_column("tool TEXT NOT NULL")
                .add_column("adjectives LIST DEFAULT '[]'")
                .execute(&mut self.0)
                .await?,
            Payload::Create
        );
        Ok(())
    }
    pub(super) async fn create_tool_costs(&mut self) -> Result<()> {
        assert_eq!(
            table("tool_costs")
                .create_table_if_not_exists()
                .add_column("tool INT NOT NULL")
                .add_column("change_date DATE NOT NULL DEFAULT NOW()")
                .add_column("new_cost DECIMAL NOT NULL")
                .execute(&mut self.0)
                .await?,
            Payload::Create
        );
        Ok(())
    }
    pub(super) async fn create_tools_tracking(&mut self) -> Result<()> {
        assert_eq!(
            table("tools_tracking")
                .create_table_if_not_exists()
                .add_column("tool INT NOT NULL")
                .add_column("from_time TIMESTAMP NOT NULL DEFAULT NOW()")
                .add_column("duration INTERVAL")
                .add_column("to_project INT")
                .add_column("by_employee INT NOT NULL")
                .execute(&mut self.0)
                .await?,
            Payload::Create
        );
        Ok(())
    }
    pub(super) async fn create_expenses(&mut self) -> Result<()> {
        assert_eq!(
            table("expenses")
                .create_table_if_not_exists()
                .add_column("on_time TIMESTAMP PRIMARY KEY DEFAULT NOW()")
                .add_column("project INTEGER")
                .add_column("reason TEXT")
                .add_column("amount DECIMAL NOT NULL")
                .execute(&mut self.0)
                .await?,
            Payload::Create
        );
        Ok(())
    }
    pub(super) async fn insert_company(&mut self, name: &str) -> Result<()> {
        assert_eq!(
            table("companies")
                .insert()
                .values(vec![name])
                .execute(&mut self.0)
                .await?,
            Payload::Insert(1)
        );
        Ok(())
    }
    pub(super) async fn select_companies(&mut self) -> Result<Vec<String>> {
        match table("companies").select().execute(&mut self.0).await? {
            Payload::Select { labels, rows } => {
                assert_eq!(labels, vec!["company"]);
                Ok(rows
                    .into_iter()
                    .map(|row| match row.as_slice() {
                            [Value::Str(s)] => s.clone(),
                            v => panic!("Expected `&[Value::Str(_)]`: {v:?}"),
                        })
                    .collect::<Vec<_>>())
            }
            p => panic!("Expected `Payload::Select {{..}}`: {p:?}"),
        }
    }
    pub(super) async fn insert_project(&mut self, name: &str, company: &str, estimate: Option<Money>) -> Result<()> {
        assert_eq!(
            table("projects")
                .insert()
                .values(vec![vec![text(name), text(company), estimate.map(num).unwrap_or(null())]])
                .execute(&mut self.0)
                .await?,
            Payload::Insert(1)
        );
        Ok(())
    }
}
