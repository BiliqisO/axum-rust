// use anyhow::Ok;
use axum::{routing::post, Json, Router};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
// use httpc_test::Cookie;
use serde::Deserialize;
use serde_json::{json, Value};
use crate::web; 

use crate::{Error, Result};

pub fn routes() -> Router{
    Router::new().route("/api/login", post(api_login))
} 
 

 async fn api_login(cookies: Cookies,  payload:Json<LoginPayload>) -> Result<Json<Value>>{
    if payload.username != "demo1" || payload.pwd != "welcome"{
        return Err(Error::LoginFail);
    }
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    //success body
    let body = Json(json!({"result": {
        "success": true
    }}));
   Ok(body)
 }
 
 #[derive(Debug, Deserialize)]

 struct LoginPayload{
username:String,
pwd:String,}