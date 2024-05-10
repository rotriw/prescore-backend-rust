use diesel::RunQueryDsl;
use diesel::prelude::*;
use crate::declare::user::{CreateUser, User};
use crate::service::postgres::DBPOOL;
use crate::service::zhixue::get_user_data;

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
    for (i_left, &item) in division_name.as_bytes().iter().enumerate() {
        if item.is_ascii_digit() {
            result += &division_name[i_left..];
            break;
        }
    }
    if result.len() != 0 {
        result += " ";
    }
    for (i_right, &item) in class_name.as_bytes().iter().enumerate() {
        if item.is_ascii_digit() {
            result += &class_name[i_right..];
            break;
        }
    }
    result
}

pub fn create_user_by_create_user(data: CreateUser) -> Option<()> {
    use crate::schema::prescore::user as usr;
    let mut conn = unsafe {
        DBPOOL.clone().unwrap().get().unwrap()
    };
    diesel::insert_into(usr::table)
        .values(&data)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .ok();
    Some(())
}

pub fn upload_user_by_create_user(data: CreateUser) -> Option<()> {
    use crate::schema::prescore::user::dsl::user as usr;
    use crate::schema::prescore::user::user_id as uid;
    let mut conn = unsafe {
        DBPOOL.clone().unwrap().get().unwrap()
    };
    diesel::update(usr.filter(uid.eq(data.user_id.clone())))
        .set(&data)
        .execute(&mut conn)
        .ok();
    Some(())
}

pub async fn upload_user_by_token(token: String) -> Option<()> { // status
    use crate::schema::prescore::user::user_id;
    use crate::schema::prescore::user::dsl::user;
    let mut conn = unsafe {
        DBPOOL.clone().unwrap().get().unwrap()
    };
    let data = get_user_data(token).await.ok()?;
    println!("{:?}", data.clone());
    let res = user
        .filter(user_id.eq(data.user_id.clone()))
        .get_result::<User>(&mut conn)
        .ok();
    if res.is_none() {
        create_user_by_create_user(data);
    } else {
        upload_user_by_create_user(data);
    }
    Some(())
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
