use web_sys::console;
use yew::prelude::*;

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
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let on_name_entry: Callback<String> = Callback::from(move |name: String| {
        let greeting = format!("Hello {}", &name);
        console::log_1(&greeting.into());
    });

    html! {
        <div>
            <HelloWorld {on_name_entry} />
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
