use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

use crate::model::coffee::Time;
use crate::schema::times;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_time_from_db() -> Option<NaiveTime> {
    let mut conn = establish_connection();

    let db_time: Vec<Time> = times::dsl::times
        .select(Time::as_select())
        .load(&mut conn)
        .expect("Could not load time");

    match db_time.len() {
        0 => None,
        _ => Some(NaiveTime::parse_from_str(&db_time[0].time, "%H:%M").unwrap()),
    }
}
