# 🦀 Rust API de Treinamento com Axum

Este repositório contém um projeto de backend em Rust com o framework [Axum](https://docs.rs/axum), utilizado como exercício prático para aprender e aplicar conceitos de:

- APIs RESTful
- Gerenciamento de estado com `Arc<Mutex<_>>`
- Modularização de código em Rust
- Boas práticas com Type Safety, Enums e Structs

---

## 🚀 Como executar o projeto

**Pré-requisitos:**

- [Rust instalado](https://www.rust-lang.org/tools/install)

**Passos:**

1. Clone o repositório:

```bash
git clone https://github.com/fagundesjg/training-rust.git
cd training-rust
```

2. Execute a aplicação:

```bash
cargo run
```

A API ficará disponível por padrão em:  
`http://localhost:3000`

---

## 📁 Estrutura do Projeto

- `main.rs` — ponto de entrada da aplicação
- `routes/` — definição de rotas organizadas por domínio
- `handlers/` — lógica de negócio dos endpoints
- `models/` — structs da aplicação (ex: User, Gender)
- `dtos/` — structs de entrada (CreateUser, UpdateUser)
- `state/` — definição de `AppState` e subestados como `UserState`

---

## ✨ Funcionalidades

- **CRUD de Usuários**

  - Criar usuário (`POST /users`)
  - Listar todos os usuários (`GET /users`)
  - Buscar usuário por ID (`GET /users/:id`)
  - Atualizar parcialmente (`PUT /users/:id`)
  - Deletar usuário (`DELETE /users/:id`)

- **Armazenamento em memória**

  - Os usuários são mantidos em um `Vec<User>` protegido por `Arc<Mutex<_>>`

- **Enum para gênero**
  - Aceita: `male`, `female`, `other` (via Serde `rename_all = "lowercase"`)

---

## 📦 Exemplo de JSON - Criação de Usuário

POST `/users`

```json
{
  "name": "João Silva",
  "birth_date": "1990-05-20",
  "gender": "male"
}
```

---

## 🔧 Melhorias Futuras

- Persistência com banco de dados (ex: SQLite, PostgreSQL)
- Testes automatizados com `tokio::test`
- Middleware de autenticação
- Paginação e filtros na listagem de usuários

---

## 📄 Licença

Este projeto é apenas para fins de estudo e aprendizado.  
Sinta-se livre para utilizar, adaptar e melhorar!
