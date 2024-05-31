use actix_web::{delete, get, post, web::Json, HttpResponse, Responder};
use chrono::prelude::*;
use diesel::prelude::*;
use log::info;

use crate::model::coffee::{SetTimerPayload, Time};
use crate::schema::times;
use crate::util::db_interaction::*;
use crate::util::gpio;

/// Getting the time that is currently set
///
/// Return the time that is currently set for the machine to turn on from the db.
#[get("/start_time")]
pub async fn hello() -> impl Responder {
    info!("Connection to /start_time");

    match get_time_from_db() {
        Some(time) => HttpResponse::Ok().json(SetTimerPayload {
            time: time.format("%H:%M").to_string(),
        }),
        None => HttpResponse::NoContent().body("No time set."),
    }
}

/// Set the time for the machine to turn on
///
/// Sets the time at which the machine will turn on.
#[post("/set_time")]
pub async fn set_time(req: Json<SetTimerPayload>) -> impl Responder {
    let payload = req.into_inner();
    info!("Connection to /set_time");

    match NaiveTime::parse_from_str(&payload.time, "%H:%M") {
        Ok(time) => {
            let mut conn = establish_connection();
            let new_time = Time {
                id: 0, // `id` will be auto-incremented by SQLite
                time: time.format("%H:%M").to_string(),
            };

            diesel::delete(times::table)
                .execute(&mut conn)
                .expect("Error saving clearing time table");

            diesel::insert_into(times::table)
                .values(&new_time)
                .execute(&mut conn)
                .expect("Error saving new time");

            info!(
                "Inserted new time {} into db.",
                time.format("%H:%M").to_string()
            );

            HttpResponse::Ok().body(format!(
                "The time was set to: {}",
                time.format("%H:%M").to_string()
            ))
        }
        Err(_) => HttpResponse::BadRequest()
            .body("Not a valid time. Expected format: %H:%M (for exmaple 14:00)"),
    }
}

/// Send an On/Off signal to the machine
///
/// This endpoint will send an On/Off signal to the coffee machine.
/// Causing it to either turn on or off depending on what its prior state was.
#[post("/toggle_on_off")]
pub async fn toggle_on_off() -> impl Responder {
    info!("Connection to /toggle_on_off");

    match gpio::blink() {
        Ok(_) => println!("blinked"),
        Err(_) => println!("can't blink"),
    };

    HttpResponse::Ok().body("Sending on/off signal to machine")
}

/// Remove set time from db
///
/// This endpoint removes the entered time from the database.
/// That way the timer is no longer running.
#[delete("/unset_time")]
pub async fn unset_time() -> impl Responder {
    info!("Connection to /unset_time");

    let mut conn = establish_connection();

    diesel::delete(times::table)
        .execute(&mut conn)
        .expect("Error saving clearing time table");

    HttpResponse::Ok().body("Removed time")
}
