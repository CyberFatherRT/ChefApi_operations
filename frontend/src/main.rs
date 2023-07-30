use yew::{function_component, html, Html, Properties, Renderer};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! { <>{"Am I loading? - "}{props.is_loading.clone()}</> }
}

// Then supply the prop
#[function_component]
fn App() -> Html {
    html! {<HelloWorld is_loading={true} />}
}


fn main() {
    Renderer::<App>::new().render();
}