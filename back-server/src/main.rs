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
        let cur_time: Option<NaiveTime> = get_time_from_db();

        match cur_time {
            Some(time) => {
                let now = Local::now().time();

                if now > time {
                    let _ = blink();

                    let now = Local::now();
                    let next_day = now.date_naive().succ_opt().unwrap();
                    let target_time = next_day.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                    let wait_duration = target_time.signed_duration_since(now.naive_local());

                    thread::sleep(wait_duration.to_std().unwrap());
                }
            }
            None => {
                // do nothing when it's None
            }
        }

        thread::sleep(Duration::from_millis(2000));
    });

    let port = 8099;
    let ip = "0.0.0.0";
    info!("Starting server on {ip}:{port}");
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/coffee")
                .service(coffee_controller::start_time)
                .service(coffee_controller::set_time)
                .service(coffee_controller::unset_time)
                .service(coffee_controller::toggle_on_off),
        )
    })
    .bind((ip, port))?
    .run()
    .await
}
