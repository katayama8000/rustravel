use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World from Yew" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
