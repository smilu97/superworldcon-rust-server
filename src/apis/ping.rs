use rocket::get;

#[get("/")]
pub fn handle_get_ping() -> &'static str {
    "pong"
}
