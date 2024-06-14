use diesel::prelude::*;
use diesel_migrations::*;
use models::*;
use once_cell::sync::Lazy;
use std::path::PathBuf;

mod models;
mod schema;

static EXE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    std::env::current_exe()
        .unwrap_or_else(|e| panic!("Cannot get executable location: {e}"))
        .parent()
        .expect("Executable's parent is inaccessible...")
        .to_path_buf()
});
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct Database(SqliteConnection);
impl Database {
    pub fn open() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = SqliteConnection::establish(&{
            let mut path = EXE_PATH.clone();
            path.push("malta.db");
            path.to_string_lossy().into_owned()
        })?;
        conn.run_pending_migrations(MIGRATIONS)?;
        Ok(Self(conn))
    }
    pub fn get_companies(&mut self) -> QueryResult<Vec<Company>> {
        use schema::companies::dsl::*;
        companies.load::<Company>(&mut self.0)
    }
}
