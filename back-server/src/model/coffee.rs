use crate::schema::times;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "times"]
pub struct Time {
    pub id: i32,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetTimerPayload {
    pub time: NaiveDateTime,
}
