#![feature(proc_macro_hygiene, decl_macro)]

use rocket::routes;

mod apis;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            apis::ping::handle_get_ping,
        ])
        .launch();
}
