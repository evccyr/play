use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, method_routing::get_service, Router},
};
use model::ModelController;
use serde::Deserialize;

use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod ctx;
mod web;
use self::error::{Error, Result};

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    dbg!(">>>>>> HELLO HANDLER");
    dbg!("Params: {}", &params);
    Html(format!("Hello, {}!", params.name.as_deref().unwrap()))
}

async fn hello2(Path(name): Path<String>) -> impl IntoResponse {
    dbg!(">>>>>> HELLO2 HANDLER");
    dbg!("Name: {}", &name);
    Html(format!("Hello, {}!", name))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/hello2/:name", get(hello2))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Model Controller
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        // .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    dbg!("Response Mapper");
    res
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
