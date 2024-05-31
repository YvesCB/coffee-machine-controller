use yew::prelude::*;

use crate::model::*;

#[function_component]
pub fn Title(props: &TimeProp) -> Html {
    html! {
        <div class={classes!("title-container")}>
            <div>
                <h1 class={classes!("title-text")}>{"The currently set time is"}</h1>
                {
                    if *props.is_active {
                        html! { <h1 class={classes!("title-time")}>{(*props.time).clone()}</h1> }
                    } else {
                        html! { <h1 class={classes!("title-time")}>{"NONE"}</h1> }
                    }
                }
            </div>
        </div>
    }
}
