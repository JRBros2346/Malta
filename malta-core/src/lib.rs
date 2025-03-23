pub mod models;

use std::path::PathBuf;

pub use models::{CreateEmployee, CreateProject, CreateTool, Employee, FakeID, Project, Tool};
use once_cell::sync::Lazy;
use surrealdb::{engine::local::Db, RecordIdKey, Result, Surreal};

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
        use surrealdb::engine::local::SurrealKv;
        let db = Self(Surreal::new::<SurrealKv>(DB_PATH.as_path()).await?);
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
    pub async fn add_project(&self, create_info: CreateProject) -> bool {
        matches!(
            self.0
                .create::<Option<FakeID>>("project")
                .content(create_info)
                .await,
            Ok(Some(_))
        )
    }
    pub async fn add_employee(&self, create_info: CreateEmployee) -> bool {
        matches!(
            self.0
                .create::<Option<FakeID>>("employee")
                .content(create_info)
                .await,
            Ok(Some(_))
        )
    }
    pub async fn add_tool(&self, create_info: CreateTool) -> bool {
        matches!(
            self.0
                .create::<Option<FakeID>>("tool")
                .content(create_info)
                .await,
            Ok(Some(_))
        )
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
