//! main.rs
use sqlx::PgPool;
use std::net::TcpListener;
use env_logger::Env;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_string = configuration.database.connection_string();
    let db_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)
        .unwrap_or_else(|_| panic!("Failed to bind port {}", configuration.application_port));
    println!("Listening to port {}", configuration.application_port);
    run(listener, db_pool)?.await
}
