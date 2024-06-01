use web_sys::console;
use yew::prelude::*;

use crate::api::calls::toggle_button;

#[function_component]
pub fn OnOffButton() -> Html {
    let onclick = Callback::from(|_| {
        console::log_1(&"Sinding toggle".into());
        wasm_bindgen_futures::spawn_local(async move {
            match toggle_button().await {
                Ok(_) => {
                    console::log_1(&"Toggled".into());
                }
                Err(e) => {
                    console::log_1(&format!("Could not toggle: {}", e).into());
                }
            };
        });
    });

    html! {
        <button {onclick} class={ classes!("on-off-btn", "my-btn")}>{ "Toggle On/Off" }</button>
    }
}
