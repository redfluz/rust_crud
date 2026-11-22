use std::sync::Arc;

use minijinja::{path_loader, Environment};
use sqlx::mysql::MySqlPoolOptions;

use crate::repository::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub repository: UserRepository,
    pub env: Arc<Environment<'static>>,
}

impl AppState {
    pub async fn init() -> Self {
        let _ = dotenvy::dotenv();

        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env or environment");

        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("Failed to connect to MySQL");

        let repository = UserRepository::new(pool);

        let mut env = Environment::new();
        env.set_loader(path_loader("templates"));
        env.set_auto_reload(true);

        Self {
            repository,
            env: Arc::new(env),
        }
    }
}