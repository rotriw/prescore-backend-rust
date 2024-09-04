use crate::{declare::user::CreateUser, handler::ResultHandler};
use crate::model::user::upload_user_by_create_user;
use actix_web::{post, services, web, Scope};
/*
直接上传数据
*/
#[post("")]
async fn upload_user(data: web::Json<CreateUser>) -> ResultHandler<String> {
    let _ = upload_user_by_create_user(data.into_inner());
    Ok(Json! {
        "code": 0,
        "msg": "ok."
    })
}

pub fn service() -> Scope {
    let services = services![upload_user];
    web::scope("/api/upload").service(services)
}
