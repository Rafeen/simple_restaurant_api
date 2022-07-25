use crate::models::table::{delete_table, fetch_table_by_id, fetch_tables, Table};
use crate::utils::api::api_response::{
    to_bad_request_response, to_internal_server_error_response, to_resource_not_found_response,
    ApiResponse,
};
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::{get, post};
use sqlx::Error;

#[get("/")]
pub async fn get_all_tables(db: &rocket::State<sqlx::PgPool>) -> ApiResponse {
    let r = fetch_tables(db).await;
    match r {
        Ok(val) => ApiResponse {
            status: Status::Ok,
            data: json!({ "items": val }),
        },
        Err(e) => to_resource_not_found_response(e.to_string().as_str()),
    }
}

#[put("/", format = "json", data = "<table>")]
pub async fn create_table(db: &rocket::State<sqlx::PgPool>, table: Json<Table>) -> ApiResponse {
    let r = fetch_table_by_id(db, table.id).await;
    match r {
        Ok(val) => ApiResponse {
            status: Status::Created,
            data: json!({
                "message": "table Created",
                "data": val
            }),
        },
        Err(e) => match e {
            Error::Database(_) => {
                to_bad_request_response("table with the same id already exists".to_string())
            }
            _ => to_internal_server_error_response(),
        },
    }
}

#[delete("/<table_id>")]
pub async fn remove_table(db: &rocket::State<sqlx::PgPool>, table_id: i32) -> ApiResponse {
    let r = delete_table(db, table_id).await;
    match r {
        Ok(val) => match val {
            None => to_resource_not_found_response("item does not exist for requested table id"),
            Some(_) => ApiResponse {
                status: Status::Ok,
                data: json!({
                    "message": "table Deleted",
                }),
            },
        },
        Err(e) => to_resource_not_found_response(e.to_string().as_str()),
    }
}
