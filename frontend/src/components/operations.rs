use leptos::*;

#[component]
fn Title(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="title no-select">
            Operations
        </div>
    }
}

#[component]
pub fn OperationsList(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="absolute w-full bottom-0 top-[30px]" style="width: calc(20% - 2px)">
            <Title/>
        </div>
    }
}
