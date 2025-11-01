use leptos::prelude::*;

#[component]
pub fn HeroImage(image_url: String) -> impl IntoView {
    view! {
        <img
            src=image_url
        />
    }
}
