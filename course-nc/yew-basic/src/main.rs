
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let header_text = "WEB APP - YEW (RUST)".to_string();
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class={classes!("container")}>
            <h4>{header_text}</h4>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
