use leptos::{ev::SubmitEvent, html::Input, *};

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Maksim".to_string());

    let input_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element().expect().value();
        set_name(value)
    };

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
