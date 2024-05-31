#[macro_use]
extern crate diesel;

use std::thread;
use std::time::Duration;

use actix_web::{web, App, HttpServer};
use chrono::prelude::*;
use dotenv::dotenv;
use log::info;
use log4rs;

mod controller;
mod model;
mod schema;
mod util;

use controller::coffee_controller;
use util::db_interaction::*;
use util::gpio::blink;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log_config.yml", Default::default()).unwrap();
    info!("Loading env");
    dotenv().ok();

    thread::spawn(|| loop {
        let cur_time = get_time_from_db();
        match cur_time {
            Some(time) => {
                let now = Local::now().time();
                if now > time {
                    let _ = blink();
                }
            }
            None => {}
        }

        thread::sleep(Duration::from_millis(2000));
    });

    let port = 8099;
    let ip = "0.0.0.0";
    info!("Starting server on {ip}:{port}");
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/coffee")
                .service(coffee_controller::hello)
                .service(coffee_controller::set_time)
                .service(coffee_controller::unset_time)
                .service(coffee_controller::toggle_on_off),
        )
    })
    .bind((ip, port))?
    .run()
    .await
}
