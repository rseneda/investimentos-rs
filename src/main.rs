use axum::{Router, http::Method};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod routes;
mod db;
mod auth;

pub use db::AppState;

#[tokio::main]
async fn main() {
    // Carrega variáveis do .env
    dotenv().ok();

    // Configura logs
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "investimentos_rs=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Conecta ao banco de dados
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL deve estar definida no .env");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");

    tracing::info!("✅ Conectado ao PostgreSQL");

    // Roda as migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Falha ao rodar migrations");

    tracing::info!("✅ Migrations executadas");

    // Configura CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    // Monta o estado compartilhado
    let state = AppState { pool };

    // Monta as rotas
    let app = Router::new()
        .merge(routes::ativos::rotas())
        .merge(routes::auth::rotas())
        .layer(cors)
        .with_state(state);

    // Inicia o servidor
    let porta = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let endereco = format!("0.0.0.0:{}", porta);

    tracing::info!("🚀 Servidor rodando em http://{}", endereco);

    let listener = tokio::net::TcpListener::bind(&endereco).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
