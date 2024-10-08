use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::Deserialize;
#[derive(Insertable, AsChangeset, Clone, Debug, Deserialize)]
#[diesel(table_name = crate::schema::prescore::user)]

pub struct CreateUser {
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub login_name: Option<String>,
    pub school_id: Option<String>,
    pub school_name: Option<String>,
    pub division_id: Option<String>,
    pub division_name: Option<String>,
    pub class_id: Option<String>,
    pub class_name: Option<String>,
    pub child_id: Option<String>,
}

#[derive(Queryable, Selectable, Debug)]
#[allow(dead_code)]
#[diesel(table_name = crate::schema::prescore::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
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
