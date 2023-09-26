
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = * counter + 1;
            counter.set(value);
        }
    };

    let handle_input = Callback::from(|_| {});

    html! {
        <main>
            <div>
                {"SEARHS"}
            </div>
            <div>
                <input type="text" oninput={handle_input}/>
            </div>
            <div>
                {"Button Counter"}
                <button {onclick}>{ "+1" }</button>
                <p>{ * counter }</p>
            </div>
        </main>
    }
}


