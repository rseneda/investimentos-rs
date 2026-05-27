use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::json;

use crate::db::{queries, AppState};
use crate::auth::criar_token;
use crate::models::user::{CriarUser, LoginUser, AuthResponse};

/// Registra as rotas de autenticação
pub fn rotas() -> Router<AppState> {
    Router::new()
        .route("/auth/cadastro", post(cadastro))
        .route("/auth/login",   post(login))
}

// ─── POST /auth/cadastro ─────────────────────────────────────────────────────
async fn cadastro(
    State(state): State<AppState>,
    Json(dados): Json<CriarUser>,
) -> impl IntoResponse {
    // Verifica se e-mail já existe
    if let Ok(Some(_)) = queries::buscar_user_por_email(&state.pool, &dados.email).await {
        return (
            StatusCode::CONFLICT,
            Json(json!({ "erro": "E-mail já cadastrado" })),
        ).into_response();
    }

    // Gera hash da senha
    let senha_hash = match hash(&dados.senha, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "erro": "Erro ao processar senha" })),
        ).into_response(),
    };

    // Cria usuário no banco
    match queries::criar_user(&state.pool, dados, senha_hash).await {
        Ok(user) => {
            let token = criar_token(user.id).unwrap();
            (
                StatusCode::CREATED,
                Json(json!(AuthResponse { token, user })),
            ).into_response()
        }
        Err(e) => {
            tracing::error!("Erro ao criar usuário: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "erro": "Erro interno ao criar usuário" })),
            ).into_response()
        }
    }
}

// ─── POST /auth/login ─────────────────────────────────────────────────────────
async fn login(
    State(state): State<AppState>,
    Json(dados): Json<LoginUser>,
) -> impl IntoResponse {
    // Busca usuário pelo e-mail
    let user = match queries::buscar_user_por_email(&state.pool, &dados.email).await {
        Ok(Some(u)) => u,
        Ok(None) => return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "erro": "E-mail ou senha incorretos" })),
        ).into_response(),
        Err(_) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "erro": "Erro interno" })),
        ).into_response(),
    };

    // Verifica senha
    let senha_ok = verify(&dados.senha, &user.senha_hash).unwrap_or(false);
    if !senha_ok {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "erro": "E-mail ou senha incorretos" })),
        ).into_response();
    }

    // Gera token e retorna
    let token = criar_token(user.id).unwrap();
    Json(json!(AuthResponse { token, user })).into_response()
}
