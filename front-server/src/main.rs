use yew::prelude::*;

mod api;
mod components;
mod model;

use components::on_off_button::OnOffButton;
use components::set_timer::SetTimer;
use components::title::Title;

#[function_component]
fn App() -> Html {
    let time = use_state(|| "00:00".to_string());
    let is_active = use_state(|| false);

    html! {
        <div>
            <Title time={time.clone()} is_active={is_active.clone()}/>
            <div class={classes!("main-container")}>
                <SetTimer time={time.clone()} is_active={is_active.clone()}/>
                <OnOffButton />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
