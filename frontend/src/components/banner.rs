use leptos::*;

#[component]
pub fn Banner(cx: Scope) -> impl IntoView {
    view! {cx,
        <div id="banner" class="bg-[#DFF0D8]">
            <div>
                <i class="material-icons">download</i>
            </div>
        </div>
    }
}
