use crate::declare::user::{CreateUser, User};
use crate::service::postgres::DBPOOL;
use crate::service::zhixue::get_user_data;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn get_user_class_id_by_user_id(user_id: String) -> Option<String> {
    use crate::schema::prescore::user::dsl::user as usr;
    use crate::schema::prescore::user::user_id as cid;
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };
    let result = usr.filter(cid.eq(user_id)).load::<User>(&mut conn).ok()?;
    let len = result.len();
    if len == 0 {
        return None;
    }
    return result[0].class_id.clone();
}

// 最会切的一集。但是其中还是有一些问题。期待更聪明的切法。（我是笨蛋）
pub fn combine_class_name(division_name: String, class_name: String) -> (String, u32, u32) {
    let (mut value_left, mut value_right) = (0 as u32, 0 as u32);
    let mut result = "".to_string();
    let mut flag = false;
    for (i_left, &item) in division_name.as_bytes().iter().enumerate() {
        if item.is_ascii_digit() {
            if flag == false {
                flag = true;
                result += &division_name[i_left..];
            }
            value_left *= 10;
            value_left += item as u32 - '0' as u32;
        }
    }
    if result.len() != 0 {
        result += " ";
    }
    flag = false;
    for (i_right, &item) in class_name.as_bytes().iter().enumerate() {
        if item.is_ascii_digit() {
            if flag == false {
                flag = true;
                result += &class_name[i_right..];
            }
            value_right *= 10;
            value_right += item as u32 - '0' as u32;
        }
    }
    (result, value_left, value_right)
}

pub fn create_user_by_create_user(data: CreateUser) -> Option<()> {
    use crate::schema::prescore::user as usr;
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };
    diesel::insert_into(usr::table)
        .values(&data)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .ok();
    Some(())
}

pub fn upload_user_by_create_user_direct(data: CreateUser) -> Option<()> {
    use crate::schema::prescore::user::dsl::user as usr;
    use crate::schema::prescore::user::user_id as uid;
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };
    diesel::update(usr.filter(uid.eq(data.user_id.clone())))
        .set(&data)
        .execute(&mut conn)
        .ok();
    Some(())
}

/*
    我突然发现我的神秘命名法撞名字了。
    这里的createUser是指那个struct。
    这里会数据库寻找并完成更新。而不是在数据库里更新。
*/
pub fn upload_user_by_create_user(data: CreateUser) -> Option<()> {
    use crate::schema::prescore::user::dsl::user;
    use crate::schema::prescore::user::user_id;
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };
    let res = user
        .filter(user_id.eq(data.user_id.clone()))
        .get_result::<User>(&mut conn)
        .ok();
    if res.is_none() {
        create_user_by_create_user(data);
    } else {
        upload_user_by_create_user_direct(data);
    }
    Some(())
}

pub async fn upload_user_by_token(token: String) -> Option<()> {
    // status
    let data = get_user_data(token).await.ok()?;
    upload_user_by_create_user(data)
}

pub fn get_class_name_by_class_id(class_id: String) -> Option<(String, u32, u32)> {
    use crate::schema::prescore::user::class_id as cid;
    use crate::schema::prescore::user::dsl::user as usr;
    let mut conn = unsafe { DBPOOL.clone().unwrap().get().unwrap() };
    let result: Vec<User> = usr
        .filter(cid.eq(class_id.clone()))
        .load::<User>(&mut conn)
        .ok()?;
    let len = result.len();
    if len == 0 {
        return Some(("未知班级".to_string(), 10000000, 100000));
    }
    for item in result {
        if item.division_name.is_some()
            && item.class_name.is_some()
            && item.division_name.clone().unwrap().len() != 0
            && item.class_name.clone().unwrap().len() != 0
        {
            return Some(combine_class_name(
                item.division_name.unwrap(),
                item.class_name.unwrap(),
            ));
        }
    }
    return Some(("未知班级".to_string(), 10000000, 100000));
}
