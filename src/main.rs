use dioxus::prelude::*;
//use serde::{Deserialize, Serialize};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        RLawyer {}
    }
}

#[component]
pub fn RLawyer() -> Element {
    let blablubb = use_server_future(get_server_data)?;

    rsx! {
        h1 { "Hello," }
        div { "{blablubb.unwrap().unwrap()}" }
    }
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("World!".to_string())
}
