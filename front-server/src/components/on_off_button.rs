use gloo_net::http::Request;
use web_sys::console;
use yew::prelude::*;

#[function_component]
pub fn OnOffButton() -> Html {
    let onclick = Callback::from(|_| {
        let message = String::from("trying to blink");
        console::log_1(&message.into());
        wasm_bindgen_futures::spawn_local(async move {
            let _ = Request::post("/api/coffee/toggle_on_off")
                .send()
                .await
                .unwrap();
        });
    });

    html! {
        <button {onclick} class={ classes!("on-off-btn", "my-btn")}>{ "Toggle On/Off" }</button>
    }
}
