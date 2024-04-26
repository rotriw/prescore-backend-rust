use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
use crate::service::postgres::DBPOOL;

#[derive(Insertable, Queryable, Clone, Debug)]
#[diesel(table_name = crate::schema::prescore::test_number)]
pub struct ExamNumber {
    pub id: i64,
    pub paper_id: String,
    pub class_id: String,
    pub number: i64,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::prescore::test_number)]
pub struct NewExamNumber {
    pub paper_id: String,
    pub class_id: String,
    pub number: i64,
}


pub fn upload_new_exam_number(new_exam_number: Vec<NewExamNumber>) -> Vec<ExamNumber> {
    use crate::schema::prescore::test_number::dsl::test_number as tn;

   // let new_exam_number = get_paper_class_number(paper_id.clone()).unwrap();
    let mut conn = unsafe {
        DBPOOL.clone().unwrap().get().unwrap()
    };
    let result = diesel::insert_into(tn)
        .values(&new_exam_number)
        .get_results::<ExamNumber>(&mut conn)
        .ok();
    match result {
        Some(r) => r,
        None => vec![],
    }
}

pub fn get_exam_number(paper_id: String) -> Vec<ExamNumber> {
    use crate::schema::prescore::test_number::dsl::test_number as tn;
    use crate::schema::prescore::test_number::paper_id as pid;
    let mut conn = unsafe {
        DBPOOL.clone().unwrap().get().unwrap()
    };
    let result = tn.filter(pid.eq(paper_id.clone()))
        .load::<ExamNumber>(&mut conn)
        .ok();
    match result {
        Some(r) => r,
        None => Vec::new(),
    }
}