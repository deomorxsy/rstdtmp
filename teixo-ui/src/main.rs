use yew::prelude::*;
use std::env;

#[function_component(App)]
fn app() -> Html {
    html!{
        <>
        <h1 class="text-primary">{ "This is from the main.rs. Yew for React developers" }</h1>
        </>
    }
}

fn main() {
    //setting environment variable for backtrace display
    env::set_var("RUST_BACKTRACE", "full");
    yew::start_app::<App>();
}
