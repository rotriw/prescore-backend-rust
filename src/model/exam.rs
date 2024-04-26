use diesel::{Insertable, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use crate::{declare::exam::ExamUpload, model::exam_number::get_exam_number, service::postgres::DBPOOL};
use diesel::prelude::*;
use super::user::get_user_class_id_by_user_id;

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::prescore::exam)]
pub struct NewExam {
    pub user_id         : String,
    pub exam_id         : String,
    pub paper_id        : String,
    pub subject_name    : Option<String>,
    pub subject_id      : Option<String>,
    pub standard_score  : Option<f64>,
    pub user_score      : Option<f64>,
    pub diagnostic_score: Option<f64>,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::prescore::exam)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Exam {
    pub id              : i64,
    pub user_id         : String,
    pub exam_id         : String,
    pub paper_id        : String,
    pub subject_name    : Option<String>,
    pub subject_id      : Option<String>,
    pub standard_score  : Option<f64>,
    pub user_score      : Option<f64>,
    pub diagnostic_score: Option<f64>,
}

#[derive(Debug)]
pub struct ClassDataExam {
    pub id              : i64,
    pub user_id         : String,
    pub exam_id         : String,
    pub paper_id        : String,
    pub subject_name    : Option<String>,
    pub subject_id      : Option<String>,
    pub standard_score  : Option<f64>,
    pub user_score      : Option<f64>,
    pub diagnostic_score: Option<f64>,
    pub class_id        : String,

}

pub fn create_exam(
    user_id         : String,
    exam_id         : String,
    paper_id        : String,
    subject_name    : Option<String>,
    subject_id      : Option<String>,
    standard_score  : Option<f64>,
    user_score      : Option<f64>,
    diagnostic_score: Option<f64>,
) -> Option<Exam> {
    use crate::schema::prescore::exam;
    let new_exam = NewExam {
        user_id,
        exam_id,
        paper_id,
        subject_name,
        subject_id,
        standard_score,
        user_score,
        diagnostic_score,
    };
    
    let mut conn = unsafe {DBPOOL.clone().unwrap().get().unwrap()};

    diesel::insert_into(exam::table)
        .values(&new_exam)
        .returning(Exam::as_returning())
        .get_result(&mut conn)
        .ok()
}

// upload by exam_id, paper_id and user_id use diesel upload
pub fn upload_exam_unique(
    user_id         : String,
    exam_id         : String,
    paper_id        : String,
    subject_name    : Option<String>,
    subject_id      : Option<String>,
    standard_score  : Option<f64>,
    user_score      : Option<f64>,
    diagnostic_score: Option<f64>,
) -> Option<Exam> {
    use crate::schema::prescore::exam;
    use crate::schema::prescore::exam::{exam_id as eid, user_id as uid, paper_id as pid};
    
        // upload by exam_id, paper_id and user_id
    let new_exam = NewExam {
        user_id: user_id.clone(),
        exam_id: exam_id.clone(),
        paper_id: paper_id.clone(),
        subject_name,
        subject_id,
        standard_score,
        user_score,
        diagnostic_score,
    };
    let mut conn = unsafe {
        DBPOOL.clone().unwrap().get().unwrap()
    };
    diesel::update(exam::table)
        .filter(eid.eq(exam_id))
        .filter(uid.eq(user_id))
        .filter(pid.eq(paper_id))
        .set(&new_exam)
        .get_result(&mut conn)
        .ok()
}

pub fn create_exam_by_examupload(data: ExamUpload) -> Option<Exam> {
    create_exam(
        data.user_id, 
        data.exam_id, 
        data.paper_id, 
        Some(data.subject_name), 
        Some(data.subject_id), 
        Some(data.standard_score),
        Some(data.user_score), 
        data.diagnostic_score
    )
}

pub fn upload_exam_by_examupload(data: ExamUpload) -> Option<i64> { // status
    use crate::schema::prescore::exam::{exam_id, user_id, paper_id, dsl::exam};
    let mut conn = unsafe {
        DBPOOL.clone().unwrap().get().unwrap()
    };
    let res = exam.filter(exam_id.eq(data.exam_id.clone()))
        .filter(user_id.eq(data.user_id.clone()))
        .filter(paper_id.eq(data.paper_id.clone()))
        .get_result::<Exam>(&mut conn)
        .ok();
    if res.is_none() {
        create_exam_by_examupload(data);
        return Some(0);
    } else {
        upload_exam_unique(
            data.user_id, 
            data.exam_id, 
            data.paper_id, 
            Some(data.subject_name), 
            Some(data.subject_id), 
            Some(data.standard_score),
            Some(data.user_score), 
            data.diagnostic_score
        );
        return Some(1);
    }
}

pub fn get_datas_by_paper_id(paper_id: String) -> Vec<Exam> {
    use crate::schema::prescore::exam;
    let mut conn = unsafe {
        DBPOOL.clone().unwrap().get().unwrap()
    };
    exam::table
        .filter(exam::paper_id.eq(paper_id))
        .load(&mut conn)
        .expect("Error loading exams")
}

use std::{collections::HashMap/* , time::{Duration, Instant}*/};


const DEFAULT_USER: f64 = 40 as f64;

pub fn predict(paper_id: String, score: f64) -> (f64, i32) {
    // get
    let mut res:f64 = 0 as f64;
    let datas = get_datas_by_paper_id(paper_id.clone());
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

pub fn score_graph(paper_id: String, class_id: String) {

}