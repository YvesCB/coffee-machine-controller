use gloo_net::http::{Request, Response};
use gloo_net::Error as GlooErr;
use serde::{Deserialize, Serialize};
use web_sys::console;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
struct TimePayload {
    time: String,
}

impl TimePayload {
    fn new(time: &str) -> Self {
        TimePayload {
            time: time.to_string(),
        }
    }
}

pub async fn get_time() -> Option<String> {
    let response = Request::get("/api/coffee/start_time").send().await;

    match response {
        Ok(res) => {
            if res.status() == 200 {
                let content: TimePayload = res.json().await.unwrap();
                Some(content.time)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub async fn set_time(time: TimePayload) -> Result<(), GlooErr> {
    let response = Request::post("/api/coffee/set_time")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&time).unwrap())
        .send()
        .await;

    match response {
        Ok(_res) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn unset_time() -> Result<(), GlooErr> {
    let response = Request::post("/api/coffee/unset_time").send().await;

    match response {
        Ok(_res) => Ok(()),
        Err(e) => Err(e),
    }
}
