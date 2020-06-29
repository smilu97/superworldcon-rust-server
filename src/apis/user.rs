use rocket::{get, post};
use rocket_contrib::json::Json;
use serde_json::json;
use diesel::prelude::*;
use chrono::{Utc, Duration};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use crate::schema;
use crate::database::DbConn;
use crate::responses::{self, APIResponse};

use crate::models::user::User;
use crate::models::user::NewUser;
use crate::models::user::UserInput;
use crate::models::user::LoginInput;

#[get("/whoami")]
pub fn handle_get_whoami(user: User) -> APIResponse {
    responses::ok().data(user)
}

#[post("/register", format = "application/json", data = "<user_in>")]
pub fn handle_post_register(db: DbConn, user_in: Json<UserInput>) -> APIResponse {
    use crate::schema::users::dsl::*;

    let hash = User::hash_password(user_in.password.as_str());

    let new_user = NewUser {
        email: user_in.email.clone(),
        password_hash: hash,
    };
    let res_user = diesel::insert_into(users)
        .values(new_user)
        .get_result::<User>(&*db)
        .expect("Error inserting a new user");
    
    responses::created().data(json!({
        "message": "Created a new user account",
        "email": user_in.email.clone(),
        "id": res_user.id,
    }))
}

#[post("/login", format = "application/json", data = "<login_in>")]
pub fn handle_post_login(db: DbConn, login_in: Json<LoginInput>) -> APIResponse {
    use crate::schema::users::dsl::*;
    let res_user = users
        .filter(schema::users::email.eq(&login_in.email))
        .first::<User>(&*db);
    if res_user.is_err() {
        return responses::unauthorized().message("Username or password incorrect.");
    }
    let user = res_user.unwrap();
    let hash = User::hash_password(login_in.password.as_str());
    if user.password_hash != hash {
        return responses::unauthorized().message("Username or password incorrect.");
    }

    let token: String;

    if user.has_valid_auth_token(Duration::days(3650)) {
        token = user.current_auth_token.unwrap();
    } else {
        let rng = thread_rng();
        let new_auth_token = rng
            .sample_iter(&Alphanumeric)
            .take(32)
            .collect::<String>();
        
        diesel::update(
            users.filter(schema::users::id.eq(user.id))
        )
            .set((
                current_auth_token.eq(&new_auth_token),
                last_login.eq(&Utc::now().naive_utc()),
            ))
            .execute(&*db)
            .expect("Error");
        token = new_auth_token;
    }   
    
    responses::ok().data(json!({
        "message": "login success",
        "id": user.id,
        "token": token,
    }))
}
