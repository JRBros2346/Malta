use surrealdb::{engine::remote::ws::{Client, Ws}, Connect, Surreal};

pub struct Database(Connect<Client, Surreal<Client>>);
impl Database {
    pub fn open() -> Result<Self, Error> {
        let db = Self(Surreal::new::<Ws>(format!(
            "{}:{}",
            dotenv::var("DB_HOST")?,
            dotenv::var("DB_PORT")?,
        )));
        Ok(db)
    }
    // pub fn add_company(&mut self, name: &str) -> Result<()> {
    //     futures::executor::block_on(self.insert_company(name))
    // }
    // pub fn get_companies(&mut self) -> Result<Vec<String>> {
    //     futures::executor::block_on(self.select_companies())
    // }
    // pub fn add_project(
    //     &mut self,
    //     name: &str,
    //     company: &str,
    //     estimate: Option<Money>,
    // ) -> Result<()> {
    //     futures::executor::block_on(self.insert_project(name, company, estimate))
    // }
}

#[derive(Debug)]
pub enum Error {
    Db(surrealdb::Error),
    Env(dotenv::Error),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Db(e) => write!(f, "{e}"),
            Self::Env(e) => write!(f, "{e}"),
        }
    }
}
impl From<surrealdb::Error> for Error {
    fn from(value: surrealdb::Error) -> Self {
        Self::Db(value)
    }
}
impl From<dotenv::Error> for Error {
    fn from(value: dotenv::Error) -> Self {
        Self::Env(value)
    }
}
impl std::error::Error for Error {
    
}