use serde::Serialize;
use crate::models::ativo::Ativo;

/// Resumo consolidado da carteira do usuário
#[derive(Debug, Serialize)]
pub struct ResumoCarteira {
    pub total_investido: f64,
    pub total_ativos:    usize,
    pub por_tipo:        Vec<GrupoPorTipo>,
    pub ativos:          Vec<Ativo>,
}

/// Agrupamento de ativos por tipo (ação, FII etc.)
#[derive(Debug, Serialize)]
pub struct GrupoPorTipo {
    pub tipo:       String,
    pub quantidade: usize,
    pub valor:      f64,
    pub percentual: f64,
}

impl ResumoCarteira {
    /// Monta o resumo a partir de uma lista de ativos
    pub fn from_ativos(ativos: Vec<Ativo>) -> Self {
        let total_investido: f64 = ativos.iter().map(|a| a.valor_total()).sum();
        let total_ativos = ativos.len();

        let tipos = ["acao", "fii", "renda_fixa", "cripto"];
        let por_tipo = tipos.iter().filter_map(|&tipo| {
            let grupo: Vec<&Ativo> = ativos.iter().filter(|a| a.tipo == tipo).collect();
            if grupo.is_empty() { return None; }

            let valor: f64 = grupo.iter().map(|a| a.valor_total()).sum();
            let percentual = if total_investido > 0.0 {
                (valor / total_investido) * 100.0
            } else { 0.0 };

            Some(GrupoPorTipo {
                tipo: tipo.to_string(),
                quantidade: grupo.len(),
                valor,
                percentual,
            })
        }).collect();

        ResumoCarteira { total_investido, total_ativos, por_tipo, ativos }
    }
}
