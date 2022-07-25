use crate::models::order::{NewOrderItem, OrderItem};
use crate::rocket_builder;
use rand::distributions::uniform::SampleBorrow;
use rocket::futures::task::Spawn;
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::serde_json;
use rocket::serde::json::serde_json::json;
use rocket::serde::{Deserialize, Serialize};

#[async_test]
async fn list_order_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let response = client.get("/item").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[derive(Deserialize)]
struct OrderItemListResponse {
    data: OrderItem,
}

#[async_test]
async fn list_all_order_for_table_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let mut response = client.get("/order/1").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[async_test]
async fn list_all_order_for_invalid_table_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let mut response = client.get("/order/0").dispatch().await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[async_test]
async fn list_all_order_for_table_by_item_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let mut response = client.get("/order/1/1").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[async_test]
async fn list_all_order_for_invalid_table_by_item_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let mut response = client.get("/order/0/1").dispatch().await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[async_test]
async fn list_all_order_for_table_by_invalid_item_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let mut response = client.get("/order/1/0").dispatch().await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[async_test]
async fn list_remaining_order_for_table_by_item_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let mut response = client.get("/order/1/remaining").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[async_test]
async fn list_remaining_order_for_invalid_table_by_item_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let mut response = client.get("/order/0/remaining").dispatch().await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[async_test]
async fn create_order_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();

    let order_items: Vec<NewOrderItem> = vec![
        NewOrderItem {
            item_id: 1,
            quantity: 2,
        },
        NewOrderItem {
            item_id: 2,
            quantity: 5,
        },
    ];

    let mut response = client
        .put("/order/4")
        .header(ContentType::JSON)
        .json(&order_items)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    //clean up
    for item in order_items {
        let res = client
            .delete(format!("/order/4/{}", item.item_id))
            .dispatch()
            .await;
        assert_eq!(res.status(), Status::Ok);
    }
}

#[async_test]
async fn delete_all_order_for_table_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let order_items: Vec<NewOrderItem> = vec![
        NewOrderItem {
            item_id: 1,
            quantity: 2,
        },
        NewOrderItem {
            item_id: 2,
            quantity: 5,
        },
    ];

    let mut response = client
        .put("/order/3")
        .header(ContentType::JSON)
        .json(&order_items)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    response = client.delete("/order/3").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[async_test]
async fn delete_all_order_for_invalid_table_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();
    let response = client.delete("/order/0").dispatch().await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[async_test]
async fn delete_order_for_table_by_invalid_item_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();

    let response = client.delete("/order/1/0").dispatch().await;
    assert_eq!(response.status(), Status::NotFound);
}

#[async_test]
async fn update_order_item_for_table_test() {
    let client = Client::tracked(rocket_builder()).await.unwrap();

    let order_items: Vec<NewOrderItem> = vec![NewOrderItem {
        item_id: 1,
        quantity: 2,
    }];

    let mut response = client
        .put("/order/2")
        .header(ContentType::JSON)
        .json(&order_items)
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Created);

    response = client
        .post("/order/2")
        .header(ContentType::JSON)
        .json(&OrderItem {
            item_id: 1,
            quantity: 15,
            served: false,
        })
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Ok);

    // cleaning up
    for item in order_items {
        response = client
            .delete(format!("/order/2/{}", item.item_id))
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
    }
}
