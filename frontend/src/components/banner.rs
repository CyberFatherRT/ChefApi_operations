use leptos::*;

#[component]
pub fn Banner(cx: Scope) -> impl IntoView {
    view! {cx,
        <div id="banner" class="bg-[#DFF0D8]">
            <div>
                <span class="material-symbols-outlined">download</span>
            </div>
        </div>
    }
}
