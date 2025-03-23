use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing, Json, Router,
};
use malta_core::{CreateEmployee, CreateProject, CreateTool, Malta};
use rust_decimal_macros::dec;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", routing::get(main_page))
        .nest_service("/styles", ServeFile::new("styles.css"))
        .nest_service("/js", ServeDir::new("js"))
        .nest_service("/teapot", ServeDir::new("teapot"))
        .route("/project", routing::get(all_projects))
        .route("/project", routing::post(new_project))
        // .route("/project/ws", routing::get(projects_stream))
        // .route("/project/{id}", routing::get(project_page))
        // .route("/project/{id}", routing::put(edit_project))
        .route("/project/{id}", routing::delete(delete_project))
        // .route("/project/{id}/ws", routing::get(project_details))
        .route("/employee", routing::get(all_employees))
        .route("/employee", routing::post(new_employee))
        // .route("/employee/ws", routing::get(employees_stream))
        // .route("/employee/{id}", routing::get(employee_page))
        // .route("/employee/{id}", routing::put(edit_employee))
        .route("/employee/{id}", routing::delete(delete_employee))
        // .route("/employee/{id}/ws", routing::get(employee_details))
        .route("/tool", routing::get(all_tools))
        .route("/tool", routing::post(new_tool))
        // .route("/tool/ws", routing::get(tools_stream))
        // .route("/tool/{id}", routing::get(tool_page))
        // .route("/tool/{id}", routing::put(edit_tool))
        .route("/tool/{id}", routing::delete(delete_tool))
        // .route("/tool/{id}/ws", routing::get(tool_details))
        .fallback(async || {
            (
                StatusCode::IM_A_TEAPOT,
                Html(include_str!("../teapot/index.html")),
            )
        })
        .with_state(Malta::open().await.unwrap());
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn main_page(State(db): State<Malta>) -> impl IntoResponse {
    Html(format!(include_str!("../templates/main.html"), 0, 0))
}

async fn all_projects(State(db): State<Malta>) -> impl IntoResponse {
    Html(format!(
        include_str!("../templates/projects.html"),
        match db.get_all_projects().await {
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
        match db.get_all_employees().await {
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
        match db.get_all_tools().await {
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

// #[axum::debug_handler]
// async fn projects_stream(ws: WebSocketUpgrade, State(db): State<Malta>) -> impl IntoResponse {
//     use std::sync::Arc;
//     use tokio::sync::Mutex;
//     ws.on_upgrade(async move |socket| {
//         let socket = Arc::new(Mutex::new(socket));
//         db.project_stream(|notif| {
//             let socket = Arc::clone(&socket);
//             async move {
//                 println!("{notif:?}");
//                 socket
//                     .lock()
//                     .await
//                     .send(Message::Text("Changed".into()))
//                     .await
//                     .unwrap();
//             }
//         })
//         .await
//     })
// }
