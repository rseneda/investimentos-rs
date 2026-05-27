use sqlx::PgPool;
use uuid::Uuid;
use crate::models::ativo::{Ativo, CriarAtivo};
use crate::models::user::{User, CriarUser};

// ─── ATIVOS ──────────────────────────────────────────────────────────────────

/// Busca todos os ativos de um usuário
pub async fn listar_ativos(pool: &PgPool, user_id: Uuid) -> Result<Vec<Ativo>, sqlx::Error> {
    sqlx::query_as!(
        Ativo,
        r#"SELECT id, user_id, ticker, nome, quantidade, preco_medio, tipo, criado_em
           FROM ativos
           WHERE user_id = $1
           ORDER BY criado_em DESC"#,
        user_id
    )
    .fetch_all(pool)
    .await
}

/// Busca um ativo específico pelo id
pub async fn buscar_ativo(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Option<Ativo>, sqlx::Error> {
    sqlx::query_as!(
        Ativo,
        r#"SELECT id, user_id, ticker, nome, quantidade, preco_medio, tipo, criado_em
           FROM ativos
           WHERE id = $1 AND user_id = $2"#,
        id,
        user_id
    )
    .fetch_optional(pool)
    .await
}

/// Cria um novo ativo para o usuário
pub async fn criar_ativo(pool: &PgPool, user_id: Uuid, dados: CriarAtivo) -> Result<Ativo, sqlx::Error> {
    sqlx::query_as!(
        Ativo,
        r#"INSERT INTO ativos (id, user_id, ticker, nome, quantidade, preco_medio, tipo)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           RETURNING id, user_id, ticker, nome, quantidade, preco_medio, tipo, criado_em"#,
        Uuid::new_v4(),
        user_id,
        dados.ticker.to_uppercase(),
        dados.nome,
        dados.quantidade,
        dados.preco_medio,
        dados.tipo
    )
    .fetch_one(pool)
    .await
}

/// Atualiza quantidade e/ou preço médio de um ativo
pub async fn atualizar_ativo(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    quantidade: Option<f64>,
    preco_medio: Option<f64>,
) -> Result<Option<Ativo>, sqlx::Error> {
    sqlx::query_as!(
        Ativo,
        r#"UPDATE ativos
           SET quantidade  = COALESCE($3, quantidade),
               preco_medio = COALESCE($4, preco_medio)
           WHERE id = $1 AND user_id = $2
           RETURNING id, user_id, ticker, nome, quantidade, preco_medio, tipo, criado_em"#,
        id,
        user_id,
        quantidade,
        preco_medio
    )
    .fetch_optional(pool)
    .await
}

/// Remove um ativo da carteira
pub async fn deletar_ativo(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
    let resultado = sqlx::query!(
        "DELETE FROM ativos WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(resultado.rows_affected() > 0)
}

// ─── USUÁRIOS ────────────────────────────────────────────────────────────────

/// Cria um novo usuário (senha já deve vir com hash)
pub async fn criar_user(pool: &PgPool, dados: CriarUser, senha_hash: String) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"INSERT INTO users (id, nome, email, senha_hash)
           VALUES ($1, $2, $3, $4)
           RETURNING id, nome, email, senha_hash, criado_em"#,
        Uuid::new_v4(),
        dados.nome,
        dados.email.to_lowercase(),
        senha_hash
    )
    .fetch_one(pool)
    .await
}

/// Busca um usuário pelo e-mail
pub async fn buscar_user_por_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, nome, email, senha_hash, criado_em FROM users WHERE email = $1",
        email.to_lowercase()
    )
    .fetch_optional(pool)
    .await
}
