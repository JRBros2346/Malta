use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json, Response},
};
use malta_core::{
    surrealdb::{self, RecordId, RecordIdKey},
    tracing::{self, instrument},
    Malta,
};
use thiserror::Error;

#[instrument(name = "get_all_projects", level = "info", skip(db))]
pub async fn get_all_projects(State(db): State<Malta>) -> Result<Html<String>, AppError> {
    #[derive(Template)]
    #[template(path = "projects.html")]
    pub struct Projects {
        projects: Vec<malta_core::models::Project>,
    }
    let view = Projects {
        projects: db.get_all_projects().await?,
    };
    Ok(Html(view.render()?))
}

#[instrument(name = "get_project", level = "info", skip(db))]
pub async fn get_project(
    State(db): State<Malta>,
    Path(id): Path<String>,
) -> Result<Html<String>, AppError> {
    #[derive(Template)]
    #[template(path = "project.html")]
    pub struct Project {
        project: malta_core::models::Project,
        incomes: Vec<malta_core::models::GeneralIncome>,
        expenses: Vec<malta_core::models::GeneralExpense>,
    }
    let mut id = id.split(":");
    let tb = id.next().ok_or(AppError::NotFound)?;
    let id = id.next().ok_or(AppError::NotFound)?;
    let view = Project {
        project: dbg!(db.get_project(RecordId::from((tb, id))).await)?.ok_or(AppError::NotFound)?,
        incomes: dbg!(db.get_project_incomes(RecordIdKey::from(id)).await)?,
        expenses: dbg!(db.get_project_expenses(RecordIdKey::from(id)).await)?,
    };
    Ok(Html(view.render()?))
}

#[instrument(name = "create_project", level = "info", skip(db))]
pub async fn create_project(
    State(db): State<Malta>,
    Json(create_info): Json<malta_core::models::CreateProject>,
) -> Result<Json<String>, AppError> {
    let x = db.add_project(create_info).await?;
    Ok(Json(serde_json::to_string(&x.unwrap()).unwrap()))
}

// #[instrument(name = "add_income", level = "info", skip(db))]
// pub async fn add_income(
//     State(db): State<Malta>,
//     Json(create_info): Json<malta_core::models::CreateProject>,
// ) -> Result<Json<String>, AppError> {
//     let x = db.add_project_income(create_info).await?;
//     Ok(Json(serde_json::to_string(&x.unwrap()).unwrap()))
// }

// #[instrument(name = "add_expense", level = "info", skip(db))]
// pub async fn add_expense(
//     State(db): State<Malta>,
//     Json(create_info): Json<malta_core::models::CreateProject>,
// ) -> Result<Json<String>, AppError> {
//     let x = db.add_project(create_info).await?;
//     Ok(Json(serde_json::to_string(&x.unwrap()).unwrap()))
// }

pub async fn not_found() -> impl IntoResponse {
    AppError::NotFound
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Askama: {0}")]
    Render(#[from] askama::Error),
    #[error("Surreal: {0}")]
    Db(#[from] surrealdb::Error),
    #[error("Not Found")]
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
        };
        (status, Html(self.to_string())).into_response()
    }
}
