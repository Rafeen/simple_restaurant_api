use crate::models::item::{Item, NewItem};
use crate::rocket_builder;
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::{Deserialize, Serialize};

#[cfg(test)]
#[async_test]
async fn items_list_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let response = client.get("/item").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}
#[derive(Deserialize)]
struct ItemResponse {
    message: String,
    data: Item,
}
#[async_test]
async fn new_item_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let item = NewItem {
        name: "test_item".to_string(),
        price: 50.00,
    };
    let mut response = client
        .post("/item")
        .header(ContentType::JSON)
        .json(&item)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let created_item = response.into_json::<ItemResponse>().await.unwrap();
    // Cleanup
    let res = client
        .delete(format!("/item/{}", created_item.data.id))
        .dispatch()
        .await;
    assert_eq!(res.status(), Status::Ok);
}

#[async_test]
async fn delete_item_error_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let item = Item {
        id: 6,
        name: "test_item".to_string(),
        duration: 15.0,
        price: 90.0,
        available: true,
    };
    let mut response = client.delete(format!("/item/{}", item.id)).dispatch().await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}
