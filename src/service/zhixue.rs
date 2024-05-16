#![allow(non_snake_case)]
#![allow(dead_code)]
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
use reqwest::header::HeaderMap;
use serde::Deserialize;
use reqwest::Client;
use crate::declare::exam::ExamUpload;
use crate::declare::user::CreateUser;
use crate::declare::zhixue::{ZhixueAccount, ZhixueExamList, ZhixuePaperCheckSheet, ZhixueReportMain};
use crate::model::exam::upload_exam_by_examupload;
use crate::model::exam_number::NewExamNumber;
use crate::ZHIXUE_LINK;

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
    let responsed = client.get(format!("{}zhixuebao/report/checksheet/?examId={exam_id}&paperId={paper_id}", *ZHIXUE_LINK.get())).send().await?;
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
    let responsed = client.get(format!("{}zhixuebao/report/exam/getReportMain?examId={exam_id}", *ZHIXUE_LINK.get())).send().await?;
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

pub async fn upload_datas_by_token_future(client: Client, _token: String) -> Result<(), reqwest::Error> {
    let responsed = client.get(format!("{}zhixuebao/report/getPageExamList", *ZHIXUE_LINK.get())).send().await?;
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
    //
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

pub async fn get_user_data(token: String) -> Result<CreateUser, reqwest::Error> {
    println!("233");
    let mut headers = HeaderMap::new();
    headers.insert("cookie", format!("tlsysSessionId={token}").parse().unwrap());
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .default_headers(headers)
        .build()
        .unwrap();
    let res = client.get(format!("{}/container/container/student/account/", *ZHIXUE_LINK.get())).send().await?;
    let res = res.json::<ZhixueAccount>().await?;
    println!("ddd");
    let student = res.student.unwrap();
    Ok(CreateUser {
        user_id: Some(student.id.clone()),
        name: None, // 我真不上传。所以谁保护隐私？
        login_name: Some(student.login_name),
        school_id: Some(student.clazz.school.id),
        school_name: Some(student.clazz.school.name),
        division_id: Some(student.clazz.division.id),
        division_name: Some(student.clazz.division.name),
        class_id: Some(student.clazz.id),
        class_name: Some(student.clazz.name),
        child_id: Some(student.id), // 我不是很懂这个和user_id的区别
    })
}
