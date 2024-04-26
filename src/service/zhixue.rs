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
use serde::Deserialize;
use crate::model::exam_number::NewExamNumber;

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