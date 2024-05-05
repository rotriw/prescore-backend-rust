#![allow(non_snake_case)]
// use crate::{declare::zhixue::ZhixuePaper, schema::prescore::exam::paper_id};

// pub fn get_user_message_fromzhixue(token: String) -> Result<ZhixueUser, reqwest::Error> {
//     let client = reqwest::blocking::Client::new();
//     let res = client.post(format!("http://zhixue.com/zhixuebao/{}", "123"))
//         .header("Xtoken", token)
//         .send()?;
    
// }


// pub fn get_paper_detail(paper_id: String) -> Result<ZhixuePaper, reqwest::Error> {
//     // 貌似这个高端接口只能从智学网
// }

use std::collections::HashMap;
use std::future::Future;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::Deserialize;
use crate::declare::exam::ExamUpload;
use crate::declare::zhixue::{ZhixueExamList, ZhixuePaperCheckSheet, ZhixueReportMain};
use crate::model::exam::upload_exam_by_examupload;
use crate::model::exam_number::NewExamNumber;
use crate::DEFAULT_ZHIXUE_LINK;

#[derive(Deserialize)]
pub struct ZhixueExamNumber {
    pub absentCount: i64,
    pub clazzId: String,
    pub clazzName: String,
    pub planNumber: i64,
    pub scanCount: i64,
}

#[derive(Deserialize, Debug)]
pub struct ZhixueExamResponse {
    pub message: String,
    pub result: String,
}

// pub fn login(username: String, password: String) -> Result<String> { // only use in get token for data

// }

pub async fn upload_paper_data_future(client: Client, paper_id: String, exam_id: String, subject_id: String, subject_name: String) -> Result<(), reqwest::Error> {
    let responsed = client.get(format!("{DEFAULT_ZHIXUE_LINK}zhixuebao/report/checksheet/?examId={exam_id}&paperId={paper_id}")).send().await?;
    let res = responsed.json::<ZhixuePaperCheckSheet>().await?;
    match res.error_code {
        0 => {
            upload_exam_by_examupload(ExamUpload {
                exam_id,
                paper_id,
                user_id: res.result.clone().unwrap().current_user_id.unwrap(),
                user_score: res.result.clone().unwrap().score.unwrap(),
                diagnostic_score: None,
                standard_score: res.result.clone().unwrap().standard_score.unwrap(),
                subject_id,
                subject_name
            });
            Ok(())
        },
        _ => Ok(())
    }
}

pub async fn upload_exam_data_future(client: Client, exam_id: String) -> Result<(), reqwest::Error> {
    let responsed = client.get(format!("{DEFAULT_ZHIXUE_LINK}zhixuebao/report/exam/getReportMain?examId={exam_id}")).send().await?;
    let res = responsed.json::<ZhixueReportMain>().await?;
    match res.error_code {
        0 => {
            for item in res.result.unwrap().paper_list.unwrap() {
                upload_paper_data_future(client.clone(), item.paper_id.unwrap(), exam_id.clone(), item.subject_code.unwrap(), item.subject_name.unwrap()).await?;
            }
            Ok(())
        },
        _ => Ok(())
    }
}

pub async fn upload_datas_by_token_future(client: Client, token: String) -> Result<(), reqwest::Error> {
    let responsed = client.get(format!("{DEFAULT_ZHIXUE_LINK}zhixuebao/report/getPageExamList")).send().await?;
    let res = responsed.json::<ZhixueExamList>().await?;
    match res.error_code {
        0 => {
            for item in res.result.unwrap().exam_info_list.unwrap() {
                upload_exam_data_future(client.clone(), item.exam_id.unwrap()).await?;
            }
            Ok(())
        },
        _ => Ok(())
    }
}

pub async fn upload_datas_by_token(token: String) -> Result<(), reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert("Xtoken", token.parse().unwrap());
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .default_headers(headers)
        .build()
        .unwrap();
    upload_datas_by_token_future(client, token).await
}

pub fn get_paper_class_number(paper_id: String) -> Result<Vec<NewExamNumber>, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let mut params = HashMap::new();
    params.insert("markingPaperId", paper_id.clone());
    let _ = client.get("https://www.zhixue.com/api-cloudmarking-scan/scanImageRecord/findPaperClassList/").send();
    let res = client.post("https://www.zhixue.com/api-cloudmarking-scan/scanImageRecord/findPaperClassList/")
        .form(&params)
        .send()?;
    println!("{:?}", res.text());
    Ok(Vec::new())
    // let res = res.json::<ZhixueExamResponse>()?;
    
    // match res.result.as_str() {
    //     "success" => {
    //         let res = res.message;
    //         let res = serde_json::from_str::<Vec<ZhixueExam>>(&res).unwrap();
            // let mut result = vec![];
            // for item in res {
            //     result.push(NewExamNumber {
            //         paper_id: paper_id.clone(),
            //         class_id: item.clazzId,
            //         number: item.planNumber,
            //     });
            // }
            // Ok(result)
    //     },
    //     _ => Ok(Vec::new())
    // }
}