use rocket::get;

use crate::responses::{self, APIResponse};

#[get("/ping")]
pub fn handle_get_ping() -> APIResponse {
    responses::ok().message("pong")
}
