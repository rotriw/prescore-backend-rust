use std::collections::HashMap;
use crate::declare::exam::{ClassDataExam, Exam};
use super::{exam::get_datas_by_paper_id, exam_number::get_exam_number, user::get_user_class_id_by_user_id};


const DEFAULT_USER: f64 = 40 as f64;

pub fn predict_with_data(datas: Vec<Exam>, paper_id: String, score: f64) -> (f64, i32) {
    let mut res:f64 = 0 as f64;
    let class_number = get_exam_number(paper_id.clone());
    let mut data_group: HashMap<String, Vec<ClassDataExam>> = HashMap::new();
    let mut class_ids = vec![];
    for data in datas {
        let class_id = get_user_class_id_by_user_id(data.user_id.clone()).unwrap_or("magic_class".to_string());
        let class_data = ClassDataExam {
            id: data.id,
            user_id: data.user_id,
            exam_id: data.exam_id,
            paper_id: data.paper_id,
            subject_name: data.subject_name,
            subject_id: data.subject_id,
            standard_score: data.standard_score,
            user_score: data.user_score,
            diagnostic_score: data.diagnostic_score,
            class_id: class_id.clone(),
        };
        if data_group.contains_key(&class_id) {
            data_group.get_mut(&class_id).unwrap().push(class_data);
        } else {
            data_group.insert(class_id.clone(), vec![class_data]);
            class_ids.push(class_id);
        }
    }
    
    let mut total_user = 0 as f64;
    let mut new_version_flag = false;
    for class_id in class_ids.clone() {
        let mut class_value = DEFAULT_USER;// class 权重 get from class_number default:40/class
        for class_numberd in class_number.clone() {
            if class_numberd.class_id == class_id {
                class_value = class_numberd.number as f64;
                new_version_flag = true;
                break;
            }

        }   
        if class_value == 0.0 {
            class_value = DEFAULT_USER; // 忽略错误上传数据。或者明显错误的，同时影响程序运行的错误。
        }
        if class_id == "magic_class" {
            class_value = 0 as f64;
        }
        total_user += class_value;
    }
    for class_id in class_ids {
        let mut sum = 0 as f64;
        let mut count = 0 as f64;
        for data in data_group.get(&class_id).unwrap() {
            if data.user_score.unwrap_or(0 as f64) > score {
                count += 1 as f64;
            }
            sum += 1 as f64;
        }
        let avg = count / sum;
        let mut class_value = DEFAULT_USER;// class 权重 get from class_number default:40/class
        for class_numberd in class_number.clone() {
            if class_numberd.class_id == class_id {
                class_value = class_numberd.number.clone() as f64;
                break;
            }

        }
        if class_id == "magic_class" {
            class_value = 0 as f64;
        }
    
        res += avg * (class_value / total_user);
    }


    (res, {
        if new_version_flag {
            3
        } else {
            2
        }
    })
}

pub fn predict(paper_id: String, score: f64) -> (f64, i32) {
    let datas = get_datas_by_paper_id(paper_id.clone());
    predict_with_data(datas, paper_id, score)
}