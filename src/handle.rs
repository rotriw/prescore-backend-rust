use actix_web::{App, HttpServer};
use crate::handler;

#[actix_web::main]
pub async fn main(port: u16) -> std::io::Result<()> {
    println!("Starting server on port: {}", port);
    HttpServer::new(|| {
        App::new()
            .service(handler::exam::service())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}