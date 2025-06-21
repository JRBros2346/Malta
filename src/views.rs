use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json, Response},
};
use malta_core::{
    surrealdb,
    tracing::{self, instrument, Span},
    Malta,
};

#[instrument(name = "get_projects", level = "info", skip(db))]
pub async fn get_projects(State(db): State<Malta>) -> Result<Html<String>, AppError> {
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

#[instrument(name = "create_project", level = "info", skip(db))]
pub async fn create_project(
    State(db): State<Malta>,
    Json(create_info): Json<malta_core::models::CreateProject>,
) -> Result<Json<String>, AppError> {
    let x = db.add_project(create_info).await?;
    Ok(Json(serde_json::to_string(&x.unwrap()).unwrap()))
}

pub async fn not_found() -> Html<String> {
    #[derive(Template)]
    #[template(path = "error.html")]
    struct Error {}
    let error = Error {};
    Html(error.render().unwrap())
}

pub enum AppError {
    Render(askama::Error),
    Db(surrealdb::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Template)]
        #[template(path = "error.html")]
        struct Error {}
        let error = Error {};
        if let Ok(body) = error.render() {
            (StatusCode::INTERNAL_SERVER_ERROR, Html(body)).into_response()
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
        }
    }
}
impl From<askama::Error> for AppError {
    fn from(value: askama::Error) -> Self {
        Self::Render(value)
    }
}
impl From<surrealdb::Error> for AppError {
    fn from(value: surrealdb::Error) -> Self {
        Self::Db(value)
    }
}
