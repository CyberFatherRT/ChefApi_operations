use common::{Operations, Request, Response};
use leptos::*;
use serde_json::to_string;

mod components;
use components::*;

async fn fetch_api(text: String) -> String {
    let request = Request::new(
        Operations::ToBase64,
        String::from("en"),
        vec![String::from("")],
        text,
    );
    let res = reqwasm::http::Request::post(&format!("http://localhost:8081/api/ciphers"))
        .body(to_string(&request).unwrap())
        .send()
        .await
        .unwrap();
    println!("{:?}", res);
    res.json::<Response>().await.unwrap().output.unwrap()
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (input, set_input) = create_signal(cx, "Test".to_string());

    let response = create_local_resource(cx, input, fetch_api);

    let input_view = move || {
        response
            .read(cx)
            .map(|data| {
                view! {cx, <p>{data}</p>}
            })
            .collect_view(cx)
    };

    let operations: Vec<Operations> = Operations::create_vec();

    view! {cx,
        <Banner/>
        <div>
            <label>
                <input
                    type="text"
                    prop:value=move || input.get()
                    on:input=move |ev| {
                        let val = event_target_value(&ev).parse::<String>().unwrap();
                        set_input.set(val)
                    }
                />
            </label>
            {input_view}
        </div>
        <select>
            {operations.into_iter()
                .map(|operation| view! {cx, <option>{to_string(&operation).unwrap()}</option>})
                .collect_view(cx)}
            <option selected>"Привет"</option>
        </select>
    }
}
