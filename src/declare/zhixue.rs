#![allow(dead_code)]
pub struct ZhixueUser {
    pub id: i64,
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub login_name: Option<String>,
    pub hashed_token: Option<String>,
    pub school_id: Option<String>,
    pub school_name: Option<String>,
    pub division_id: Option<String>,
    pub division_name: Option<String>,
    pub class_id: Option<String>,
    pub class_name: Option<String>,
    pub child_id: Option<String>,
}

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
