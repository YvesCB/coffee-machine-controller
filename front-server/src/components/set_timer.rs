use web_sys::console;
use yew::prelude::*;

use crate::api::calls as api_calls;
use crate::model::*;

#[function_component]
pub fn SetTimer(props: &TimeProp) -> Html {
    let local_time = use_state(|| (*props.time).clone());

    {
        let local_time = local_time.clone();
        let time = props.time.clone();
        let is_active = props.is_active.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let local_time = local_time.clone();
                let time = time.clone();
                let is_active = is_active.clone();

                let fetched_time = api_calls::get_time().await;

                match fetched_time {
                    Ok(ok_time) => match ok_time {
                        Some(fetched_time) => {
                            time.set(fetched_time.to_owned());
                            local_time.set(fetched_time.to_owned());
                            is_active.set(true);
                        }
                        _ => {
                            time.set("00:00".to_string());
                            local_time.set("00:00".to_string());
                            is_active.set(false);
                        }
                    },
                    Err(e) => {
                        console::log_1(&format!("Could not fetch time: {}", e).into());
                        local_time.set("00:00".to_string());
                        is_active.set(false);
                    }
                };
            });
            || ()
        });
    }

    let oninput = {
        let local_time = local_time.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                local_time.set(input.value());
            }
        })
    };

    let onsubmit = {
        let local_time = local_time.clone();
        let time = props.time.clone();
        let is_active = props.is_active.clone();

        Callback::from(move |event: web_sys::SubmitEvent| {
            event.prevent_default();
            // Handle form submission logic with the `time` value
            console::log_1(&format!("Submitting time: {}", *local_time).into());
            let local_time = local_time.clone();
            let time = time.clone();
            let is_active = is_active.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match api_calls::set_time(TimePayload::new(local_time.to_string())).await {
                    Ok(_) => {
                        time.set((*local_time).clone());
                        is_active.set(true);
                        console::log_1(&format!("Time set: {}", *local_time).into());
                    }
                    Err(e) => {
                        console::log_1(&format!("Could not submit time: {}", e).into());
                    }
                };
            });
        })
    };

    let onclick = {
        let time = props.time.clone();
        let is_active = props.is_active.clone();

        Callback::from(move |_| {
            let time = time.clone();
            let is_active = is_active.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api_calls::unset_time().await {
                    Ok(_) => {
                        is_active.set(false);
                        time.set("00:00".to_string());
                        console::log_1(&"Time unset".into());
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Can't delete time: {}", e).into());
                    }
                }
            });
        })
    };

    html! {
        <div class={classes!("timer-container")}>
            <form onsubmit={onsubmit}>
                <div class={classes!("timer-left")}>
                    <input type={"time"} id={"time-input"} value={(*local_time).clone()} oninput={oninput} />
                </div>
                <div class={classes!("timer-right")}>
                    <button type="submit" class={classes!("my-btn", "first")}>{ "Set" }</button>
                    <button {onclick} type="button" class={classes!("my-btn", "red")}>{ "Unset" }</button>
                </div>
            </form>
        </div>
    }
}
