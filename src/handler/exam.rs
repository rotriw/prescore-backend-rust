#![allow(non_snake_case)] //i like non-snake-case!!!

// use std::thread;

use crate::declare::exam::{ExamNumberUpload, ExamUpload, TokenUpload};
use crate::handler::ResultHandler;
use crate::model::exam::{
    get_class_info_by_exam_id, get_score_info_by_exam_id, predict, upload_exam_by_examupload,
};
use crate::model::exam_number::{upload_new_exam_number, NewExamNumber};
use crate::service::zhixue::upload_datas_by_token;
use actix_web::{get, post, services, web, Scope};

#[get("/predict/{examId}/{score}")]
async fn get_predict(path: web::Path<(String, f64)>) -> ResultHandler<String> {
    let (examId, score) = path.into_inner();
    let (predict, version) = predict(examId, score);
    Ok(Json! {
        "code": 0,
        "percent": predict,
        "version": version
    })
}

#[get("/score_info/{examId}")]
async fn get_score_info(path: web::Path<String>) -> ResultHandler<String> {
    let examId = path.into_inner();
    let (max, min, med, avg) = get_score_info_by_exam_id(examId);
    Ok(JsonWithFloat! {
        "code": 0,
        "data": {
            "max": max,
            "min": min,
            "med": med,
            "avg": avg
        }
    })
}

#[get("/class_info/{examId}")]
async fn get_class_info(path: web::Path<String>) -> ResultHandler<String> {
    let examId = path.into_inner();
    let res = get_class_info_by_exam_id(examId);
    Ok(JsonWithFloat! {
        "code": 0,
        "data": res
    })
}

// #[get("/predict/{examId}/{score}/grade")]
// async fn get_predict_grade(path: web::Path<(String, f64)>) -> ResultHandler<String> {
//     let (examId, score) = path.into_inner();
//     let (predict, version) = predict_with_grade(examId, score);
//     println!("done");
//     Ok(Json! {
//         "code": 0,
//         "predict": predict, "version": version
//     })
// }

//puppter
#[post("/submit")]
async fn upload_score(data: web::Json<ExamUpload>) -> ResultHandler<String> {
    let data = data.into_inner();
    let score = upload_exam_by_examupload(data).unwrap();
    Ok(Json! {
        "code": 0,
        "msg": format!("ok. {} 1 test-paper data", {
            match score {
                0 => "create",
                1 => "upload",
                _ => "unknown",
            }
        })
    })
}

// 获取单次考试信息
#[post("/submit/exam_data")]
async fn upload_exam_data(data: web::Json<ExamNumberUpload>) -> ResultHandler<String> {
    let data = data.into_inner();
    let mut result = vec![];
    for item in data.data {
        if item.scanCount > 0 {
            result.push(NewExamNumber {
                paper_id: data.paper_id.clone(),
                class_id: item.clazzId,
                number: item.scanCount,
            });
        }
    }
    upload_new_exam_number(result);
    Ok(Json! {
        "code": 0,
        "msg": "ok. thanks for upload"
    })
}

#[post("/submit/token")]
async fn upload_exam_data_by_token(data: web::Json<TokenUpload>) -> ResultHandler<String> {
    let data = data.into_inner();
    let _ = upload_datas_by_token(data.token).await;
    Ok(Json! {
        "code": 0,
        "msg": "ok. thanks for upload (high authorize mode supported.)"
    })
}

pub fn service() -> Scope {
    let services = services![
        get_predict,
        get_class_info,
        get_score_info,
        upload_score,
        upload_exam_data_by_token,
        upload_exam_data
    ];
    web::scope("/api/exam").service(services)
}
