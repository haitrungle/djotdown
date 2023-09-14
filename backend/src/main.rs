use std::collections::HashMap;
use std::path::PathBuf;

use axum::{
    Router,
    body::Bytes,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    response::{IntoResponse, Html},
};
use std::sync::{Mutex, OnceLock};
use tower_http::services::{ServeDir, ServeFile};

use sqlx::PgPool;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
    /* #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.POSTGRES_PASSWORD}@localhost:{secrets.POSTGRES_PORT}/postgres"
    )] pool: PgPool, */
) -> shuttle_axum::ShuttleAxum {
    let router =
        Router::new().nest_service(
            "/",
            ServeDir::new(static_folder).not_found_service(ServeFile::new("index.html")),
        ).nest_service(
            "/new-djot",
            post(create_post)
        ).nest_service(
            "/djots/:uuid",
            get(read_post)
        );

    Ok(router.into())
}

async fn read_post(Path(uuid): Path<String>) -> Html<Bytes> {
    let db = db().lock().expect("Failed to get database");
    let content = db.get(&uuid);
    let empty = Bytes::new();
    Html(content.unwrap_or(&empty).to_owned())
}

async fn create_post(body: Bytes) -> impl IntoResponse {
    let uuid = uuid7::uuid7().encode().to_string();
    let mut db = db().lock().expect("Failed to get database");
    db.insert(uuid.clone(), body);
    (StatusCode::CREATED, uuid)
}

fn db() -> &'static Mutex<HashMap<String, Bytes>> {
    static HASHMAP: OnceLock<Mutex<HashMap<String, Bytes>>> = OnceLock::new();
    HASHMAP.get_or_init(|| Mutex::new(HashMap::new()))
}