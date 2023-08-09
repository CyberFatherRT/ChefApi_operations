mod components;

use components::Banner;
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <Banner/>
    }
}
