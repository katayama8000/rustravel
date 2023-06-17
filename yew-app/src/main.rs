use components::header::Header;
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Header />
    }
}

fn main() {
    yew::start_app::<App>();
}
