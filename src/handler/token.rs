use crate::declare::user::CreateUser;
use crate::handler::ResultHandler;
use crate::model::user::{upload_user_by_create_user, upload_user_by_token};
use actix_web::{post, services, web, Scope, HttpRequest};
use serde::Deserialize;
use crate::utils::check_user_permission;
use perm_macro::perm;

#[derive(Deserialize)]
struct TokenS {
    password: String,
}

/*
原版接口是这么写的。
*/
#[post("/token")]
async fn upload_token(data: web::Json<TokenS>) -> ResultHandler<String> {
    let _ = upload_user_by_token(data.into_inner().password).await;
    Ok(Json! {
        "code": 0,
        "msg": "ok."
    })
}

/*
直接上传数据
*/
#[post("/upload")]
#[perm("discuss-view")]
async fn upload_user(data: web::Json<CreateUser>) -> ResultHandler<String> {
    let _ = upload_user_by_create_user(data.into_inner());
    Ok(Json! {
        "code": 0,
        "msg": "ok."
    })
}

pub fn service() -> Scope {
    let services = services![upload_token, upload_user];
    web::scope("/api").service(services)
}
