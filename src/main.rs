#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;

use rocket::{routes, catchers};
use dotenv::dotenv;

mod apis;
mod config;
mod database;
mod handlers;
mod responses;
mod schema;
mod utils;
mod models;

fn main() -> Result<(), String> {
    dotenv().ok();

    let (app_config, rocket_config) =
        config::get_rocket_config().map_err(|x| format!("{}", x))?;

    let rocket = rocket::custom(rocket_config)
        .attach(database::DbConn::fairing())
        .manage(app_config)
        .mount("/", routes![
            apis::ping::handle_get_ping,
            apis::contest::handle_get_contests,
            apis::contest::handle_get_contest,
            apis::contest::handle_post_contest,
            apis::user::handle_get_whoami,
            apis::user::handle_post_register,
            apis::user::handle_post_login,
            apis::tournament::handle_post_tournament,
        ])
        .register(catchers![
            handlers::bad_request_handler,
            handlers::unauthorized_handler,
            handlers::forbidden_handler,
            handlers::not_found_handler,
            handlers::internal_server_error_handler,
            handlers::service_unavailable_handler,
        ])
        .launch();
    
    Ok(())
}
