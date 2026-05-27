-- Migration: 0001_init
-- Cria as tabelas iniciais do sistema de carteira de investimentos

-- Extensão para gerar UUIDs
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ─── Tabela de usuários ────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS users (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nome        VARCHAR(100) NOT NULL,
    email       VARCHAR(255) NOT NULL UNIQUE,
    senha_hash  TEXT NOT NULL,
    criado_em   TIMESTAMP NOT NULL DEFAULT NOW()
);

-- ─── Tabela de ativos ──────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS ativos (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    ticker       VARCHAR(20)  NOT NULL,
    nome         VARCHAR(100) NOT NULL,
    quantidade   DOUBLE PRECISION NOT NULL CHECK (quantidade > 0),
    preco_medio  DOUBLE PRECISION NOT NULL CHECK (preco_medio > 0),
    tipo         VARCHAR(20) NOT NULL CHECK (tipo IN ('acao', 'fii', 'renda_fixa', 'cripto')),
    criado_em    TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Índices para performance
CREATE INDEX IF NOT EXISTS idx_ativos_user_id ON ativos(user_id);
CREATE INDEX IF NOT EXISTS idx_users_email    ON users(email);
