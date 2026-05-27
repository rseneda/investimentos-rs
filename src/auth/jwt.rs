use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use uuid::Uuid;
use chrono::Utc;

/// Dados armazenados dentro do token JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,   // user_id
    pub exp: usize,    // quando expira (timestamp Unix)
    pub iat: usize,    // quando foi criado
}

/// Cria um token JWT para o usuário
pub fn criar_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let agora = Utc::now().timestamp() as usize;
    let expira_em = agora + 7 * 24 * 60 * 60; // 7 dias

    let claims = Claims {
        sub: user_id.to_string(),
        iat: agora,
        exp: expira_em,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET não definido");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Verifica e decodifica um token JWT
pub fn verificar_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET não definido");

    let dados = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(dados.claims)
}

// ─── Extrator do Axum ────────────────────────────────────────────────────────
// Permite usar `claims: Claims` como parâmetro nas rotas protegidas

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Pega o header Authorization: Bearer <token>
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| erro_auth("Token não fornecido"))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| erro_auth("Formato inválido. Use: Bearer <token>"))?;

        let claims = verificar_token(token)
            .map_err(|_| erro_auth("Token inválido ou expirado"))?;

        Ok(claims)
    }
}

fn erro_auth(msg: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "erro": msg })),
    )
        .into_response()
}
