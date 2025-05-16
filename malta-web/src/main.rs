use axum::{
    Json, Router,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{Html, IntoResponse},
    routing,
};
use chrono::{DateTime, Utc};
use malta_core::{CreateEmployee, CreateProject, CreateTool, Malta};
use memory_serve::{MemoryServe, load_assets};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use surrealdb::RecordIdKey;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", routing::get(main_page))
        .route(
            "/styles",
            routing::get(async || {
                (
                    [(header::CONTENT_TYPE, "text/css")],
                    include_str!("../styles.css"),
                )
            }),
        )
        .nest("/js", MemoryServe::new(load_assets!("js")).into_router())
        .route("/income", routing::post(add_income))
        .route("/expense", routing::post(add_expense))
        .route("/project", routing::get(all_projects))
        .route("/project", routing::post(new_project))
        .route("/project/{id}", routing::get(project_page))
        // .route("/project/{id}", routing::put(edit_project))
        .route("/project/{id}", routing::delete(delete_project))
        .route("/project/{id}/income", routing::post(add_project_income))
        .route("/project/{id}/expense", routing::post(add_project_expense))
        .route("/employee", routing::get(all_employees))
        .route("/employee", routing::post(new_employee))
        // .route("/employee/{id}", routing::get(employee_page))
        // .route("/employee/{id}", routing::put(edit_employee))
        .route("/employee/{id}", routing::delete(delete_employee))
        .route("/tool", routing::get(all_tools))
        .route("/tool", routing::post(new_tool))
        // .route("/tool/{id}", routing::get(tool_page))
        // .route("/tool/{id}", routing::put(edit_tool))
        .route("/tool/{id}", routing::delete(delete_tool))
        .fallback(async || StatusCode::IM_A_TEAPOT)
        .with_state(Malta::open().await.unwrap());
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn main_page(State(db): State<Malta>) -> impl IntoResponse {
    Html(format!(
        include_str!("../templates/main.html"),
        income = db.get_income().await.unwrap_or(dec!(0)),
        expense = db.get_expense().await.unwrap_or(dec!(0)),
        incomes = db
            .get_general_incomes()
            .await
            .unwrap_or(vec![])
            .into_iter()
            .fold(String::new(), |mut a, e| {
                a.push_str(&format!(
                    "<li>{} - {} - {}</li>",
                    e.on_date, e.source, e.amount
                ));
                a
            }),
        expenses = db
            .get_general_expenses()
            .await
            .unwrap_or(vec![])
            .into_iter()
            .fold(String::new(), |mut a, e| {
                a.push_str(&format!(
                    "<li>{} - {} - {}</li>",
                    e.on_date, e.reason, e.amount
                ));
                a
            }),
    ))
}

async fn add_income(
    State(db): State<Malta>,
    Json(payload): Json<CreateGeneralIncome>,
) -> impl IntoResponse {
    Json(
        db.add_general_income(payload.source, payload.on_date, payload.amount)
            .await,
    )
}

async fn add_expense(
    State(db): State<Malta>,
    Json(payload): Json<CreateGeneralExpense>,
) -> impl IntoResponse {
    Json(
        db.add_general_expense(payload.on_date, payload.reason, payload.amount)
            .await,
    )
}

async fn all_projects(State(db): State<Malta>) -> impl IntoResponse {
    Html(format!(
        include_str!("../templates/projects.html"),
        projects = match db.get_all_projects().await {
            Ok(list) => list
                .into_iter()
                .map(|p| format!(
                    "<li><a href=\"/project/{0}\">{1}</a>: {2} {3} {4}</li>",
                    p.id.key().to_string(),
                    p.name,
                    p.estimate.unwrap_or(dec!(0)),
                    0,
                    0
                ))
                .fold(String::new(), |mut a, e| {
                    a.push_str(&e);
                    a
                }),
            Err(e) => e.to_string(),
        }
    ))
}

async fn all_employees(State(db): State<Malta>) -> impl IntoResponse {
    Html(format!(
        include_str!("../templates/employees.html"),
        employees = match db.get_all_employees().await {
            Ok(list) => list
                .into_iter()
                .map(|p| format!(
                    "<li><a href=\"/employee/{0}\">{1}</a></li>",
                    p.id.key().to_string(),
                    p.name
                ))
                .fold(String::new(), |mut a, e| {
                    a.push_str(&e);
                    a
                }),
            Err(e) => e.to_string(),
        }
    ))
}

async fn all_tools(State(db): State<Malta>) -> impl IntoResponse {
    Html(format!(
        include_str!("../templates/tools.html"),
        tools = match db.get_all_tools().await {
            Ok(list) => list
                .into_iter()
                .map(|p| format!(
                    "<li><a href=\"/tool/{0}\">{1}</a>: {2}</li>",
                    p.id.key().to_string(),
                    p.name,
                    p.adjectives.into_iter().collect::<Vec<_>>().join(", ")
                ))
                .fold(String::new(), |mut a, e| {
                    a.push_str(&e);
                    a
                }),
            Err(e) => e.to_string(),
        }
    ))
}

async fn new_project(
    State(db): State<Malta>,
    Json(payload): Json<CreateProject>,
) -> impl IntoResponse {
    Json(db.add_project(payload).await)
}

async fn new_employee(
    State(db): State<Malta>,
    Json(payload): Json<CreateEmployee>,
) -> impl IntoResponse {
    Json(db.add_employee(payload).await)
}

async fn new_tool(State(db): State<Malta>, Json(payload): Json<CreateTool>) -> impl IntoResponse {
    Json(db.add_tool(payload).await)
}

async fn delete_project(State(db): State<Malta>, Path(id): Path<String>) -> impl IntoResponse {
    Json(db.remove_project(id).await)
}
async fn delete_employee(State(db): State<Malta>, Path(id): Path<String>) -> impl IntoResponse {
    Json(db.remove_employee(id).await)
}
async fn delete_tool(State(db): State<Malta>, Path(id): Path<String>) -> impl IntoResponse {
    Json(db.remove_tool(id).await)
}

async fn project_page(
    State(db): State<Malta>,
    Path(id): Path<String>,
) -> Result<Html<String>, String> {
    let project = db
        .get_project(id.into())
        .await
        .map_err(|e| format!("{e}"))?
        .ok_or("Project not found")?;
    Ok(Html(format!(
        include_str!("../templates/project.html"),
        name = project.name,
        estimate = project.estimate.unwrap_or(dec!(0)),
        income_expense = "",
        income = "",
        expense = "",
        id = project.id.key().to_string(),
    )))
}

async fn add_project_income(
    State(db): State<Malta>,
    Path(id): Path<String>,
    Json(payload): Json<CreateProjectIncome>,
) -> impl IntoResponse {
    Json(
        db.add_project_income(RecordIdKey::from(id), payload.on_date, payload.amount)
            .await,
    )
}

async fn add_project_expense(
    State(db): State<Malta>,
    Path(id): Path<String>,
    Json(payload): Json<CreateProjectExpense>,
) -> impl IntoResponse {
    Json(
        db.add_project_expense(
            RecordIdKey::from(id),
            payload.on_date,
            payload.reason,
            payload.amount,
        )
        .await,
    )
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateGeneralIncome {
    pub source: String,
    pub on_date: Option<DateTime<Utc>>,
    pub amount: Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateGeneralExpense {
    pub on_date: Option<DateTime<Utc>>,
    pub reason: String,
    pub amount: Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateProjectIncome {
    pub on_date: Option<DateTime<Utc>>,
    pub amount: Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateProjectExpense {
    pub on_date: Option<DateTime<Utc>>,
    pub reason: String,
    pub amount: Decimal,
}
