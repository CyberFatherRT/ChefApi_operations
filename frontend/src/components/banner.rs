use leptos::{component, view, IntoView, Scope};

#[component]
fn DownloadMeDaddy(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="ml-2.5 text-center">
            <a href="#">
                Download CyberKnight
                <i class="material-icons">file_download</i>
            </a>
        </div>
    }
}

#[component]
fn MiddleStuff(cx: Scope) -> impl IntoView {
    view! {cx,
        <span>
            <a href="https://github.com/TheGodfatherru/CyberKnight/blob/master/CHANGELOG.md">
                "\u{2012} "<b>New version is here!</b>
            </a>
            Read about the new features
            <a href="https://github.com/TheGodfatherru/CyberKnight/blob/master/CHANGELOG.md">here</a>
        </span>
    }
}

#[component]
fn OptionAbout(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <a href="#"> Settings <i class="material-icons"> settings </i></a>
            <a href="#"> About / Support <i class="material-icons"> help </i></a>
        </div>
    }
}

#[component]
pub fn Banner(cx: Scope) -> impl IntoView {
    view! {cx,
         <div id="banner" class="flex flex-nowrap justify-between bg-[#252525] text-[#c0c0c0] w-full h-[30px]">
            <DownloadMeDaddy/>
            <MiddleStuff/>
            <OptionAbout/>
        </div>
    }
}
