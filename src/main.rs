#[macro_use] extern crate rocket;

mod app;

#[cfg(test)]
mod test;

use rocket_boilerplate::{print_version_info, logger_log};

#[launch]
fn rocket() -> _ {
    print_version_info!("Rocket", "1.0.0");

    logger_log!("RocketFactory", "Starting Rocket application...");
    rocket::build().attach(app::module())
}