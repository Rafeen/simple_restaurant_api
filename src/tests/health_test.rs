use rand::distributions::uniform::SampleBorrow;
use rocket::futures::task::Spawn;
use rocket::http::{ContentType, Status};
use rocket::serde::json::serde_json;
use crate::models::table::Table;
use rocket::local::blocking::Client;
use crate::rocket_builder;

#[test]
fn test_ping() {
    let client = Client::tracked(rocket_builder()).unwrap();
    let response = client.get("/ping").dispatch();
    assert_eq!(response.status(), Status::Ok);
}