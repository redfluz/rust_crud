use axum::{
    extract::{Form, Path, State},
    response::{Html, IntoResponse, Redirect},
};
use minijinja::{context, value::Serde};

use crate::{
    models::{User, UserForm},
    state::AppState,
};

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let users: Vec<User> = state.repository.list().await;

    let tmpl = state.env.get_template("index.html").unwrap();
    let rendered = tmpl
        .render(context!(
            users => Serde(&users),
        ))
        .unwrap_or_default();

    Html(rendered)
}

pub async fn new_user_page(State(state): State<AppState>) -> impl IntoResponse {
    let editing: Option<User> = None;

    let tmpl = state.env.get_template("form.html").unwrap();
    let rendered = tmpl
        .render(context!(
            editing => Serde(&editing),
        ))
        .unwrap_or_default();

    Html(rendered)
}

pub async fn edit_user_page(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let editing: Option<User> = state.repository.find_by_id(id).await;

    if editing.is_none() {
        return Redirect::to("/").into_response();
    }

    let tmpl = state.env.get_template("form.html").unwrap();
    let rendered = tmpl
        .render(context!(
            editing => Serde(&editing),
        ))
        .unwrap_or_default();

    Html(rendered).into_response()
}

pub async fn add_user(
    State(state): State<AppState>,
    Form(input): Form<UserForm>,
) -> Redirect {
    let name = input.name.trim();
    if !name.is_empty() {
        state.repository.add(name, input.age).await;
    }
    Redirect::to("/")
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Form(input): Form<UserForm>,
) -> Redirect {
    let name = input.name.trim();
    if !name.is_empty() {
        state.repository.update(id, name, input.age).await;
    }
    Redirect::to("/")
}

pub async fn remove_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Redirect {
    state.repository.remove(id).await;
    Redirect::to("/")
}