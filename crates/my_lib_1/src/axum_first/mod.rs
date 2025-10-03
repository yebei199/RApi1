use axum::{
    http::StatusCode, routing::{get, post},
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::process;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

/// hh
pub fn hh() {
    println!("there is hh function\n");
}

pub async fn run() {
    // 初始化 tracing 日志订阅器，设置日志级别
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        // 设置固定的日志级别，而不是依赖环境变量
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        // 记录所有请求和响应信息
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE) // 不打印 new/enter/exit/close
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .layer(
            TraceLayer::new_for_http().make_span_with(
                tower_http::trace::DefaultMakeSpan::new()
                    .level(tracing::Level::INFO),
            ),
        );

    let listener = match tokio::net::TcpListener::bind(
        "0.0.0.0:3000",
    )
    .await
    {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!(
                "Failed to bind to address 0.0.0.0:3000: {}",
                e
            );
            process::exit(1);
        }
    };

    println!("Server listening on http://0.0.0.0:3000");

    match axum::serve(listener, app).await {
        Ok(_) => println!("Server stopped normally"),
        Err(e) => {
            eprintln!("Server error: {}", e);
            process::exit(1);
        }
    }
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: payload.id,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    id: u64,
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
