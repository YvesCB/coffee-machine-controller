use gloo_net::http::Request;
use gloo_net::Error as GlooErr;

use crate::model::*;

pub async fn get_time() -> Result<Option<String>, GlooErr> {
    let response = Request::get("/api/coffee/start_time").send().await;

    match response {
        Ok(res) => {
            if res.status() == 200 {
                let content: TimePayload = res.json().await.unwrap();
                Ok(Some(content.time))
            } else {
                Ok(None)
            }
        }
        Err(e) => Err(e),
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
    let response = Request::delete("/api/coffee/unset_time").send().await;

    match response {
        Ok(_res) => Ok(()),
        Err(e) => Err(e),
    }
}
