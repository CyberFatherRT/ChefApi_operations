use leptos::{component, view, IntoView, Scope};

#[component]
fn DownloadMeDaddy(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="ml-2.5">
            <a href="#" class="text-center">
                Download CyberKnight
                <i class="material-icons">file_download</i>
            </a>
        </div>
    }
}

#[component]
fn MiddleStuff(cx: Scope) -> impl IntoView {
    view! {cx,
        <span> - New is here! Read about the new features <a href=""></a> </span>
    }
}

#[component]
fn OptionAbout(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <a href="#">
                Settings
                <i class="material-icons">settings</i>
            </a>
            <a href="#">
                About / Support
                <i class="material-icons">help</i>
            </a>
        </div>
    }
}

#[component]
pub fn Banner(cx: Scope) -> impl IntoView {
    view! {cx,
        <div id="banner" class="flex flex-nowrap justify-between -mr-15 bg-[#DFF0D8] text-[#468847] absolute w-full h-7">
            <DownloadMeDaddy/>
            <MiddleStuff/>
            <OptionAbout/>
        </div>
    }
}
