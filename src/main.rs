#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod commands;
mod parser;
mod routes;

pub fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![routes::parse, routes::list_all])
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        println!("Whoops! Rocket didn't launch! {:?}", e);
        drop(e);
    }
}
