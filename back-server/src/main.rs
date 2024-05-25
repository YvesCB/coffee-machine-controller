#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use log4rs;

mod controller;
mod model;
mod schema;
mod util;

use controller::coffee_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log_config.yml", Default::default()).unwrap();
    info!("Loading env");
    dotenv().ok();

    let port = 8099;
    let ip = "0.0.0.0";
    info!("Starting server on {ip}:{port}");
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/coffee")
                .service(coffee_controller::hello)
                .service(coffee_controller::set_time),
        )
    })
    .bind((ip, port))?
    .run()
    .await
}
