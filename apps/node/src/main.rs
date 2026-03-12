mod config;
mod db;
mod error;
mod models;
mod routes;
mod validation;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "osp_node=info,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;

    tracing::info!("Connecting to database: {}", config.database_url);
    let db = db::init(&config.database_url).await?;

    let state = AppState { db, config: config.clone() };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // ── OSP public endpoints ──
        .route("/.well-known/osp",  get(routes::osp::well_known))
        .route("/shop.json",        get(routes::osp::shop_manifest))
        .route("/products.json",    get(routes::osp::products_listing))
        // ── Admin API ──
        .route("/admin/shop",               get(routes::admin::get_shop))
        .route("/admin/shop",               patch(routes::admin::update_shop))
        .route("/admin/products",           get(routes::admin::list_products))
        .route("/admin/products",           post(routes::admin::create_product))
        .route("/admin/products/:id",       patch(routes::admin::update_product))
        .route("/admin/products/:id",       delete(routes::admin::delete_product))
        // ── Admin UI ──
        .route("/admin",                    get(serve_admin_ui))
        .layer(cors)
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("OSP Node listening on http://{}", addr);
    tracing::info!("Admin UI → http://{}/admin", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn serve_admin_ui() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("admin.html"))
}
