use sqlx::PgPool;

pub mod queries;

/// Estado compartilhado pela aplicação inteira
/// Contém o pool de conexões com o banco
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
