use sqlx::MySqlPool;

use crate::models::User;

#[derive(Clone)]
pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Vec<User> {
        sqlx::query_as("SELECT id, name, age FROM users ORDER BY id ASC")
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default()
    }

    pub async fn find_by_id(&self, id: u32) -> Option<User> {
        sqlx::query_as("SELECT id, name, age FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .unwrap_or_default()
    }

    pub async fn add(&self, name: &str, age: u32) {
        let _ = sqlx::query("INSERT INTO users (name, age) VALUES (?, ?)")
            .bind(name)
            .bind(age)
            .execute(&self.pool)
            .await;
    }

    pub async fn update(&self, id: u32, name: &str, age: u32) {
        let _ = sqlx::query("UPDATE users SET name = ?, age = ? WHERE id = ?")
            .bind(name)
            .bind(age)
            .bind(id)
            .execute(&self.pool)
            .await;
    }

    pub async fn remove(&self, id: u32) {
        let _ = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await;
    }
}