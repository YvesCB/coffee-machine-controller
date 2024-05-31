use web_sys::console;
use yew::prelude::*;

mod api;
mod components;

use components::on_off_button::OnOffButton;
use components::set_timer::SetTimer;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    props.on_name_entry.emit(String::from("Bob"));

    html! {
        <p>{ "Hello" }</p>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <SetTimer />
            <OnOffButton />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
