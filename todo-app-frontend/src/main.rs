use leptos::prelude::*;

mod api;
mod components;
mod core;
use crate::components::Home;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let hero_image_url = option_env!("HERO_IMAGE_URL")
        .unwrap_or("https://picsum.photos/80")
        .to_string();
    dbg!(&hero_image_url);

    view! {
        <main>
            <Home hero_image_url=hero_image_url/>
        </main>
    }
}
