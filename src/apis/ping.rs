use rocket::get;

#[get("/ping")]
pub fn handle_get_ping() -> &'static str {
    "pong"
}
