use rocket::get;

#[get("/ping")]
pub fn health_check() -> &'static str {
    "PONG!!"
}
