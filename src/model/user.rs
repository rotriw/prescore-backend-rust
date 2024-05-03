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

#[derive(Queryable, Selectable, Debug)]
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

// 最会切的一集。但是其中还是有一些问题。期待更聪明的切法。（我是笨蛋）
pub fn combine_class_name(division_name: String, class_name: String) -> String {
    let mut result = "".to_string();
    for (i, &item) in division_name.as_bytes().iter().enumerate() {
        if item.is_ascii_digit() {
            result += &division_name[i..];
            break;
        }
    }
    if result.len() != 0 {
        result += " ";
    }
    for (i, &item) in class_name.as_bytes().iter().enumerate() {
        if item.is_ascii_digit() {
            result += &class_name[i..];
            break;
        }
    }
    result
}

pub fn get_class_name_by_class_id(class_id: String) -> Option<String> {
    use crate::schema::prescore::user::dsl::user as usr;
    use crate::schema::prescore::user::class_id as cid;
    let mut conn = unsafe {
        DBPOOL.clone().unwrap().get().unwrap()
    };
    let result: Vec<User> = usr.filter(cid.eq(class_id.clone()))
        .load::<User>(&mut conn)
        .ok()?;
    let len = result.len();
    if len == 0 {
        return Some("未知班级".to_string());
    }
    for item in result {
        if item.division_name.is_some() && item.class_name.is_some() {
            return Some(combine_class_name(item.division_name.unwrap(), item.class_name.unwrap()));
        }
    }
    return Some("未知班级".to_string());
}