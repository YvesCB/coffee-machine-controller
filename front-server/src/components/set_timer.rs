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

#[function_component]
pub fn SetTimer() -> Html {
    let time = use_state(|| "00:00".to_string());
    {
        let time = time.clone();
        use_effect_with((), move |_| {
            let time = time.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response: Result<Response, GlooErr> =
                    Request::get("/api/coffee/start_time").send().await;

                match response {
                    Ok(res) => {
                        let payload: TimePayload = res.json().await.unwrap();
                        time.set(payload.time);
                    }
                    // This is when the server doesn't send anything back
                    Err(_) => {
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

    html! {
        <div class={classes!("timer-container")}>
            <form onsubmit={onsubmit}>
                <div class={classes!("timer-left")}>
                    <input type={"time"} id={"time-input"} value={(*time).clone()} oninput={oninput} />
                </div>
                <div class={classes!("timer-right")}>
                    <button type="submit" class={classes!("my-btn")}>{ "Set" }</button>
                    <button type="button" class={classes!("my-btn")}>{ "Unset" }</button>
                </div>
            </form>
        </div>
    }
}
