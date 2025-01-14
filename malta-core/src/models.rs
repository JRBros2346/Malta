use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

// Models
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub estimate: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalaryTracking {
    pub employee: Thing,
    pub change_date: Datetime,
    pub work_type: WorkType,
    pub new_salary: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Thing,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum WorkType {
    Full = 4,
    Half = 2,
    Quarter = 1,
}
impl Serialize for WorkType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}
struct WorkTypeVisitor;
impl<'de> serde::de::Visitor<'de> for WorkTypeVisitor {
    type Value = WorkType;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a u8 representing a WorkType")
    }
    fn visit_u8<E>(self, value: u8) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            4 => Ok(WorkType::Full),
            2 => Ok(WorkType::Half),
            1 => Ok(WorkType::Quarter),
            _ => Err(E::custom(format!("unknown value: {}", value))),
        }
    }
}
impl<'de> Deserialize<'de> for WorkType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u8(WorkTypeVisitor)
    }
}
