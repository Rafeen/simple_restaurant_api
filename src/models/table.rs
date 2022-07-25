use rocket::serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize,sqlx::FromRow)]
pub struct Table {
    pub(crate) id: i32,
}

pub async fn fetch_tables(db:&rocket::State<sqlx::PgPool>)->sqlx::Result<Vec<Table>> {
    let query = format!("SELECT id from tables;");
    sqlx::query_as::<_,Table>(&query).fetch_all(&**db).await
}

pub async fn fetch_table_by_id(db:&rocket::State<sqlx::PgPool>, table_id:i32) ->sqlx::Result<Option<Table>> {
    sqlx::query_as::<_, Table>(r#"INSERT INTO tables (id) VALUES ($1) returning * "#)
        .bind(table_id)
        .fetch_optional(&**db).await
}

pub async fn delete_table(db: &rocket::State<sqlx::PgPool>, table_id: i32) -> sqlx::Result<Option<Table>> {
    sqlx::query_as::<_, Table>(r#"
            delete from tables
            WHERE id = $1
            returning *
            "#)
        .bind(table_id)
        .fetch_optional(&**db).await
}
