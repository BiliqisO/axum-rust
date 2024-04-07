  
use anyhow::Ok;
use axum::{async_trait, RequestPartsExt};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async  fn mw_require_auth(ctx: Result<Ctx>, req:Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
ctx?;
  Ok(next.run(req).await)  
}

//region: --- Ctx Extractor

#[async_trait]
impl <S:Send +Sync> FromRequestParts<S> for Ctx{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S ) -> Result<Self>{
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        //User the cookies extractor
        let Cookies = parts.extract::<Cookies>()
.await.unwrap();   
let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());   
       
       //Parse toke
       let (user_id, exp, sign) = auth_token
        .ok_or(Error::AUthFailNoAuthTokenCookie)
        .and_then(parse_token(token)); 
        Ok(Ctx::new(user_id))
    }
}
//endregion: --- Ctx Extractor
//Parse a token of format `user-[user-id].[expiration].[signature]`
//Returs (user_id, expiration, signature)

fn parse_token(token:String) -> Result<(u64, String, String)>{
    todo!()
}