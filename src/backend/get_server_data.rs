use dioxus::prelude::*;

#[server]
pub async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("World!".to_string())
}
