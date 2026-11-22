mod handlers;
mod models;
mod repository;
mod state;

use axum::{
    routing::{get, post},
    Router,
};

use handlers::{
    add_user, edit_user_page, index, new_user_page, remove_user, update_user,
};
use state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::init().await;

    let app = Router::new()
        .route("/", get(index))
        .route("/user/new", get(new_user_page))
        .route("/user", post(add_user))
        .route("/user/{id}/edit", get(edit_user_page))
        .route("/user/{id}/update", post(update_user))
        .route("/user/{id}/delete", post(remove_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}