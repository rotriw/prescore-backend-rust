#![allow(non_snake_case)] //i like non-snake-case!!!

use std::error::Error;

use actix_web::{get, services, web, Scope};
use serde_json::json;

#[get("/predict/{examId}/{score}")]
async fn get_predict(path: web::Path<(String, String)>) -> Result<String, Box<dyn Error>>  {
    let (examId, score) = path.into_inner();
    Ok(json!{{"code": 0, "predict": 1}}.to_string())
}

pub fn service() -> Scope {
    let services = services![
        get_predict
    ];
    web::scope("/exam")
        .service(services)
}