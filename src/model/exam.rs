use diesel::{prelude::Insertable, Queryable, RunQueryDsl, Selectable, SelectableHelper};

use crate::service::postgres::DBCONN;


#[derive(Insertable)]
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

#[derive(Debug, Queryable, Selectable)]
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

pub fn create_exam(user_id: String, exam_id: String, paper_id: String, subject_name: Option<String>, subject_id: Option<String>, standard_score: Option<f64>, user_score: Option<f64>, diagnostic_score: Option<f64>) -> Option<Exam> {
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
    unsafe {
        return diesel::insert_into(exam::table).values(&new_exam).returning(Exam::as_returning()).get_result(DBCONN.as_mut().unwrap()).ok();
    }
}