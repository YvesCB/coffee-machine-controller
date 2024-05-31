use crate::schema::times;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = times)]
pub struct Time {
    pub id: i32,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetTimerPayload {
    pub time: String,
}
