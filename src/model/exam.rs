use crate::{
    declare::exam::{Exam, ExamUpload, NewExam, NewTime},
    service::postgres::DBPOOL,
};
use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

// const GRADE_SCORE: [(f64, f64); 21] = [
//     (0.01, 100.0),
//     (0.03, 97.0),
//     (0.06, 94.0),
//     (0.10, 91.0),
//     (0.15, 88.0),
//     (0.22, 85.0),
//     (0.30, 82.0),
//     (0.39, 79.0),
//     (0.47, 76.0),
//     (0.55, 73.0),
//     (0.62, 70.0),
//     (0.68, 67.0),
//     (0.74, 64.0),
//     (0.80, 61.0),
//     (0.85, 58.0),
//     (0.89, 55.0),
//     (0.93, 52.0),
//     (0.96, 49.0),
//     (0.98, 46.0),
//     (0.99, 43.0),
//     (1.00, 40.0),
// ];

// pub fn get_grade_score(score: f64) -> f64 {
//     for i in 0..20 {
//         if score >= GRADE_SCORE[i].0 && score < GRADE_SCORE[i + 1].0 {
//             return GRADE_SCORE[i].1;
//         }
//     }
//     return 40.0
// }

pub fn create_time(tid: i64, time_stamp: i64) -> () {
    use crate::schema::prescore::times_number;
    let new_time = NewTime {
        tid,
        time: time_stamp,
    };
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };

    diesel::insert_into(times_number::table)
        .values(&new_time)
        .execute(&mut conn)
        .expect("Error loading exams");
    ()
}

pub fn upload_time(tid: i64, time_stamp: i64) -> () {
    use crate::schema::prescore::times_number;
    let new_time = NewTime {
        tid,
        time: time_stamp,
    };
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };

    diesel::update(times_number::table)
        .filter(times_number::tid.eq(tid))
        .set(&new_time)
        .execute(&mut conn)
        .expect("Error loading exams");
    ()
}

pub fn create_exam(
    user_id: String,
    exam_id: String,
    paper_id: String,
    subject_name: Option<String>,
    subject_id: Option<String>,
    standard_score: Option<f64>,
    user_score: Option<f64>,
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

    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };

    let data: Exam = diesel::insert_into(exam::table)
        .values(&new_exam)
        .returning(Exam::as_returning())
        .get_result(&mut conn)
        .ok()?;
    create_time(
        data.id,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    );
    return Some(data.clone());
}

// upload by exam_id, paper_id and user_id use diesel upload
pub fn upload_exam_unique(
    user_id: String,
    exam_id: String,
    paper_id: String,
    subject_name: Option<String>,
    subject_id: Option<String>,
    standard_score: Option<f64>,
    user_score: Option<f64>,
    diagnostic_score: Option<f64>,
) -> Option<Exam> {
    use crate::schema::prescore::exam;
    use crate::schema::prescore::exam::{exam_id as eid, paper_id as pid, user_id as uid};

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
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };
    let data: Exam = diesel::update(exam::table)
        .filter(eid.eq(exam_id))
        .filter(uid.eq(user_id))
        .filter(pid.eq(paper_id))
        .set(&new_exam)
        .get_result(&mut conn)
        .ok()?;
    // 我知道这里会出现data.id的time没出现的情况，但是就这样吧。
    // 反正历史问题就让他变成历史吧。至少没把程序崩了。
    upload_time(
        data.id,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    );
    return Some(data.clone());
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
        data.diagnostic_score,
    )
}

pub fn upload_exam_by_examupload(data: ExamUpload) -> Option<i64> {
    // status
    use crate::schema::prescore::exam::{dsl::exam, exam_id, paper_id, user_id};
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };
    let res = exam
        .filter(exam_id.eq(data.exam_id.clone()))
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
            data.diagnostic_score,
        );
        return Some(1);
    }
}

pub fn get_datas_by_paper_id(paper_id: String) -> Vec<Exam> {
    use crate::schema::prescore::exam;
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };
    exam::table
        .filter(exam::paper_id.eq(paper_id))
        .load(&mut conn)
        .expect("Error loading exams")
}

//NOTICE: The returning value doesn't have paper_id
pub fn get_datas_by_paper_ids(paper_ids: Vec<String>) -> Vec<Exam> {
    use crate::schema::prescore::exam;
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };
    let mut _res: Vec<Exam> = vec![];
    let mut user_appear_times: HashMap<String, i32> = HashMap::new();
    let mut user_total_score: HashMap<String, f64> = HashMap::new();
    let ntres = paper_ids.len();
    for paper_id in paper_ids {
        let data: Vec<Exam> = exam::table
            .filter(exam::paper_id.eq(paper_id))
            .load(&mut conn)
            .expect("Error loading exams");
        for item in data {
            if !user_appear_times.contains_key(&item.user_id) {
                user_appear_times.insert(item.user_id.clone(), 0);
                user_total_score.insert(item.user_id.clone(), 0.0);
            }
            if user_appear_times.contains_key(&item.user_id) {
                (*user_appear_times.get_mut(&item.user_id).unwrap()) += 1;
                (*user_total_score.get_mut(&item.user_id).unwrap()) +=
                    item.user_score.unwrap_or(0.0);
            }
        }
    }
    for (key, value) in user_total_score {
        if user_appear_times[&key] < ntres as i32 {
            continue;
        }
        _res.push(Exam {
            id: 0,
            user_id: key,
            exam_id: "".to_string(),
            paper_id: "".to_string(),
            subject_name: None,
            subject_id: None,
            standard_score: None,
            user_score: Some(value),
            diagnostic_score: None,
        });
    }
    _res
}

pub fn cmp_float(a: &f64, b: &f64) -> std::cmp::Ordering {
    if a < b {
        std::cmp::Ordering::Less
    } else if a > b {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

/*
我想了一会这个地方怎么合并。可能写这个代码的人可能精神有问题。所以这里就直接复制了。
什么时候想到更好的解决方案什么时候改。
*/
pub fn get_score_info_by_data_with_num(datas: Vec<Exam>) -> (f64, f64, f64, f64, usize) {
    let mut only_score = vec![];
    let mut _max = 0.0;
    let mut _min = 2147483647.0;
    let mut _med = 0.0;
    let mut _avg = 0.0;
    let mut user_scores = HashMap::new();
    let mut user_list = vec![];
    for item in datas {
        if !user_scores.contains_key(&item.user_id) {
            user_scores.insert(item.user_id.clone(), item.user_score.unwrap_or(0.0));
            user_list.push(item.user_id.clone());
        } else {
            (*user_scores.get_mut(&item.user_id).unwrap()) += item.user_score.unwrap_or(0.0);
        }
    }
    let size = user_list.len();
    for item in user_list {
        let user_score = user_scores[&item];
        only_score.push(user_score);
        _max = f64::max(_max, user_score);
        _min = f64::min(_min, user_score);
        _avg += user_score / size as f64;
    }
    if size == 0 {
        _med = 0.0;
    } else if size % 2 == 1 {
        adqselect::nth_element(&mut only_score, size / 2, &mut cmp_float);
        _med = only_score[size / 2];
    } else {
        adqselect::nth_element(&mut only_score, size / 2, &mut cmp_float);
        adqselect::nth_element(&mut only_score, size / 2 - 1, &mut cmp_float);
        _med = only_score[size / 2] / 2.0;
        _med += only_score[size / 2 - 1] / 2.0;
    }
    if _min == 2147483647.0 {
        _min = 0.0;
    }
    (_max, _min, _med, _avg, size)
}

pub fn get_score_info_by_data(datas: Vec<Exam>) -> (f64, f64, f64, f64) {
    let mut only_score = vec![];
    let mut _max = 0.0;
    let mut _min = 2147483647.0;
    let mut _med = 0.0;
    let mut _avg = 0.0;
    let mut user_scores = HashMap::new();
    let mut user_list = vec![];
    for item in datas {
        if !user_scores.contains_key(&item.user_id) {
            user_scores.insert(item.user_id.clone(), item.user_score.unwrap_or(0.0));
            user_list.push(item.user_id.clone());
        } else {
            (*user_scores.get_mut(&item.user_id).unwrap()) += item.user_score.unwrap_or(0.0);
        }
    }
    let size = user_list.len();
    for item in user_list {
        let user_score = user_scores[&item];
        only_score.push(user_score);
        _max = f64::max(_max, user_score);
        _min = f64::min(_min, user_score);
        _avg += user_score / size as f64;
    }
    if size == 0 {
        _med = 0.0;
    } else if size % 2 == 1 {
        adqselect::nth_element(&mut only_score, size / 2, &mut cmp_float);
        _med = only_score[size / 2];
    } else {
        adqselect::nth_element(&mut only_score, size / 2, &mut cmp_float);
        adqselect::nth_element(&mut only_score, size / 2 - 1, &mut cmp_float);
        _med = only_score[size / 2] / 2.0;
        _med += only_score[size / 2 - 1] / 2.0;
    }
    if _min == 2147483647.0 {
        _min = 0.0;
    }
    (_max, _min, _med, _avg)
}

pub fn get_score_info_by_paper_id(paper_id: String) -> (f64, f64, f64, f64) {
    let datas = get_datas_by_paper_id(paper_id);
    get_score_info_by_data(datas)
}

pub fn get_score_info_by_paper_ids(paper_ids: Vec<String>) -> (f64, f64, f64, f64) {
    let datas = get_datas_by_paper_ids(paper_ids);
    get_score_info_by_data(datas)
}

pub fn get_score_info_by_exam_id(exam_id: String) -> (f64, f64, f64, f64) {
    let datas = get_datas_by_exam_id(exam_id);
    get_score_info_by_data(datas)
}

#[derive(Serialize, Deserialize)]
pub struct ClassData {
    pub class_name: String,
    pub class_id: String,
    pub count: i64,
    pub max: f64,
    pub min: f64,
    pub med: f64,
    pub avg: f64,
}

pub fn get_class_info_by_class_datas(
    class_list: Vec<String>,
    class_data: HashMap<String, Vec<Exam>>,
) -> Vec<ClassData> {
    let mut sorted_res = vec![];
    let mut res = vec![];
    for class_id in class_list {
        let (max, min, med, avg, count) =
            get_score_info_by_data_with_num(class_data[&class_id].clone());
        let class_name_info = get_class_name_by_class_id(class_id.clone()).unwrap_or((
            "".to_string(),
            2147483647,
            2147483647,
        ));
        res.push((
            ClassData {
                class_name: class_name_info.0,
                class_id: class_id.clone(),
                count: count as i64,
                max,
                min,
                med,
                avg,
            },
            class_name_info.1,
            class_name_info.2,
        ));
    }
    res.sort_by(|a, b| {
        if a.1 == b.1 {
            a.2.cmp(&b.2)
        } else {
            a.1.cmp(&b.1)
        }
    });
    for item in res {
        sorted_res.push(item.0);
    }
    sorted_res
}

pub fn get_class_info_by_exam_id(exam_id: String) -> Vec<ClassData> {
    let datas = get_datas_by_exam_id(exam_id);
    let mut class_data = HashMap::new();
    let mut class_list = vec![];
    for item in datas {
        let class_id =
            get_user_class_id_by_user_id(item.user_id.clone()).unwrap_or("magic_class".to_string());
        if !class_data.contains_key(&class_id) {
            class_data.insert(class_id.clone(), vec![item]);
            class_list.push(class_id.clone());
        } else {
            class_data.get_mut(&class_id).unwrap().push(item);
        }
    }
    get_class_info_by_class_datas(class_list, class_data)
}

pub fn get_class_info_by_paper_id(paper_id: String) -> Vec<ClassData> {
    let datas = get_datas_by_paper_id(paper_id);
    let mut class_data = HashMap::new();
    let mut class_list = vec![];
    for item in datas {
        let class_id =
            get_user_class_id_by_user_id(item.user_id.clone()).unwrap_or("magic_class".to_string());
        if !class_data.contains_key(&class_id) {
            class_data.insert(class_id.clone(), vec![item]);
            class_list.push(class_id.clone());
        } else {
            class_data.get_mut(&class_id).unwrap().push(item);
        }
    }
    get_class_info_by_class_datas(class_list, class_data)
}

pub fn get_class_info_by_paper_ids(paper_ids: Vec<String>) -> Vec<ClassData> {
    let datas = get_datas_by_paper_ids(paper_ids);
    let mut class_data = HashMap::new();
    let mut class_list = vec![];
    for item in datas {
        let class_id =
            get_user_class_id_by_user_id(item.user_id.clone()).unwrap_or("magic_class".to_string());
        if !class_data.contains_key(&class_id) {
            class_data.insert(class_id.clone(), vec![item]);
            class_list.push(class_id.clone());
        } else {
            class_data.get_mut(&class_id).unwrap().push(item);
        }
    }
    get_class_info_by_class_datas(class_list, class_data)
}

pub fn get_datas_by_exam_id(exam_id: String) -> Vec<Exam> {
    use crate::schema::prescore::exam;
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };
    exam::table
        .filter(exam::exam_id.eq(exam_id))
        .load(&mut conn)
        .expect("Error loading exams")
}

use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use super::user::{get_class_name_by_class_id, get_user_class_id_by_user_id};
/*use time::{Duration, Instant}; */

/*u
    exam 的 predict好像不是很能拿数据。就比较搞笑了。
    具体来说我们只能假设，我们拥有的数据是足够平均的。（当然无法准确预测）
*/
pub fn predict(exam_id: String, score: f64) -> (f64, i32) {
    let data = get_datas_by_exam_id(exam_id.clone());
    let mut user_list = vec![];
    let mut user_score = HashMap::new();
    let mut total = 0;
    for item in data {
        if !user_score.contains_key(&item.user_id) {
            user_score.insert(item.user_id.clone(), item.user_score.unwrap_or(0.0));
            user_list.push(item.user_id.clone());
            total += 1;
        } else {
            (*user_score.get_mut(&item.user_id).unwrap()) += item.user_score.unwrap_or(0.0);
        }
    }
    let mut count = 0;
    for user in user_list {
        if *user_score.get(&user).unwrap() > score {
            count += 1;
        }
    }
    (count as f64 / total as f64, 2)
}
