use gloo_net::http::{Request, Response};
use gloo_net::Error as GlooErr;
use serde::{Deserialize, Serialize};
use web_sys::console;
use yew::prelude::*;

use crate::api::calls as api_calls;

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

#[function_component]
pub fn SetTimer() -> Html {
    let time = use_state(|| "00:00".to_string());
    {
        let time = time.clone();
        use_effect_with((), move |_| {
            let time = time.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_time = api_calls::get_time().await;

                match fetched_time {
                    Some(fetched_time) => {
                        time.set(fetched_time);
                    }
                    _ => {
                        time.set("00:00".to_string());
                    }
                }
            });
            || ()
        });
    }

    let oninput = {
        let time = time.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                time.set(input.value());
            }
        })
    };

    let onsubmit = {
        let time = time.clone();
        Callback::from(move |event: web_sys::SubmitEvent| {
            event.prevent_default();
            // Handle form submission logic with the `time` value
            let message = String::from("submitting");
            web_sys::console::log_1(&message.into());

            let time_value = (*time).clone();
            wasm_bindgen_futures::spawn_local(async move {
                let payload = TimePayload::new(&time_value);
                let response = Request::post("/api/coffee/set_time")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .send()
                    .await;

                match response {
                    Ok(_) => web_sys::console::log_1(&"Successfully sent the request".into()),
                    Err(err) => web_sys::console::log_1(&format!("Error: {:?}", err).into()),
                }
            });
        })
    };

    let onclick = {
        let time = time.clone();
        Callback::from(move |_| {
            let has_time = has_time.clone();
            let time = time.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let time = time.clone();
                let response = Request::delete("/api/coffee/unset_time").send().await;

                match response {
                    Ok(_) => {
                        time.set("00:00".to_string());
                    }
                    Err(_) => {
                        web_sys::console::log_1(&"Can't delete time".into());
                        has_time.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div class={classes!("timer-container")}>
            <form onsubmit={onsubmit}>
                <div class={classes!("timer-left")}>
                    <input type={"time"} id={"time-input"} value={(*time).clone()} oninput={oninput} />
                </div>
                <div class={classes!("timer-right")}>
                    <button type="submit" class={classes!("my-btn")}>{ "Set" }</button>
                    <button {onclick} type="button" class={classes!("my-btn")}>{ "Unset" }</button>
                </div>
            </form>
        </div>
    }
}
