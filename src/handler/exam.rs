#![allow(non_snake_case)] //i like non-snake-case!!!


use actix_web::{get, post, services, web, Scope};

use crate::handler::ResultHandler;
use crate::declare::exam::{ExamNumberUpload, ExamUpload};
use crate::model::exam::{predict, upload_exam_by_examupload};
use crate::model::exam_number::{upload_new_exam_number, NewExamNumber};


#[get("/predict/{paperId}/{score}")]
async fn get_predict(path: web::Path<(String, f64)>) -> ResultHandler<String> {
    let (paperId, score) = path.into_inner();
    let (predict, version) = predict(paperId, score);
    Ok(Json!{"code": 0, "predict": predict, "version": version})
}


//puppter 
#[post("/submit")]
async fn upload_score(data: web::Json<ExamUpload>) -> ResultHandler<String> {
    let data = data.into_inner();
    let score = upload_exam_by_examupload(data).unwrap();
    
    Ok(Json!{
            "code": 0, 
            "msg": format!("ok. {} 1 test-paper data", {
                            match score {
                                0 => "create",
                                1 => "upload",
                                _ => "unknow",
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
        result.push(NewExamNumber {
            paper_id: data.paper_id.clone(),
            class_id: item.clazzId,
            number: item.scanCount,
        });
    }
    upload_new_exam_number(result);
    Ok(Json!{
            "code": 0, 
            "msg": "ok. thanks for upload"
        })
}


pub fn service() -> Scope {
    let services = services![
        get_predict
    ];
    web::scope("/api/exam")
        .service(services)
        .service(upload_score)
        .service(upload_exam_data)
}