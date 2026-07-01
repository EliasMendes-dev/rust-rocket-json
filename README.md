# Rust Rocket JSON API

Uma API RESTful construída com Rust e Rocket que gerencia recursos e fornece autenticação JWT.

## 🚀 Visão geral

Esta aplicação demonstra uma arquitetura em camadas:
- `controllers` para rotas e respostas HTTP
- `dtos` para validação de entrada
- `servicos` para lógica de negócio
- `models` e `model_views` para modelos de domínio e respostas JSON

## 🧰 Tecnologias usadas

- Rust (Edition 2024)
- Rocket 0.5
- Serde / serde_json
- JSON Web Tokens (`jsonwebtoken`)
- Chrono
- Time

## 📦 Estrutura do projeto

```text
rust-rocket-json/
├── src/
│   ├── controllers/      # Endpoints HTTP
│   ├── dtos/             # Objetos de transferência de dados
│   ├── model_views/      # Estruturas de resposta
│   ├── models/           # Entidades e erros
│   ├── servicos/         # Lógica de negócio e JWT
│   └── main.rs           # Inicialização do Rocket
├── tests_curls/          # Scripts de teste com curl
├── Cargo.toml
├── Cargo.lock
├── Rocket.toml
└── README.md
```

## 🚀 Funcionalidades

- Lista todos os recursos
- Busca recurso por ID
- Cria, atualiza e exclui recursos
- Login de administrador com JWT
- Respostas JSON consistentes para sucesso e erro

## 🔧 Como executar

1. Certifique-se de ter o Rust instalado:

```bash
rustup toolchain install stable
```

2. Clone o repositório:

```bash
git clone <repo-url>
cd rust-rocket-json
```

3. Defina a variável de ambiente para o segredo JWT (opcional):

```bash
set SECRET_JWT=seu_seguro_secreto
```

> Se `SECRET_JWT` não for definido, o projeto usa a chave padrão `your_secret_key`.

4. Execute o projeto:

```bash
cargo run
```

5. Acesse a API em:

```text
http://localhost:8000
```

## 📌 Endpoints disponíveis

### GET /

Retorna uma mensagem de boas-vindas e endpoints disponíveis.

### POST /login

Autentica um administrador e retorna um token JWT.

Request body:

```json
{
  "email": "admin@teste.com",
  "senha": "123456"
}
```

Resposta de sucesso:

```json
{
  "id": 1,
  "nome": "Elias",
  "email": "admin@teste.com",
  "token": "...jwt..."
}
```

### GET /recursos

Lista todos os recursos.

### GET /recursos/<id>

Retorna os dados de um recurso por ID.

### POST /recursos

Cria um novo recurso.

Request body:

```json
{
  "titulo": "Título do recurso",
  "descricao": "Descrição do recurso"
}
```

### PUT /recursos/<id>

Atualiza um recurso existente.

### DELETE /recursos/<id>

Remove um recurso.

## 🧪 Testes com curl

Os scripts em `tests_curls/` permitem testar os endpoints:

- `get_home.sh`
- `get_recurso.sh`
- `get_por_id.sh`
- `post_recurso.sh`
- `put_recurso.sh`
- `delete_recurso.sh`

## Observações

- O login atualmente valida apenas o usuário fixo `elias@teste.com` com senha `123456`.
- A geração de token JWT usa o `SECRET_JWT` se estiver definido.
- As rotas de recursos não exigem autenticação no código atual.

## 📚 Dependências principais

- `rocket = { version = "0.5.1", features = ["json"] }`
- `serde = { version = "1.0.228", features = ["derive"] }`
- `serde_json = "1.0"`
- `time = "=0.3.35"`
- `chrono = "0.4"`
- `jsonwebtoken = "10.4.0"`
