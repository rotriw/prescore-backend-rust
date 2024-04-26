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