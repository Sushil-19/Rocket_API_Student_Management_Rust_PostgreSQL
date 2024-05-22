#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod db;
mod handlers;
mod models;
mod schema;

use rocket::figment::Figment;
use rocket::serde::json::Json;
use rocket::State;
use std::env;
use db::init_pool;
use handlers::*;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url);

    rocket::build()
        .manage(pool)
        .mount(
            "/",
            routes![
                get_students,
                get_student_by_id,
                create_student,
                update_student,
                delete_student
            ],
        )
}
