use crate::handler::ResultHandler;
use crate::model::user::upload_user_by_token;
use actix_web::{post, services, web, Scope};
use serde::Deserialize;


#[derive(Deserialize)]
#[allow(dead_code)]
struct TokenS {
    username: String,
    password: String,
}

/*
原版接口是这么写的。
*/
#[post("")]
async fn upload_token(data: web::Form<TokenS>) -> ResultHandler<String> {
    let _ = upload_user_by_token((&data.password).clone()).await;
    Ok(Json! {
        "code": 0,
        "msg": "ok."
    })
}

pub fn service() -> Scope {
    let services = services![upload_token];
    web::scope("/api/token").service(services)
}
