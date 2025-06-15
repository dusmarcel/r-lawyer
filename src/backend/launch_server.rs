use dioxus::prelude::*;

use crate::components::App;

#[cfg(feature = "server")]
pub async fn launch_server() {
    dioxus::logger::initialize_default();
    let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .into_make_service();
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}