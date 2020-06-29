use rocket::get;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{catch, Outcome};

use crate::responses::{self, APIResponse};

pub struct TestData {
    pub value: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for TestData {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<TestData, ()> {
        Outcome::Success(TestData {
            value: String::from("pong!"),
        })
    }
}

#[get("/ping")]
pub fn handle_get_ping(data: TestData) -> APIResponse {
    responses::ok().message(format!("{}", data.value).as_str())
}
