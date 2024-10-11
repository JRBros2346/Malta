use std::path::PathBuf;

use once_cell::sync::Lazy;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::{Db, SpeeDb},
    sql::Thing,
    Result, Surreal,
};

// Models
#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub estimate: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: Thing,
}

static DB_PATH: Lazy<PathBuf> = Lazy::new(|| std::env::current_exe().unwrap().parent().unwrap().join("malta.db"));

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
    pub async fn add_project(&self, name: String, estimate: Option<Decimal>) -> Result<Vec<Record>> {
        self.0.create("project")
            .content(Project { name, estimate })
            .await
    }
    pub async fn projects(&self) -> Result<Vec<Project>> {
        self.0.select::<Vec<Project>>("project").await
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
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
        println!("{:?}", std::env::current_exe().unwrap());
        let db = get_db().await;
        let project_name = String::from("New Project");
        let estimate = Some(Decimal::new(100000, 2));
        
        // Add the project
        let project_id = db.add_project(project_name.clone(), estimate).await?;
        println!("{project_id:?}");
        assert_eq!(project_id.len(), 1, "Only one Project ID should be returned");

        // Retrieve all projects and check if our project was added
        let project = db.get::<Project>(&project_id[0]).await?;
        assert!(!project.is_none(), "Project should be available");

        Ok(())
    }

    #[tokio::test]
    async fn test_projects_list() -> Result<()> {
        let db = get_db().await;
        
        // Make sure there are some projects
        db.add_project("Project A".to_string(), Some(Decimal::new(50000, 2))).await?;
        db.add_project("Project B".to_string(), None).await?;

        // Retrieve the list of projects
        let projects = db.projects().await?;
        assert!(projects.len() >= 2, "There should be at least 2 projects in the database");
        Ok(())
    }
}
