# 💰 InvestRS — Carteira de Investimentos em Rust

Aplicação full-stack de gerenciamento de carteira de investimentos, desenvolvida em **Rust** com foco em performance, segurança e boas práticas de desenvolvimento moderno.

## 🚀 Tecnologias

| Camada | Tecnologia | Função |
|--------|-----------|--------|
| Servidor | [Axum](https://github.com/tokio-rs/axum) | Framework web assíncrono |
| Async | [Tokio](https://tokio.rs) | Runtime assíncrono |
| Banco | [SQLx](https://github.com/launchbadge/sqlx) + PostgreSQL | Persistência com queries verificadas em compile-time |
| Auth | bcrypt + JWT | Hash de senha e autenticação stateless |
| Templates | [Askama](https://djc.github.io/askama/) | HTML renderizado no servidor |
| Serialização | Serde + serde_json | JSON type-safe |

## 📁 Estrutura do Projeto

```
investimentos-rs/
├── Cargo.toml               # Dependências
├── .env                     # Variáveis de ambiente
├── migrations/
│   └── 0001_init.sql        # Schema do banco
├── static/
│   └── style.css            # Estilos do dashboard
└── src/
    ├── main.rs              # Ponto de entrada
    ├── models/
    │   ├── ativo.rs         # Struct Ativo + payloads
    │   ├── carteira.rs      # Resumo da carteira
    │   └── user.rs          # Struct User + payloads
    ├── routes/
    │   ├── ativos.rs        # CRUD de ativos (protegido por JWT)
    │   └── auth.rs          # Cadastro e login
    ├── db/
    │   └── queries.rs       # Queries SQL (SQLx)
    └── auth/
        └── jwt.rs           # Criação e validação de tokens
```

## 🔌 Endpoints da API

### Autenticação
| Método | Rota | Descrição |
|--------|------|-----------|
| POST | `/auth/cadastro` | Cria novo usuário |
| POST | `/auth/login` | Login e retorno do token JWT |

### Ativos (requer `Authorization: Bearer <token>`)
| Método | Rota | Descrição |
|--------|------|-----------|
| GET | `/ativos` | Lista todos os ativos do usuário |
| GET | `/ativos/resumo` | Resumo consolidado da carteira |
| GET | `/ativos/:id` | Busca um ativo específico |
| POST | `/ativos` | Adiciona novo ativo |
| PUT | `/ativos/:id` | Atualiza quantidade/preço |
| DELETE | `/ativos/:id` | Remove um ativo |

## ⚙️ Como rodar localmente

### Pré-requisitos
- Rust (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- PostgreSQL rodando localmente
- `sqlx-cli` (`cargo install sqlx-cli`)

### Passo a passo

```bash
# 1. Clone o repositório
git clone https://github.com/seu-usuario/investimentos-rs
cd investimentos-rs

# 2. Configure as variáveis de ambiente
cp .env.example .env
# Edite o .env com sua DATABASE_URL e JWT_SECRET

# 3. Crie o banco e rode as migrations
createdb investimentos
sqlx migrate run

# 4. Compile e rode
cargo run
```

O servidor sobe em `http://localhost:3000` 🚀

## 📬 Exemplo de uso com curl

```bash
# Cadastro
curl -X POST http://localhost:3000/auth/cadastro \
  -H "Content-Type: application/json" \
  -d '{"nome":"João","email":"joao@email.com","senha":"senha123"}'

# Login
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"joao@email.com","senha":"senha123"}'

# Adicionar ativo (use o token retornado no login)
curl -X POST http://localhost:3000/ativos \
  -H "Authorization: Bearer SEU_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"ticker":"PETR4","nome":"Petrobras","quantidade":100,"preco_medio":38.50,"tipo":"acao"}'

# Resumo da carteira
curl http://localhost:3000/ativos/resumo \
  -H "Authorization: Bearer SEU_TOKEN"
```

## 🧠 Conceitos aplicados

- **Ownership e Borrowing** — gerenciamento de memória sem garbage collector
- **Async/Await** com Tokio — servidor não-bloqueante de alta performance  
- **Type safety** — erros detectados em tempo de compilação com SQLx
- **JWT stateless** — autenticação sem estado no servidor
- **Error handling** com `thiserror` e `Result<T, E>`

## 📄 Licença

MIT
