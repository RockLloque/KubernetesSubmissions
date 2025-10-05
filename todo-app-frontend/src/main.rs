use leptos::prelude::*;

mod components;
mod core;
use crate::components::Home;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <Home />
        </main>
    }
}
