use actix_web::{error, web, App, HttpResponse, HttpServer};
use crate::handler;

#[actix_web::main]
pub async fn main(port: u16) -> std::io::Result<()> {
    println!("Starting server on port: {}", port);
    HttpServer::new(|| {
        App::new()
        .service(handler::paper::service())
        .service(handler::exam::service())
        .service(handler::img::service())
        .service(handler::token::service())
        .app_data(web::JsonConfig::default().error_handler(|err, _req| {
            error::InternalError::from_response(
                "",
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(Json!{"code": -1, "msg": err.to_string()})
            ).into()
        }))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
