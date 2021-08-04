use super::super::schema::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct UserChange {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl Queryable<users::SqlType, diesel::pg::Pg> for UserChange {
    type Row = (i64, String, String, String, bool, chrono::NaiveDateTime);

    fn build(row: Self::Row) -> Self {
        Self {
            name: Some(row.1),
            email: Some(row.2),
            password: Some("sensitive content".to_owned()),
        }
    }
}

#[derive(Queryable, Serialize, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    //true for admins false for normal users
    pub clearance: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Insertable, Serialize)]
#[table_name = "users"]
pub struct UserInsert {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl UserInsert {
    pub fn from_details<N: Into<String>, E: Into<String>, P: Into<String>>(
        name: N,
        email: E,
        password: P,
    ) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            password: password.into(),
        }
    }
}

//for hidding fields in response
#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
    pub clearance: bool,
}

//like userSchema.toJSON in mongoose
impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        Self {
            email: user.email,
            clearance: user.clearance,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub enum FindBy {
    Email(String),
    Id(i64),
}

#[derive(Deserialize)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub password: String,
}

use crate::errors::ServiceError;
use actix_identity::Identity;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

//impl fromrequest so that we can extract LoggedUser from req: HttpRequest as Json
impl FromRequest for SlimUser {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<SlimUser, Error>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, payload).into_inner() {
            if let Some(user_json) = identity.identity() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ok(user);
                }
            }
        }
        err(ServiceError::Unauthorized.into())
    }
}

//testing raw sql

use diesel::sql_types::*;

#[derive(QueryableByName, Deserialize, Serialize)]
pub struct RawUser {
    #[sql_type = "BigSerial"]
    id: i64,
    #[sql_type = "VarChar"]
    name: String,
    #[sql_type = "VarChar"]
    email: String,
    #[sql_type = "Timestamp"]
    created_at: chrono::NaiveDateTime,
    #[sql_type = "Text"]
    about_email: String,
}
