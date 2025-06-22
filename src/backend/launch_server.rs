use dioxus::prelude::*;

#[cfg(feature = "server")]
use std::env; 
#[cfg(feature = "server")]
use sqlx::mysql::MySqlPoolOptions;

use crate::components::App;

#[cfg(feature = "server")]
pub async fn launch_server() {
    use std::env;

    dotenvy::dotenv().unwrap();
    dioxus::logger::initialize_default();

    let mysql_user = env::var("MYSQL_USER").unwrap();
    let mysql_password = env::var("MYSQL_PASSWORD").unwrap();
    let mysql_host = env::var("MYSQL_HOST").unwrap();
    let mysql_port = env::var("MYSQL_PORT").unwrap();

    let mysql_connect_str = format!("mysql://{}:{}@{}:{}/jlawyerdb",
        mysql_user,
        mysql_password,
        mysql_host,
        mysql_port
    );

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&mysql_connect_str)
        .await
        .unwrap();

    let cases = sqlx::query("SELECT * FROM cases")
        .fetch_one(&pool)
        .await
        .unwrap();

    print!("{:#?}", cases);

    let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .into_make_service();
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}