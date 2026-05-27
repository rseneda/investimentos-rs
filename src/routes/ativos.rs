use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use crate::auth::Claims;
use crate::db::{queries, AppState};
use crate::models::ativo::{AtualizarAtivo, CriarAtivo};
use crate::models::carteira::ResumoCarteira;

/// Registra as rotas de ativos (todas protegidas por JWT)
pub fn rotas() -> Router<AppState> {
    Router::new()
        .route("/ativos",          get(listar).post(criar))
        .route("/ativos/resumo",   get(resumo))
        .route("/ativos/:id",      get(buscar).put(atualizar).delete(deletar))
}

// ─── GET /ativos ──────────────────────────────────────────────────────────────
async fn listar(
    claims: Claims,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    match queries::listar_ativos(&state.pool, user_id).await {
        Ok(ativos) => Json(json!({ "ativos": ativos, "total": ativos.len() })).into_response(),
        Err(e) => {
            tracing::error!("Erro ao listar ativos: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "erro": "Erro ao buscar ativos" }))).into_response()
        }
    }
}

// ─── GET /ativos/resumo ───────────────────────────────────────────────────────
async fn resumo(
    claims: Claims,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    match queries::listar_ativos(&state.pool, user_id).await {
        Ok(ativos) => {
            let resumo = ResumoCarteira::from_ativos(ativos);
            Json(json!(resumo)).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "erro": "Erro ao montar resumo" })),
        ).into_response(),
    }
}

// ─── GET /ativos/:id ─────────────────────────────────────────────────────────
async fn buscar(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    match queries::buscar_ativo(&state.pool, id, user_id).await {
        Ok(Some(ativo)) => Json(json!(ativo)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({ "erro": "Ativo não encontrado" }))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "erro": "Erro interno" }))).into_response(),
    }
}

// ─── POST /ativos ─────────────────────────────────────────────────────────────
async fn criar(
    claims: Claims,
    State(state): State<AppState>,
    Json(dados): Json<CriarAtivo>,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    // Validações básicas
    if dados.quantidade <= 0.0 || dados.preco_medio <= 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "erro": "Quantidade e preço médio devem ser maiores que zero" })),
        ).into_response();
    }

    match queries::criar_ativo(&state.pool, user_id, dados).await {
        Ok(ativo) => (StatusCode::CREATED, Json(json!(ativo))).into_response(),
        Err(e) => {
            tracing::error!("Erro ao criar ativo: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "erro": "Erro ao criar ativo" }))).into_response()
        }
    }
}

// ─── PUT /ativos/:id ─────────────────────────────────────────────────────────
async fn atualizar(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dados): Json<AtualizarAtivo>,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    match queries::atualizar_ativo(&state.pool, id, user_id, dados.quantidade, dados.preco_medio).await {
        Ok(Some(ativo)) => Json(json!(ativo)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({ "erro": "Ativo não encontrado" }))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "erro": "Erro ao atualizar" }))).into_response(),
    }
}

// ─── DELETE /ativos/:id ──────────────────────────────────────────────────────
async fn deletar(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    match queries::deletar_ativo(&state.pool, id, user_id).await {
        Ok(true)  => Json(json!({ "mensagem": "Ativo removido com sucesso" })).into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, Json(json!({ "erro": "Ativo não encontrado" }))).into_response(),
        Err(_)    => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "erro": "Erro ao deletar" }))).into_response(),
    }
}
