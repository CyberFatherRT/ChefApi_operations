mod components;

use components::Banner;
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="absolute top-0 left-0 w-full h-full flex">
            <Banner/>
            <div class="float-left bg-black">
                <div class="absolute border border-lime-600 bottom-0 top-[30px]" style="width: calc(25% - 2px)">
                    <div>Operations</div>
                </div>
            </div>
        </div>
    }
}
