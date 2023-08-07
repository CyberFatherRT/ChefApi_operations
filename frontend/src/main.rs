use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Maksim".to_string());

    view! { cx,
        <input type="text" prop:value=name
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
        />
        <p>"Name is: " {name}</p>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
