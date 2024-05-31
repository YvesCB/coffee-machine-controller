use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TimeProp {
    pub time: UseStateHandle<String>,
    pub is_active: UseStateHandle<bool>,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct TimePayload {
    pub time: String,
}

impl TimePayload {
    pub fn new(time: String) -> Self {
        TimePayload { time }
    }
}
