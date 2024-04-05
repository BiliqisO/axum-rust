#![allow(unused)]
use std::{net::{Ipv4Addr, SocketAddr}, path::Path};
use tower_cookies::CookieManagerLayer;
pub use self::error::{Error, Result};
 use axum::{extract::Query, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router};

mod error;
mod model;
mod web;
#[tokio::main] 
async fn main() {

    let mc = ModelController::new().await?;
let routes_all = Router::new()
.merge(routes_hello())
.merge(web::routes_login::routes()
    .nest("/api", web::routes_tickets::routes(mc.clone()))
.layer(middleware::map_response(main_response_mapper))
.layer(CookieManagerLayer::new())
).fallback_service(routes_static());
//region: ---Start Server
// let ip = Ipv4Addr::new(192, 168, 8, 1); 
let addr = SocketAddr::from(([127, 0, 0, 1], 8080));


println!("->> LISTENING on {addr}\n"); 
axum::Server::bind(&addr).serve(routes_all.into_make_service())
    .await.unwrap();
//endregion: ---Start Server
}
async fn main_response_mapper(res:Response) -> Response {
      println!("->>{:<12} - main_response_mapper", "RES_MAPPER");

      println!();
      res
}
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
//region: ---  Routes Hello
fn routes_hello() -> Router{
    Router::new().route("/hello", get(handler_hello)).route("/hello2/:name", get(handler_hello2))
}

use serde::{Serialize, Deserialize};
use tower_http::services::ServeDir; 
#[derive(Debug, Deserialize)]
struct HelloParams{
    name:Option<String >

}
async fn handler_hello(Query(params):Query<HelloParams>) -> impl IntoResponse {
    println!("->>{:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");  
    {Html("Hello <strong>{name}</strong>")}
}
async fn handler_hello2((name): String) -> impl IntoResponse {
    println!("->>{:<12} - handler_hello2 - {name:?}", "HANDLER");
  
    Html(format!("Hello2 <strong>{name}</strong>"))
}

//endregion: ---  Handler Hello
