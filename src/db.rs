use surrealdb::{engine::remote::ws::Ws, Surreal};

use crate::types::Money;

pub struct Database(Surreal<Ws>);
impl Database {
    pub fn open() -> Result<Self> {
        let mut db = SledStorage::new(DB_PATH.to_str().expect("Invalid Path to DB..!"))
            .map(Glue::new)
            .map(Self)?;
        // futures::executor::block_on(db.create_companies())?;
        // futures::executor::block_on(db.create_projects())?;
        // futures::executor::block_on(db.create_employees())?;
        // futures::executor::block_on(db.create_salaries())?;
        // futures::executor::block_on(db.create_worked())?;
        // futures::executor::block_on(db.create_payment())?;
        // futures::executor::block_on(db.create_tools())?;
        // futures::executor::block_on(db.create_tool_costs())?;
        // futures::executor::block_on(db.create_tools_tracking())?;
        // futures::executor::block_on(db.create_expenses())?;
        Ok(db)
    }
    pub fn add_company(&mut self, name: &str) -> Result<()> {
        futures::executor::block_on(self.insert_company(name))
    }
    pub fn get_companies(&mut self) -> Result<Vec<String>> {
        futures::executor::block_on(self.select_companies())
    }
    pub fn add_project(
        &mut self,
        name: &str,
        company: &str,
        estimate: Option<Money>,
    ) -> Result<()> {
        futures::executor::block_on(self.insert_project(name, company, estimate))
    }
}