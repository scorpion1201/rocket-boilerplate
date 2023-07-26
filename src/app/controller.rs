use super::service;

#[get("/")]
pub fn hello() -> &'static str {
    service::hello()
}