use crud_example::routes;
use crud_example::config::{database, environment};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Настройка логирования
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Настройка базы данных
    environment::load_env();
    let database_url = environment::database_url();
    let pool = database::create_pool(&database_url);

    // Подключение маршрутов
    let app = routes::create_routes().with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    tracing::info!("App configured and ready to run");
}
