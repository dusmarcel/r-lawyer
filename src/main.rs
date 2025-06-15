use dioxus::prelude::*;

mod backend;
#[cfg(feature = "server")]
use crate::backend::launch_server::launch_server;

mod components;

fn main() {
    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(launch_server());

    #[cfg(not(feature = "server"))]
    launch(components::App);
}