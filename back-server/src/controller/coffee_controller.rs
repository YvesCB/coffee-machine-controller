use actix_web::{get, post, web::Json, HttpRequest, HttpResponse, Responder};
use chrono::prelude::*;
use diesel::prelude::*;
use log::info;

use crate::model::coffee::{SetTimerPayload, Time};
use crate::schema::times;
use crate::util::db_interaction::*;

/// Getting the time that is currently set
///
/// Return the time that is currently set for the machine to turn on from the db.
#[get("/start_time")]
pub async fn hello() -> impl Responder {
    info!("Connection to /start_time");

    let cur_time = Utc::now().naive_utc();

    HttpResponse::Ok().body(format!("The current time is: {}", cur_time.to_string()))
}

/// Set the time for the machine to turn on
///
/// Sets the time at which the machine will turn on.
#[post("/set_time")]
pub async fn set_time(req: Json<SetTimerPayload>) -> impl Responder {
    let payload = req.into_inner();
    info!("Connection to /set_time");

    let mut conn = establish_connection();
    let new_time = Time {
        id: 0, // `id` will be auto-incremented by SQLite
        time: payload.time.to_string(),
    };

    diesel::insert_into(times::table)
        .values(&new_time)
        .execute(&mut conn)
        .expect("Error saving new time");

    let payload_time_str = payload.time.to_string();
    info!("Inserted new time {payload_time_str} into db.");

    HttpResponse::Ok().body(format!("The time was set to: {}", payload_time_str))
}

/// Send an On/Off signal to the machine
///
/// This endpoint will send an On/Off signal to the coffee machine.
/// Causing it to either turn on or off depending on what its prior state was.
#[post("/toggle_on_off")]
pub async fn toggle_on_off() -> impl Responder {
    info!("Connection to /toggle_on_off");

    HttpResponse::Ok().body("Sending on/off signal to machine")
}
