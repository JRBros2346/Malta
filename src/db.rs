use rusqlite::*;
use std::borrow::Cow;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    Rusqlite(rusqlite::Error),
    Io(std::io::Error),
    CompanyExists,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rusqlite(e) => {
                write!(f, "{e}")
            }
            Self::Io(e) => {
                write!(f, "{e}")
            }
            Self::CompanyExists => {
                write!(f, "Company Already Exists!")
            }
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Rusqlite(e) => Some(e),
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

pub fn exe_path() -> PathBuf {
    std::env::current_exe()
        .expect("Cannot get executable location...")
        .parent()
        .expect("Executable's parent is inaccessible...")
        .to_owned()
}

pub fn get_companies<'a>() -> Result<Vec<Cow<'a, str>>> {
    let mut path = exe_path();
    path.push("tables");
    let x = std::fs::read_dir(path)
        .map_err(Error::Io)?
        .filter_map(|p| p.ok())
        .collect::<Vec<_>>();

    Ok(Vec::new())
}

pub struct Database(Connection);
impl Database {
    pub fn create(name: &str) -> Result<Self> {
        let mut path = exe_path();
        path.push("tables");
        if !path.exists() {
            let _ = std::fs::create_dir_all(&path);
        }
        path.push(format!("{name}.db"));
        if path.exists() {
            return Err(Error::CompanyExists);
        }
        eprintln!("{path:?}");
        let conn = Connection::open(path).map_err(Error::Rusqlite)?;
        conn.execute(
            "
            CREATE TABLE projects (
                project_id INTEGER PRIMARY KEY,
                project_name TEXT NOT NULL,
                project_estimate INTEGER
            )
        ",
            [],
        )
        .map_err(Error::Rusqlite)?;
        conn.execute(
            "
            CREATE TABLE employees (
                employee_id INTEGER PRIMARY KEY,
                employee_name TEXT NOT NULL
            )
        ",
            [],
        )
        .map_err(Error::Rusqlite)?;
        conn.execute(
            "
            CREATE TABLE tools (
                tool_id INTEGER PRIMARY KEY,
                tool_name TEXT NOT NULL
            )
        ",
            [],
        )
        .map_err(Error::Rusqlite)?;
        conn.execute(
            "
            CREATE TABLE employee_salary (
                date INTEGER NOT NULL,
                employee_id INTEGER NOT NULL,
                salary INTEGER NOT NULL,

                PRIMARY KEY (date, employee_id),
                FOREIGN KEY (employee_id) REFERENCES employees(employee_id)
            )
        ",
            [],
        )
        .map_err(Error::Rusqlite)?;
        conn.execute(
            "
            CREATE TABLE tools_stati (
                track_id INTEGER PRIMARY KEY,
                from_date INTEGER NOT NULL,
                to_date INTEGER NOT NULL,
                tool_id INTEGER NOT NULL,
                employee_id INTEGER NOT NULL,
                from_project INTEGER NULLABLE,
                to_project INTEGER NULLABLE,

                FOREIGN KEY (tool_id) REFERENCES tools(tool_id),
                FOREIGN KEY (employee_id) REFERENCES employees(employee_id),
                FOREIGN KEY (from_project) REFERENCES projects(project_id),
                FOREIGN KEY (to_project) REFERENCES projects(project_id)
            )
        ",
            [],
        )
        .map_err(Error::Rusqlite)?;
        conn.execute(
            "
            CREATE TABLE expenses (
                expense_id INTEGER PRIMARY KEY,
                date INTEGER NOT NULL,
                project_id INTEGER NULLABLE,
                expense INTEGER NOT NULL,

                FOREIGN KEY (project_id) REFERENCES projects(project_id)
            )
        ",
            [],
        )
        .map_err(Error::Rusqlite)?;
        Ok(Self(conn))
    }
}
