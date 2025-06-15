use dioxus::prelude::*;

use crate::backend::get_server_data::get_server_data;

#[component]
pub fn RLawyer() -> Element {
    let r = use_server_future(get_server_data)?;

    rsx! {
        h1 { "Hello," }
        div { "{r.unwrap().unwrap()}" }
    }
}