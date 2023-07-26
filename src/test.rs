use super::rocket;
use rocket::local::blocking::Client;

#[test]
fn index() {
    let client = Client::tracked(rocket()).unwrap();

    let response = client.get("/").dispatch();
    assert_eq!(response.into_string().unwrap(), "안녕, 세계!");
}