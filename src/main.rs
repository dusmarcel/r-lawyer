use dioxus::prelude::*;
//use tokio;
//use axum;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(launch_server());
    #[cfg(not(feature = "server"))]
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
    let r = use_server_future(get_server_data)?;

    rsx! {
        h1 { "Hello," }
        div { "{r.unwrap().unwrap()}" }
    }
}

#[cfg(feature = "server")]
async fn launch_server() {
    dioxus::logger::initialize_default();
    let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .into_make_service();
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("World!".to_string())
}
