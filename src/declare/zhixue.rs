#![allow(dead_code)]
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaperList {
	#[serde(rename = "beSubSubject")]
	pub be_sub_subject: Option<bool>,

	#[serde(rename = "beVipExp")]
	pub be_vip_exp: Option<bool>,

	#[serde(rename = "isSinglePublish")]
	pub is_single_publish: Option<bool>,

	#[serde(rename = "paperId")]
	pub paper_id: Option<String>,

	#[serde(rename = "paperName")]
	pub paper_name: Option<String>,

	#[serde(rename = "scoringModel")]
	pub scoring_model: Option<i32>,

	#[serde(rename = "standardScore")]
	pub standard_score: Option<f64>,

	#[serde(rename = "subjectCode")]
	pub subject_code: Option<String>,

	#[serde(rename = "subjectName")]
	pub subject_name: Option<String>,

	#[serde(rename = "title")]
	pub title: Option<String>,

	#[serde(rename = "userScore")]
	pub user_score: Option<f64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZhixueReportMainResultPaper {
	#[serde(rename = "examTypeCode")]
	pub exam_type_code: Option<String>,

	#[serde(rename = "paperList")]
	pub paper_list: Option<Vec<PaperList>>,

	#[serde(rename = "showAnalysis")]
	pub show_analysis: Option<bool>,

	#[serde(rename = "showDownload")]
	pub show_download: Option<bool>,

	#[serde(rename = "showScoreLv")]
	pub show_score_lv: Option<bool>,

	#[serde(rename = "title")]
	pub title: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZhixueReportMain {
	#[serde(rename = "errorCode")]
	pub error_code: i32,

	#[serde(rename = "errorInfo")]
	pub error_info: String,

	#[serde(rename = "result")]
	pub result: Option<ZhixueReportMainResultPaper>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZhixuePaperCheckSheetResultStepDatas {
	#[serde(rename = "createTime")]
	pub create_time: Option<i64>,

	#[serde(rename = "id")]
	pub id: Option<i32>,

	#[serde(rename = "markingPaperId")]
	pub marking_paper_id: Option<String>,

	#[serde(rename = "stepNum")]
	pub step_num: Option<i32>,

	#[serde(rename = "stepStandardScore")]
	pub step_standard_score: Option<f64>,

	#[serde(rename = "stepTitle")]
	pub step_title: Option<String>,

	#[serde(rename = "subTopicIndex")]
	pub sub_topic_index: Option<i32>,

	#[serde(rename = "topicStartNum")]
	pub topic_start_num: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZhixuePaperCheckSheetResult {
	#[serde(rename = "currentUserId")]
	pub current_user_id: Option<String>,

	#[serde(rename = "cutBlockDetail")]
	pub cut_block_detail: Option<String>,

	#[serde(rename = "examType")]
	pub exam_type: Option<String>,

	#[serde(rename = "forbidAnnotation")]
	pub forbid_annotation: Option<bool>,

	#[serde(rename = "isRelatedTopic")]
	pub is_related_topic: Option<bool>,

	#[serde(rename = "markingTopicDetail")]
	pub marking_topic_detail: Option<String>,

	#[serde(rename = "score")]
	pub score: Option<f64>,

	#[serde(rename = "scoreMode")]
	pub score_mode: Option<String>,

	#[serde(rename = "sheetDatas")]
	pub sheet_datas: Option<String>,

	#[serde(rename = "sheetImages")]
	pub sheet_images: Option<String>,

	#[serde(rename = "showSingleCorrectTag")]
	pub show_single_correct_tag: Option<bool>,

	#[serde(rename = "showSingleTopicScore")]
	pub show_single_topic_score: Option<bool>,

	#[serde(rename = "showUserScore")]
	pub show_user_score: Option<bool>,

	#[serde(rename = "standardScore")]
	pub standard_score: Option<f64>,

	#[serde(rename = "stepDatas")]
	pub step_datas: Option<Vec<ZhixuePaperCheckSheetResultStepDatas>>,

	#[serde(rename = "tScoreDesc")]
	pub t_score_desc: Option<String>,

	#[serde(rename = "tScoreTitle")]
	pub t_score_title: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZhixuePaperCheckSheet {
	#[serde(rename = "errorCode")]
	pub error_code: i32,

	#[serde(rename = "errorInfo")]
	pub error_info: String,

	#[serde(rename = "result")]
	pub result: Option<ZhixuePaperCheckSheetResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZhixueExamListExamInfoList {
	#[serde(rename = "examCreateDateTime")]
	pub exam_create_date_time: Option<i64>,

	#[serde(rename = "examId")]
	pub exam_id: Option<String>,

	#[serde(rename = "examName")]
	pub exam_name: Option<String>,

	#[serde(rename = "examType")]
	pub exam_type: Option<String>,

	#[serde(rename = "final")]
	pub exam_info_list_final: Option<bool>,

	#[serde(rename = "hasExamReport")]
	pub has_exam_report: Option<bool>,

	#[serde(rename = "homeWork")]
	pub home_work: Option<bool>,

	#[serde(rename = "isFinal")]
	pub is_final: Option<bool>,

	#[serde(rename = "newType")]
	pub new_type: Option<bool>,

	#[serde(rename = "score")]
	pub score: Option<f64>,

	#[serde(rename = "showExportOfflineReport")]
	pub show_export_offline_report: Option<bool>,

	#[serde(rename = "showHomeWorkAnalysis")]
	pub show_home_work_analysis: Option<bool>,

	#[serde(rename = "sign")]
	pub sign: Option<bool>,

	#[serde(rename = "standardTotalScore")]
	pub standard_total_score: Option<i32>,

	#[serde(rename = "threeOrX")]
	pub three_or_x: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZhixueExamListPagination {
	#[serde(rename = "actualPosition")]
	pub actual_position: Option<i32>,

	#[serde(rename = "pageIndex")]
	pub page_index: Option<i32>,

	#[serde(rename = "pageSize")]
	pub page_size: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZhixueExamListResult {
	#[serde(rename = "examInfoList")]
	pub exam_info_list: Option<Vec<ZhixueExamListExamInfoList>>,

	#[serde(rename = "hasNextPage")]
	pub has_next_page: Option<bool>,

	#[serde(rename = "pagination")]
	pub pagination: Option<ZhixueExamListPagination>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZhixueExamList {
	#[serde(rename = "errorCode")]
	pub error_code: i32,

	#[serde(rename = "errorInfo")]
	pub error_info: String,

	#[serde(rename = "result")]
	pub result: Option<ZhixueExamListResult>,
}

#[derive(Serialize, Deserialize)]
pub struct ZhixueAccountStruct {
	#[serde(rename = "cName")]
	pub c_name: String,
	#[serde(rename = "eName")]
	pub e_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ZhixueAccountEducationalSystem {
	pub code: String,
	pub name: String,
	pub phase: String,
	pub phases: Vec<ZhixueAccountPhase>,
}

#[derive(Serialize, Deserialize)]
pub struct ZhixueAccountSchool {
	#[serde(rename = "areaId")]
	pub area_id: String,
	#[serde(rename = "cityId")]
	pub city_id: String,
	pub code: String,
	#[serde(rename = "countryId")]
	pub country_id: String,
	#[serde(rename = "districtId")]
	pub district_id: String,
	#[serde(rename = "educationalSystem")]
	pub educational_system: ZhixueAccountEducationalSystem,
	pub id: String,
	pub name: String,
	pub phase: ZhixueAccountPhase,
	#[serde(rename = "provinceId")]
	pub province_id: String,
	#[serde(rename = "schoolType")]
	pub school_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ZhixueAccountPhase {
	pub code: String,
	pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ZhixueAccountGrade {
	pub code: String,
	pub name: String,
	pub phase: ZhixueAccountPhase,
	pub sort: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ZhixueAccountDivision {
	pub grade: ZhixueAccountGrade,
	pub id: String,
	pub name: String,
	pub phase: ZhixueAccountPhase,
	pub school: ZhixueAccountSchool,
	#[serde(rename = "schoolInYear")]
	pub school_in_year: i64,
	#[serde(rename = "startYear")]
	pub start_year: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ZhixueAccountCreator {
	pub gender: String,
	pub id: String,
	pub lable: String,
}

#[derive(Serialize, Deserialize)]
pub struct Clazz {
	#[serde(rename = "classType")]
	pub class_type: String,
	#[serde(rename = "clazzLabel")]
	pub clazz_label: String,
	pub code: String,
	#[serde(rename = "createTime")]
	pub create_time: i64,
	pub creator: ZhixueAccountCreator,
	pub division: ZhixueAccountDivision,
	pub grade: ZhixueAccountGrade,
	pub id: String,
	#[serde(rename = "isGraduated")]
	pub is_graduated: bool,
	pub name: String,
	pub order: i64,
	#[serde(rename = "orgType")]
	pub org_type: String,
	pub school: ZhixueAccountSchool,
	pub year: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ZhixueAccountStudent {
	pub avatar: String,
	pub birthday: i64,
	pub clazz: Clazz,
	pub code: String,
	#[serde(rename = "createTime")]
	pub create_time: i64,
	pub email: String,
	pub gender: String,
	pub id: String,
	pub im: String,
	pub lable: String,
	#[serde(rename = "loginName")]
	pub login_name: String,
	pub mobile: String,
	pub name: String,
	pub roles: Vec<ZhixueAccountStruct>,
	#[serde(rename = "studentNo")]
	pub student_no: String,
}

#[derive(Serialize, Deserialize)]
pub struct ZhixueAccount {
	pub student: Option<ZhixueAccountStudent>,
	pub user_score: i64,
	#[serde(rename = "user_vipLevel")]
	pub user_vip_level: i64,
	pub user_exp: i64,
	#[serde(rename = "user_vipDays")]
	pub user_vip_days: i64,
}
