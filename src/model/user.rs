use diesel::{Insertable, Queryable, RunQueryDsl, Selectable};
use diesel::prelude::*;
use crate::service::postgres::DBPOOL;

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::prescore::user)]

pub struct CreateUser {
    pub id: i64,
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

#[derive(Queryable, Selectable)]
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


pub fn get_user_class_id_by_user_id(user_id: String) -> Option<String> {
    use crate::schema::prescore::user::dsl::user as usr;
    use crate::schema::prescore::user::user_id as cid;
    let mut conn = unsafe {
        DBPOOL.clone().unwrap().get().unwrap()
    };
    let result = usr.filter(cid.eq(user_id))
        .load::<User>(&mut conn)
        .ok()?;
    let len = result.len();
    if len == 0 {
        return None;
    }
    return result[0].class_id.clone();
}