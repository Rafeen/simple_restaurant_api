use crate::models::item::{delete_item, fetch_items, insert_item, NewItem};
use crate::utils::api::api_response::{
    to_bad_request_response, to_resource_not_found_response, ApiResponse,
};
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::{delete, get, put};

#[get("/")]
pub async fn get_all_items(db: &rocket::State<sqlx::PgPool>) -> ApiResponse {
    let r = fetch_items(db).await;
    match r {
        Ok(val) => ApiResponse {
            status: Status::Ok,
            data: json!({ "items": val }),
        },
        Err(e) => to_resource_not_found_response(e.to_string().as_str()),
    }
}

#[put("/", format = "json", data = "<item>")]
pub async fn create_item(db: &rocket::State<sqlx::PgPool>, item: Json<NewItem>) -> ApiResponse {
    let r = insert_item(db, item).await;
    match r {
        Ok(val) => ApiResponse {
            status: Status::Created,
            data: json!({
                "message": "Item Created",
                "data": val
            }),
        },
        Err(_) => to_bad_request_response("item id already exists".to_string()),
    }
}

#[delete("/<item_id>")]
pub async fn remove_item(db: &rocket::State<sqlx::PgPool>, item_id: i32) -> ApiResponse {
    let r = delete_item(db, item_id).await;
    match r {
        Ok(val) => match val {
            None => to_resource_not_found_response("item does not exist for requested item_id"),
            Some(_) => ApiResponse {
                status: Status::Ok,
                data: json!({
                    "message": "Item Deleted",
                }),
            },
        },
        Err(e) => to_resource_not_found_response(e.to_string().as_str()),
    }
}
