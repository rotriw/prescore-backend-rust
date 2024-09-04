use crate::service::zhixue::ZhixueExamNumber;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExamUpload {
    pub paper_id: String,
    pub user_score: f64,
    pub user_id: String,
    pub subject_id: String,
    pub subject_name: String,
    pub standard_score: f64,
    pub diagnostic_score: Option<f64>,
    pub exam_id: String,
}

#[derive(Deserialize)]
pub struct ScoreInfoUpload {
    pub paper_id: Vec<String>,
}

#[derive(Deserialize)]
pub struct ExamNumberUpload {
    pub paper_id: String,
    pub data: Vec<ZhixueExamNumber>,
}

#[derive(Deserialize)]
pub struct TokenUpload {
    pub token: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::prescore::exam)]
pub struct NewExam {
    pub user_id: String,
    pub exam_id: String,
    pub paper_id: String,
    pub subject_name: Option<String>,
    pub subject_id: Option<String>,
    pub standard_score: Option<f64>,
    pub user_score: Option<f64>,
    pub diagnostic_score: Option<f64>,
}

#[derive(Insertable, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::prescore::times_number)]
pub struct NewTime {
    pub tid: i64,
    pub time: i64,
}

#[derive(Debug, Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::prescore::exam)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Exam {
    pub id: i64,
    pub user_id: String,
    pub exam_id: String,
    pub paper_id: String,
    pub subject_name: Option<String>,
    pub subject_id: Option<String>,
    pub standard_score: Option<f64>,
    pub user_score: Option<f64>,
    pub diagnostic_score: Option<f64>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClassDataExam {
    pub id: i64,
    pub user_id: String,
    pub exam_id: String,
    pub paper_id: String,
    pub subject_name: Option<String>,
    pub subject_id: Option<String>,
    pub standard_score: Option<f64>,
    pub user_score: Option<f64>,
    pub diagnostic_score: Option<f64>,
    pub class_id: String,
}
