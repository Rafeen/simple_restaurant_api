use crate::models::order::{
    delete_all_orders_for_table, delete_order_item_for_table, fetch_all_order_item_for_table,
    fetch_order_item_by_item, fetch_remaining_order_item_for_table, insert_order_item,
    is_duplicate_order, update_order_item, NewOrderItem, OrderItem,
};
use crate::utils::api::api_response::{
    to_bad_request_response, to_resource_not_found_response, ApiResponse,
};
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use sqlx::Error;

#[get("/<table_id>")]
pub async fn get_all_order_for_table(
    db: &rocket::State<sqlx::PgPool>,
    table_id: i32,
) -> ApiResponse {
    let r = fetch_all_order_item_for_table(db, table_id).await;
    match r {
        Ok(val) => ApiResponse {
            status: Status::Ok,
            data: json!({ "data": val }),
        },
        Err(e) => match e {
            Error::RowNotFound => to_resource_not_found_response("table doesn't exist"),
            _ => to_bad_request_response(e.to_string()),
        },
    }
}

#[get("/<table_id>/<item_id>")]
pub async fn get_order_for_table_by_item(
    db: &rocket::State<sqlx::PgPool>,
    table_id: i32,
    item_id: i32,
) -> ApiResponse {
    let r = fetch_order_item_by_item(db, table_id, item_id).await;
    match r {
        Ok(val) => ApiResponse {
            status: Status::Ok,
            data: json!({
                "data": val,
            }),
        },
        Err(e) => match e {
            Error::RowNotFound => to_resource_not_found_response("invalid item_id or table_id"),
            _ => to_bad_request_response(e.to_string()),
        },
    }
}

#[get("/<table_id>/remaining")]
pub async fn get_remaining_order_for_table(
    db: &rocket::State<sqlx::PgPool>,
    table_id: i32,
) -> ApiResponse {
    let r = fetch_remaining_order_item_for_table(db, table_id).await;
    match r {
        Ok(val) => ApiResponse {
            status: Status::Ok,
            data: json!({ "data": val }),
        },
        Err(e) => match e {
            Error::RowNotFound => to_resource_not_found_response("invalid table_id"),
            _ => to_bad_request_response(e.to_string()),
        },
    }
}

#[put("/<table_id>", format = "json", data = "<order_items>")]
pub async fn create_order(
    db: &rocket::State<sqlx::PgPool>,
    table_id: i32,
    order_items: Json<Vec<NewOrderItem>>,
) -> ApiResponse {
    let is_duplicate = is_duplicate_order(db, table_id, &order_items).await;
    return match is_duplicate {
        true => to_bad_request_response(format!(
            "order for these items for table {} already exists",
            table_id
        )),
        false => {
            let r = insert_order_item(db, order_items, table_id).await;
            match r {
                Ok(_) => ApiResponse {
                    status: Status::Created,
                    data: json!({
                        "message": "Order Created",
                    }),
                },
                Err(e) => match e {
                    Error::RowNotFound => to_resource_not_found_response("table doesn't exist"),
                    _ => to_bad_request_response(e.to_string()),
                },
            }
        }
    };
}

#[delete("/<table_id>")]
pub async fn remove_all_order_for_table(
    db: &rocket::State<sqlx::PgPool>,
    table_id: i32,
) -> ApiResponse {
    let r = delete_all_orders_for_table(db, table_id).await;
    match r {
        Ok(val) => match val {
            None => to_resource_not_found_response("order does not exist for requested table"),
            Some(_) => ApiResponse {
                status: Status::Ok,
                data: json!({
                    "message": "all items deleted for the table",
                }),
            },
        },
        Err(e) => to_resource_not_found_response(e.to_string().as_str()),
    }
}

#[delete("/<table_id>/<item_id>")]
pub async fn remove_order_item(
    db: &rocket::State<sqlx::PgPool>,
    table_id: i32,
    item_id: i32,
) -> ApiResponse {
    let r = delete_order_item_for_table(db, table_id, item_id).await;
    match r {
        Ok(val) => match val {
            None => to_resource_not_found_response("order does not exist for requested table"),
            Some(_) => ApiResponse {
                status: Status::Ok,
                data: json!({
                    "message": "Order for item Deleted",
                }),
            },
        },
        Err(e) => to_resource_not_found_response(e.to_string().as_str()),
    }
}

#[post("/<table_id>", data = "<order_item>")]
pub async fn update_order(
    db: &rocket::State<sqlx::PgPool>,
    table_id: i32,
    order_item: Json<OrderItem>,
) -> ApiResponse {
    let r = update_order_item(db, order_item, table_id).await;
    match r {
        Ok(val) => match val {
            None => to_resource_not_found_response("item does not exist for requested table_id"),
            Some(_) => ApiResponse {
                status: Status::Ok,
                data: json!({
                    "message": "Order Updated",
                }),
            },
        },
        Err(e) => to_resource_not_found_response(e.to_string().as_str()),
    }
}
