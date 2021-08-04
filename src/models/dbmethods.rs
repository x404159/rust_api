use actix_web::web;
use diesel::prelude::*;

use crate::db::Pool;
use crate::errors::ServiceError;
use crate::models::user::{AuthData, FindBy, SlimUser, User, UserChange, UserData, UserInsert};
use crate::utils::{hash_password, verify_hash};

//route handles helper function
pub fn login_user(user_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{email, users};
    let conn = &pool.get().unwrap();
    let mut items = users
        .filter(email.eq(&user_data.email))
        .load::<User>(conn)?;
    if let Some(user) = items.pop() {
        if let Ok(matching) = verify_hash(&user.password, &user_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}

//route handler helpers
pub fn delete_account(user_email: String, pool: web::Data<Pool>) -> Result<bool, ServiceError> {
    use crate::schema::users::dsl::{email, users};
    let conn = &pool.get().unwrap();
    let result = diesel::delete(users)
        .filter(email.eq_all(user_email))
        .execute(conn)?;
    if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn user_update(
    user: SlimUser,
    updates: UserChange,
    pool: web::Data<Pool>,
) -> Result<UserChange, ServiceError> {
    use crate::schema::users::dsl::{email, users};
    let mut updates = updates;
    if let Some(ref mut passwd) = updates.password {
        *passwd = hash_password(&passwd)?;
    }
    dbg!(&updates);
    let conn = &pool.get().unwrap();
    let result = diesel::update(users)
        .filter(email.eq(user.email))
        .set(&updates)
        .get_result::<UserChange>(conn)?;
    Ok(result)
}

pub fn find_by(data: FindBy, pool: web::Data<Pool>) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::{email, id, users};
    let conn = &pool.get().unwrap();
    let mut user;
    match data {
        FindBy::Email(e) => {
            user = users.filter(email.eq(&e)).get_results::<User>(conn)?;
        }
        FindBy::Id(v) => {
            user = users.filter(id.eq(&v)).get_results::<User>(conn)?;
        }
    }
    if let Some(u) = user.pop() {
        return Ok(u);
    }
    Err(ServiceError::NotFound)
}

//route handles helper function
pub fn change_account(user_id: i64, pool: web::Data<Pool>) -> Result<String, ServiceError> {
    use crate::schema::users::dsl::{clearance, users};
    let conn = &pool.get().unwrap();

    let mut return_string = String::new();
    let target = users.find(user_id);
    let current_clearance = target.select(clearance).get_result::<bool>(conn)?;
    if current_clearance {
        return_string.push_str("change account type from admin to normal user");
        let _ = diesel::update(target)
            .set(clearance.eq_all(false))
            .execute(conn)?;
    } else {
        return_string.push_str("change account type from normal user to admin");
        let _ = diesel::update(target)
            .set(clearance.eq_all(true))
            .execute(conn)?;
    }
    Ok(return_string)
}

pub fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<SlimUser>, ServiceError> {
    use crate::schema::users::dsl::users;
    let conn = &pool.get().unwrap();
    let all_users = users.load::<User>(conn)?;
    Ok(all_users.into_iter().map(|u| u.into()).collect())
}

pub fn insert_user(user_data: UserData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::users;
    let password = crate::utils::hash_password(&user_data.password)?;

    let new_user = UserInsert::from_details(user_data.name, user_data.email, password);

    let conn = &pool.get().unwrap();
    let inserted_user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)?;
    Ok(inserted_user.into())
}

use crate::models::user::RawUser;
use diesel::sql_types::Integer;

pub fn test_raw(pool: &web::Data<Pool>) -> Result<Vec<RawUser>, ServiceError> {
    use diesel::sql_query;
    let conn = &pool.get().unwrap();
    Ok(sql_query(
        "SELECT
    id,
    name,
    email,
    password,
    created_at,
    CONCAT ('your email is ' , LENGTH ( email ), ' charactors long') AS about_email
    FROM users
    WHERE created_at >= '2021-01-01'
    AND created_at < '2021-07-19'
    ORDER BY id
    LIMIT $1
    ",
    )
    .bind::<Integer, _>(4)
    .load(conn)?)
}
