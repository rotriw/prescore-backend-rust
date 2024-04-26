use serde::Deserialize;

use crate::service::zhixue::ZhixueExamNumber;

#[derive(Deserialize)]
pub struct ExamUpload {
    pub paper_id: String,
    pub user_score: f64,
    pub user_id: String,
    pub subject_id: String,
    pub subject_name: String,
    pub standard_score: f64,
    pub diagnostic_score: Option<f64>,
    pub exam_id: String
}

#[derive(Deserialize)]
pub struct ExamNumberUpload {
    pub paper_id: String,
    pub data: Vec<ZhixueExamNumber>
}