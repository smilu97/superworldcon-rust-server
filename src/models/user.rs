use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{catch, Outcome};
use diesel::prelude::*;

use ring::constant_time::verify_slices_are_equal;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, NaiveDateTime, Utc};
use argon2rs::argon2i_simple;

use crate::database::DbConn;
use crate::schema::users;

#[derive(Queryable, Deserialize, Serialize, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Vec<u8>,
    pub current_auth_token: Option<String>,
    pub last_login: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password_hash: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub struct UserInput {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn hash_password<'r>(password: &'r str) -> Vec<u8> {
        argon2i_simple(password, "loginsalt").to_vec()
    }
    pub fn get_user_from_login_token(token: &str, db: DbConn) -> Option<User> {
        use crate::schema::users::dsl::*;

        let v: Vec<&str> = token.split(':').collect();
        let user_id = Uuid::parse_str(v.get(0).unwrap_or(&"")).unwrap_or_default();
        let auth_token = v.get(1).unwrap_or(&"").to_string();

        let user = users.find(user_id).first::<User>(&*db).optional();
        if let Ok(Some(u)) = user {
            if let Some(token) = u.current_auth_token.clone() {
                if verify_slices_are_equal(token.as_bytes(), auth_token.as_bytes()).is_ok() {
                    return Some(u);
                }
            }
        }
        None
    }
    pub fn has_valid_auth_token(&self, auth_token_timeout: Duration) -> bool {
        let latest_valid_date = Utc::now() - auth_token_timeout;
        if let Some(last_login) = self.last_login {
            if self.current_auth_token.is_some() {
                last_login > latest_valid_date.naive_utc()
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let db = <DbConn as FromRequest>::from_request(request)?;
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        };

        let token_header = keys[0];
        let token = token_header.replace("Bearer ", "");

        match User::get_user_from_login_token(&token, db) {
            Some(user) => Outcome::Success(user),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
