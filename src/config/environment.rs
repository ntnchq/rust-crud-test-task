use std::env;

/// Загружает переменные окружения из `.env`
pub fn load_env() {
    dotenvy::dotenv().ok();
}

/// Получает строку подключения к базе данных
pub fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
