mod controller;
mod service;

use rocket::fairing::AdHoc;

pub fn module() -> AdHoc {
    AdHoc::on_ignite("AppModule", |rocket| async {
        rocket.mount("/", routes![self::controller::hello])
    })
}