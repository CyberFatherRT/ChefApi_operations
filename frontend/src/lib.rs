mod components;

use components::{Banner, OperationsList};
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <Banner/>
        // <OperationsList/>
    }
}
