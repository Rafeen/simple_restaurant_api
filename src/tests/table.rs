use crate::models::table::Table;
use crate::rocket_builder;
use rand::distributions::uniform::SampleBorrow;
use rocket::futures::task::Spawn;
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::{Client, LocalResponse};
use rocket::serde::json::serde_json;

#[async_test]
async fn table_list_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let response = client.get("/table").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[async_test]
async fn new_table_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let table = Table { id: 100 };
    let mut response = client
        .post("/table")
        .header(ContentType::JSON)
        .json(&table)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // Cleanup
    let res = client
        .delete(format!("/table/{}", table.id))
        .dispatch()
        .await;
    assert_eq!(res.status(), Status::Ok);
}

#[async_test]
async fn new_table_error_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let table = Table { id: 15 };
    let mut response = client
        .post("/table")
        .header(ContentType::JSON)
        .json(&table)
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    response = client
        .post("/table")
        .header(ContentType::JSON)
        .json(&table)
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // Cleanup
    let res = client
        .delete(format!("/table/{}", table.id))
        .dispatch()
        .await;
    assert_eq!(res.status(), Status::Ok);
}

#[async_test]
async fn delete_table_error_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let table = Table { id: 20 };
    let mut response = client
        .delete(format!("/table/{}", table.id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}
