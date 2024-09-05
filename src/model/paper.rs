use super::{
    exam::get_datas_by_paper_id,
    exam_number::{get_exam_number, ExamNumber},
    user::get_user_class_id_by_user_id,
};
use crate::declare::{
    exam::{ClassDataExam, Exam},
    paper::PaperDistribute,
};
use std::collections::HashMap;

const DEFAULT_USER: f64 = 40 as f64;

pub fn get_distribute_with_data_class_list(
    datas: Vec<Exam>,
    step: f64,
    class_number: Vec<ExamNumber>) -> (
    Vec<PaperDistribute>,
    Vec<PaperDistribute>,
    Vec<PaperDistribute>
) {
    let mut distribute: Vec<PaperDistribute> = vec![];
    let mut suffix: Vec<PaperDistribute> = vec![];
    let mut prefix: Vec<PaperDistribute> = vec![];
    let mut score_list: Vec<(f64, i32)> = vec![];
    let data = get_distribute_total_with_class_number(datas, class_number);
    // from total data get distribute suffix prefix
    for item in data {
        score_list.push((item.score, item.sum));
    }
    let mut nscore = 0 as f64;
    let mut ncnt = 0;
    for score in score_list {
        while score.0 > nscore {
            distribute.push(PaperDistribute {
                score: nscore,
                sum: ncnt,
            });
            ncnt = 0;
            nscore += step;
        }
        ncnt += score.1 as i32;
    }
    distribute.push(PaperDistribute {
        score: nscore,
        sum: ncnt,
    });
    let mut cnt = 0;
    for data in distribute.clone() {
        cnt += data.sum;
        suffix.push(PaperDistribute {
            score: data.score,
            sum: cnt,
        });
    }
    let mut cnt = 0;
    for data in distribute.iter().rev() {
        cnt += data.sum;
        prefix.push(PaperDistribute {
            score: data.score,
            sum: cnt,
        });
    }
    (distribute, prefix, suffix)
}

pub fn get_distribute_with_data(
    datas: Vec<Exam>,
    step: f64,
) -> (
    Vec<PaperDistribute>,
    Vec<PaperDistribute>,
    Vec<PaperDistribute>,
) {
    let mut distribute: Vec<PaperDistribute> = vec![];
    let mut suffix: Vec<PaperDistribute> = vec![];
    let mut prefix: Vec<PaperDistribute> = vec![];
    let mut score_list: Vec<f64> = vec![];
    for data in datas {
        score_list.push(data.user_score.unwrap_or(0 as f64));
    }
    score_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut nscore = 0 as f64;
    let mut ncnt = 0;
    for score in score_list {
        while score > nscore {
            distribute.push(PaperDistribute {
                score: nscore,
                sum: ncnt,
            });
            ncnt = 0;
            nscore += step;
        }
        ncnt += 1;
    }
    distribute.push(PaperDistribute {
        score: nscore,
        sum: ncnt,
    });
    let mut cnt = 0;
    for data in distribute.clone() {
        cnt += data.sum;
        suffix.push(PaperDistribute {
            score: data.score,
            sum: cnt,
        });
    }
    let mut cnt = 0;
    for data in distribute.iter().rev() {
        cnt += data.sum;
        prefix.push(PaperDistribute {
            score: data.score,
            sum: cnt,
        });
    }
    (distribute, prefix, suffix)
}

pub fn chore_data_into_class_list(datas: Vec<Exam>, class_number: Vec<ExamNumber>) -> (HashMap<String, Vec<ClassDataExam>>, Vec<String>, f64, bool) {
    let mut data_group: HashMap<String, Vec<ClassDataExam>> = HashMap::new();
    let mut class_ids = vec![];
    let mut new_version_flag = false;
    let mut total_user = 0 as f64;
    for data in datas {
        let class_id =
            get_user_class_id_by_user_id(data.user_id.clone()).unwrap_or("magic_class".to_string());
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
    for class_id in class_ids.clone() {
        let mut class_value = DEFAULT_USER; // class 权重 get from class_number default:40/class
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
    (data_group, class_ids, total_user, new_version_flag)
}

pub fn get_distribute_total_with_class_number(datas: Vec<Exam>, class_number: Vec<ExamNumber>) -> Vec<PaperDistribute> {
    let (data, class, total, _) = chore_data_into_class_list(datas, class_number.clone());
    let mut score_list: Vec<(f64, f64)> = vec![]; // score, value.
    let mut result = vec![];
    for class_id in class {
        if class_id == "magic_class" {
            continue;
        }
        let mut class_actual_number = DEFAULT_USER;
        for class_data in class_number.clone() {
            if class_data.class_id == class_id {
                class_actual_number = class_data.number as f64;
                break;
            }
        }
        let value = class_actual_number / total / (data[&class_id].len() as f64);
        for data in data.get(&class_id).unwrap() {
            score_list.push((data.user_score.unwrap_or(0 as f64), value));
        }
    }
    score_list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    // merge score are same
    let mut unique_score_list = vec![];
    let mut last_score = 0 as f64;
    let mut last_value = 0 as f64;
    for item in score_list {
        if item.0 == last_score {
            last_value += item.1;
        } else {
            unique_score_list.push((last_score, last_value));
            last_score = item.0;
            last_value = item.1;
        }
    }
    unique_score_list.push((last_score, last_value));
    for item in unique_score_list {
        result.push(PaperDistribute {
            score: item.0,
            sum: (total * item.1).ceil() as i32,
        });
    }
    result
}

pub fn predict_with_data_with_class_number(
    datas: Vec<Exam>,
    score: f64,
    class_number: Vec<ExamNumber>,
) -> (f64, i32) {
    let mut res: f64 = 0 as f64;
    let (data_group, class_ids, total_user, new_version_flag) = chore_data_into_class_list(datas, class_number.clone());
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
        let mut class_value = DEFAULT_USER; // class 权重 get from class_number default:40/class
        for class_numberd in class_number.clone() {
            if class_numberd.class_id == class_id {
                class_value = class_numberd.number.clone() as f64;
                break;
            }
        }
        if class_id == "magic_class" {
            class_value = 0 as f64;
        }
        res += avg * (class_value / (total_user as f64));
    }

    (res, if new_version_flag {
        3
    } else {
        2
    })
}

pub fn predict_with_data(datas: Vec<Exam>, paper_id: String, score: f64) -> (f64, i32) {
    let class_number = get_exam_number(paper_id.clone());
    predict_with_data_with_class_number(datas, score, class_number)
}

pub fn predict(paper_id: String, score: f64) -> (f64, i32) {
    let datas = get_datas_by_paper_id(paper_id.clone());
    predict_with_data(datas, paper_id, score)
}

// deprecated
#[allow(dead_code)]
pub fn get_distribute(
    paper_id: String,
    step: f64,
) -> (
    Vec<PaperDistribute>,
    Vec<PaperDistribute>,
    Vec<PaperDistribute>,
) {
    let datas = get_datas_by_paper_id(paper_id.clone());
    get_distribute_with_data(datas, step)
}
