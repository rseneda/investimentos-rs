use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

/// Representa um ativo financeiro na carteira
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ativo {
    pub id:           Uuid,
    pub user_id:      Uuid,
    pub ticker:       String,      // Ex: "PETR4", "BTC", "TESOURO-SELIC"
    pub nome:         String,
    pub quantidade:   f64,
    pub preco_medio:  f64,         // Preço médio de compra (R$)
    pub tipo:         String,      // "acao", "fii", "renda_fixa", "cripto"
    pub criado_em:    NaiveDateTime,
}

/// Payload para criar um novo ativo (enviado pelo frontend via JSON)
#[derive(Debug, Deserialize)]
pub struct CriarAtivo {
    pub ticker:      String,
    pub nome:        String,
    pub quantidade:  f64,
    pub preco_medio: f64,
    pub tipo:        String,
}

/// Payload para atualizar um ativo existente
#[derive(Debug, Deserialize)]
pub struct AtualizarAtivo {
    pub quantidade:  Option<f64>,
    pub preco_medio: Option<f64>,
}

impl Ativo {
    /// Calcula o valor total investido neste ativo
    pub fn valor_total(&self) -> f64 {
        self.quantidade * self.preco_medio
    }
}
