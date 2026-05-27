use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

/// Usuário do sistema
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id:         Uuid,
    pub nome:       String,
    pub email:      String,
    #[serde(skip_serializing)] // nunca retorna a senha no JSON
    pub senha_hash: String,
    pub criado_em:  NaiveDateTime,
}

/// Payload de cadastro
#[derive(Debug, Deserialize)]
pub struct CriarUser {
    pub nome:  String,
    pub email: String,
    pub senha: String,
}

/// Payload de login
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub senha: String,
}

/// Resposta após login bem-sucedido
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user:  User,
}
