use std::path::PathBuf;

use axum::{
    body::{Body, Bytes},
    extract::{Path, State},
    http::{Request, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use sqlx::{types::Uuid, PgPool};
use tower::util::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    not_found: ServeFile,
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:postgres@localhost:5432/postgres"
    )]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed");

    let not_found = ServeFile::new(static_folder.join("404.html"));

    let router = Router::new()
        .nest_service(
            "/",
            ServeDir::new(&static_folder).not_found_service(not_found.clone()),
        )
        .route("/djots/:uuid", get(read_post))
        .route("/new-djot", post(create_post))
        .with_state(AppState { pool, not_found });

    Ok(router.into())
}

async fn read_post(
    State(state): State<AppState>,
    Path(p): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let res_404 = (StatusCode::NOT_FOUND, state.not_found.oneshot(req).await).into_response();
    let Ok(uuid) = p.parse::<Uuid>() else {
        println!("INVALID UUID {:?}", p);
        return res_404;
    };
    let Ok(djot) = sqlx::query_as!(Djot, "SELECT html FROM djots WHERE id=$1", uuid)
        .fetch_one(&state.pool)
        .await
    else {
        println!("NO DJOT WITH UUID FOUND");
        return res_404;
    };
    (StatusCode::OK, Html(djot.html)).into_response()
}

async fn create_post(State(state): State<AppState>, body: Bytes) -> impl IntoResponse {
    let uuid = uuid7::new_v7();
    let result = sqlx::query!(
        "INSERT INTO djots(id, html) VALUES ($1, $2)",
        uuid,
        body.to_vec()
    )
    .execute(&state.pool)
    .await;
    match result {
        Ok(_) => (StatusCode::CREATED, uuid.to_string()).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
    }
}

struct Djot {
    html: Bytes,
}
