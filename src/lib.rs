mod models;
mod routes;
mod utils;
#[cfg(test)]
mod tests;

#[macro_use] extern crate rocket;
use rocket::fairing::{self, AdHoc};
use rocket::{Build, Request, Rocket};
use sqlx::postgres::PgPoolOptions;
use crate::routes::item_routes::{create_item, get_all_items, remove_item};
use crate::routes::health_check_routes::{health_check};
use crate::routes::order_routes::{create_order, get_all_order_for_table, get_order_for_table_by_item, get_remaining_order_for_table, remove_all_order_for_table, remove_order_item, update_order};
use crate::routes::table_routes::{create_table, get_all_tables, remove_table};



async fn init_db(rocket: Rocket<Build>) -> fairing::Result {

    let opt = sqlx::postgres::PgConnectOptions::new()
        .host("localhost")
        .database("simple_restaurant")
        .username("simple_restaurant")
        .password("simple_restaurant");

    let db = match PgPoolOptions::new()
        .connect_with(opt).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to connect to SQLx database: {}", e.to_string());
            return Err(rocket);
        }
    };

    return match sqlx::migrate!("./migrations").run(&db).await {
        Ok(_) => Ok(rocket.manage(db)),
        Err(e) => {
            error!("Failed to initialize SQLx database: {}", e);
            Err(rocket)
        }
    };

}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
            rocket.attach(AdHoc::try_on_ignite("init SQLx Database",init_db ))
    })
}

#[catch(404)]
fn not_found(req: &Request) ->  String {
    let _domain = req.headers().get_one("Host").unwrap();
    let _uri = req.uri();
    format!("Invalid URL '{}{}'. Try something else?", _domain, _uri)
}

#[catch(500)]
fn internal_error() -> String {
    "Whoops! Something went wrong.".to_string()
}


pub fn rocket_builder() -> Rocket<Build> {
    rocket::build()
        .attach(stage())
        .mount("/",routes![health_check])
        .mount("/item",routes![create_item, remove_item, get_all_items])
        .mount("/table",routes![get_all_tables, create_table, remove_table])
        .mount("/order",routes![create_order, get_all_order_for_table, get_remaining_order_for_table, remove_all_order_for_table, remove_order_item, update_order, get_order_for_table_by_item])
        .register("/", catchers![not_found, internal_error])
}