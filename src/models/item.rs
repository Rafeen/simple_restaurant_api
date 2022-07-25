use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Json};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Item {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) duration: f64,
    pub(crate) price: f64,
    pub(crate) available: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewItem {
    pub(crate) name: String,
    pub(crate) price: f64,
}

pub async fn fetch_items(db: &rocket::State<sqlx::PgPool>) -> sqlx::Result<Vec<Item>> {
    let query = format!("SELECT * from items;");
    sqlx::query_as::<_, Item>(&query).fetch_all(&**db).await
}

pub async fn insert_item(db: &rocket::State<sqlx::PgPool>, item: Json<NewItem>) -> sqlx::Result<Item> {
    sqlx::query_as::<_, Item>(r#"INSERT INTO items ( name, duration, price, available) VALUES ($1, $2, $3, $4) returning * "#)
        .bind(item.name.to_string())
        .bind(generate_random_duration().await)
        .bind(item.price)
        .bind(true)
        .fetch_one(&**db).await
}

pub async fn delete_item(db: &rocket::State<sqlx::PgPool>, item_id: i32) -> sqlx::Result<Option<Item>> {
    sqlx::query_as::<_, Item>(r#"
            delete from items
            WHERE id = $1
            returning *
            "#)
        .bind(item_id)
        .fetch_optional(&**db).await
}

async fn generate_random_duration()-> f64{
    let mut rng = rand::thread_rng();
    return rng.gen_range(5..=15) as f64;
}

