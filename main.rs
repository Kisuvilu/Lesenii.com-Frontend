use log::info;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
       <p class="text-white">{ "Rendered" }</p>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    info!("Starting app...");
    yew::Renderer::<App>::new().render();
}