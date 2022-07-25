use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Json};
use sqlx::{Error, PgPool};
use chrono::{DateTime, Duration};
use chrono::prelude::*;
use rocket::http::Status;
use rocket::State;
use crate::models::item::Item;
use crate::models::table::Table;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItem {
    pub(crate) item_id: i32,
    pub(crate) quantity: i32,
    pub(crate) served: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct NewOrderItem {
    pub(crate) item_id: i32,
    pub(crate) quantity: i32,
}

pub async fn insert_order_item(db: &rocket::State<sqlx::PgPool>, new_order: Json<Vec<NewOrderItem>>, table_id: i32) -> Result<Status, Error> {

    let mut transaction = db.begin().await?;
    for new_item in new_order.to_vec() {

        let query = format!("SELECT * from items where id={} ;", new_item.item_id);
        let item = sqlx::query_as::<_, Item>(&query).fetch_one(&**db).await;
        let duration = match item {
            Ok(item) => {
                item.duration
            }
            Err(e) => {
                return Err(e);
            }
        };

          let order_item = sqlx::query_as::<_, OrderItem>(r#"INSERT INTO order_items ( table_id, item_id, quantity, serving_at) VALUES ($1, $2, $3, $4) returning * "#)
            .bind(table_id)
            .bind(new_item.item_id)
            .bind(new_item.quantity)
            .bind(calculate_item_serving_time(duration).await)
            .fetch_one(&mut *transaction).await;

        match order_item {
            Ok(_) => {
                continue;
            }
            Err(e) => {
                transaction.rollback().await?;
                return Err(e)
            }
        };

    }
    transaction.commit().await?;
    return Ok(Status::Ok);

}

pub async fn fetch_order_item_by_item(db: &rocket::State<sqlx::PgPool>, table_id: i32, item_id: i32) -> sqlx::Result<Vec<OrderItem>> {
    let query = format!("SELECT * from items where id={} ;", item_id);
    let item = sqlx::query_as::<_, Item>(&query).fetch_optional(&**db).await.unwrap();
    let query = format!("SELECT * from tables where id={} ;", table_id);
    let table = sqlx::query_as::<_, Table>(&query).fetch_optional(&**db).await.unwrap();
    match item {
        None => {
            return Err(sqlx::Error::RowNotFound);
        }
        Some(_) => {}
    }

    match table {
        None => {
            return Err(sqlx::Error::RowNotFound);
        }
        Some(_) => {}
    }
    sqlx::query_as::<_, OrderItem>(r#"
            SELECT * from order_items
            where table_id = $1 and item_id = $2
            "#)
        .bind(table_id)
        .bind(item_id)
        .fetch_all(&**db).await
}

pub async fn fetch_all_order_item_for_table(db: &rocket::State<sqlx::PgPool>, table_id: i32) -> sqlx::Result<Vec<OrderItem>> {
    let query = format!("SELECT * from tables where id={} ;", table_id);
    let table = sqlx::query_as::<_, Table>(&query).fetch_optional(&**db).await.unwrap();

    return match table {
        Some(_) => {
            sqlx::query_as::<_, OrderItem>(r#"
            SELECT * from order_items
                where table_id=$1
                "#)
                .bind(table_id)
                .fetch_all(&**db).await
        }
        None => {
            Err(sqlx::Error::RowNotFound)
        }
    };
}

pub async fn fetch_remaining_order_item_for_table(db: &rocket::State<sqlx::PgPool>, table_id: i32) -> sqlx::Result<Vec<OrderItem>> {
    let query = format!("SELECT * from tables where id={} ;", table_id);
    let table = sqlx::query_as::<_, Table>(&query).fetch_optional(&**db).await.unwrap();

    return match table {
        Some(_) => {
            sqlx::query_as::<_, OrderItem>(r#"
            SELECT * from order_items
            where table_id = $1 and served = $2
            "#)
                .bind(table_id)
                .bind(false)
                .fetch_all(&**db).await
        }
        None => {
            Err(sqlx::Error::RowNotFound)
        }
    };
}

pub async fn update_order_item(db: &rocket::State<sqlx::PgPool>, order_item: Json<OrderItem>, table_id: i32) -> sqlx::Result<Option<OrderItem>> {
    sqlx::query_as::<_, OrderItem>(r#"
            update order_items
            set quantity=$1, served=$2
            where table_id = $3 and item_id = $4
            returning *
            "#)
        .bind(order_item.quantity)
        .bind(order_item.served)
        .bind(table_id)
        .bind(order_item.item_id)
        .fetch_optional(&**db).await
}

pub async fn delete_order_item_for_table(db: &rocket::State<sqlx::PgPool>, table_id: i32, item_id: i32) -> sqlx::Result<Option<OrderItem>> {
    sqlx::query_as::<_, OrderItem>(r#"
            delete from order_items
            WHERE table_id = $1
            and item_id = $2
            returning *
            "#)
        .bind(table_id)
        .bind(item_id)
        .fetch_optional(&**db).await
}

pub async fn delete_all_orders_for_table(db: &rocket::State<sqlx::PgPool>, table_id: i32) -> Result<Option<OrderItem>, Error> {
    sqlx::query_as::<_, OrderItem>(r#"
            delete from order_items
            WHERE table_id = $1
            returning *
            "#)
        .bind(table_id)
        .fetch_optional(&**db).await
}

pub async fn is_duplicate_order(db: &rocket::State<sqlx::PgPool>, table_id:i32, order_items: &Json<Vec<NewOrderItem>>) -> bool {

    for item in order_items.to_vec() {
        let query = format!("SELECT * from order_items where table_id={} and item_id={} ;", table_id, item.item_id);
        let rows_affected = sqlx::query(&query).execute(&**db).await.unwrap().rows_affected();

        if rows_affected > 0{
            return true;
        }

    }
     return false;
}

async fn calculate_item_serving_time(duration: f64) -> DateTime<Local> {
    return Local::now() + Duration::minutes(f64::from(duration) as i64);
}