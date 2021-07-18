use super::super::schema::*;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Insertable)]
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
