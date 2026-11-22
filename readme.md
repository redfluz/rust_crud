# User CRUD (Axum + SQLx + MiniJinja)

A modular Rust web application for managing users with **Axum**, **SQLx** (MySQL), and **MiniJinja** server-side HTML templates, structured around manual dependency injection via `AppState` and a `UserRepository`.

## Architecture & Project Structure

```text
.
├── Cargo.toml
├── .env
├── templates/
│   ├── index.html      # List page (GET /)
│   └── form.html       # Add / Edit form page (GET /user/new, GET /user/{id}/edit)
└── src/
    ├── main.rs         # Router setup and server startup
    ├── state.rs        # Composition root / DI container (AppState)
    ├── models.rs       # Data structs (User, UserForm)
    ├── repository.rs   # Data access layer owning MySqlPool (UserRepository)
    └── handlers.rs     # HTTP route handlers and MiniJinja rendering
```

### Design Highlights
* **Manual Dependency Injection:** `AppState::init()` creates the `MySqlPool`, wraps it in `UserRepository::new(pool)`, initializes the MiniJinja `Environment`, and shares `AppState` across routes via `.with_state(state)`.
* **Clear Layer Separation:** Handlers call methods on `state.repository` (`list`, `find_by_id`, `add`, `update`, `remove`) rather than executing raw SQL queries or passing connection pools manually to every helper.
* **Two-Page Flow + Post/Redirect/Get (PRG):** Listing lives on `/` (`index.html`), creating/editing lives on `/user/new` and `/user/{id}/edit` (`form.html`), and write actions (`POST`) redirect back to `/` to prevent duplicate form submissions on refresh.

## Routes

| Method | Path | Handler | Description |
| :--- | :--- | :--- | :--- |
| `GET` | `/` | `index` | Renders `index.html` with all users |
| `GET` | `/user/new` | `new_user_page` | Renders `form.html` in create mode |
| `POST` | `/user` | `add_user` | Inserts a new user and redirects to `/` |
| `GET` | `/user/{id}/edit` | `edit_user_page` | Renders `form.html` pre-filled with user `{id}` |
| `POST` | `/user/{id}/update` | `update_user` | Updates user `{id}` and redirects to `/` |
| `POST` | `/user/{id}/delete` | `remove_user` | Deletes user `{id}` and redirects to `/` |

## Prerequisites & Setup

1. **Create the MySQL table:**
   ```sql
   CREATE TABLE users (
       id INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
       name VARCHAR(255) NOT NULL,
       age INT UNSIGNED NOT NULL
   );
   ```

2. **Configure `.env`:**
   ```env
   DATABASE_URL=mysql://root:password@127.0.0.1:3306/your_database
   ```

3. **Run the application:**
   ```bash
   cargo run
   ```

The server starts at `http://127.0.0.1:3000`.