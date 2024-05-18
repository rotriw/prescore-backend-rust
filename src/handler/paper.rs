#![allow(non_snake_case)] //i like non-snake-case!!!

use actix_web::{get, services, web, Scope};

use crate::handler::ResultHandler;
use crate::model::exam::{get_class_info_by_paper_id, get_score_info_by_paper_id};
use crate::model::paper;

#[get("/predict/{paperId}/{score}")]
async fn get_predict(path: web::Path<(String, f64)>) -> ResultHandler<String> {
    let (paperId, score) = path.into_inner();
    let (predict, version) = paper::predict(paperId, score);
    Ok(Json! {
        "code": 0, 
        "percent": predict, 
        "version": version
    })
}

#[get("/score_info/{paperId}")]
async fn get_score_info(path: web::Path<String>) -> ResultHandler<String> {
    let paperId = path.into_inner();
    let (max, min, med, avg) = get_score_info_by_paper_id(paperId);
    Ok(JsonWithFloat!{
        "code": 0,
        "data": {
            "max": max,
            "min": min,
            "med": med,
            "avg": avg
        }
    })
}

#[get("/distribute/{paperId}/{step}")]
async fn get_distribute(path: web::Path<(String, f64)>) -> ResultHandler<String> {
    let (paperId, step) = path.into_inner();
    if step < 0.1 || step > 150.0 {
        return Ok(Json!{
            "code": -1,
            "msg": "Not allowed range."
        });
    }
    let (distribute, suffix, prefix) = paper::get_distribute(paperId, step);
    Ok(Json!{
        "code": 0,
        "data": {
            "distribute": distribute,
            "suffix": suffix,
            "prefix": prefix 
        }
    })
}

#[get("/class_info/{paperId}")]
async fn get_class_info(path: web::Path<String>) -> ResultHandler<String> {
    let paperId = path.into_inner();
    let res = get_class_info_by_paper_id(paperId);
    Ok(JsonWithFloat!{
        "code": 0,
        "data": res
    })
}

pub fn service() -> Scope {
    let services = services![
        get_predict,
        get_score_info,
        get_class_info,
        get_distribute
    ];
    web::scope("/api/paper")
        .service(services)
}
