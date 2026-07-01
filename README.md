# Rust Rocket JSON API

Uma API RESTful construída com Rust e Rocket que gerencia recursos e fornece autenticação JWT.

## 🚀 Visão geral

Esta aplicação demonstra uma arquitetura em camadas:

- `controllers` para rotas e respostas HTTP
- `dtos` para validação de entrada
- `servicos` para lógica de negócio
- `models` e `model_views` para modelos de domínio e respostas JSON
- `middlewares` para autenticação e proteção das rotas

## 🧰 Tecnologias usadas

- Rust (Edition 2024)
- Rocket 0.5
- Serde / serde_json
- JSON Web Token (JWT)
- jsonwebtoken
- Chrono
- Time

## 📦 Estrutura do projeto

```text
rust-rocket-json/
├── src/
│   ├── controllers/      # Endpoints HTTP
│   ├── dtos/             # Objetos de transferência de dados
│   ├── middlewares/      # Middleware de autenticação JWT
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
- Cria recursos
- Atualiza recursos
- Exclui recursos
- Login de administrador
- Geração de token JWT
- Proteção das rotas através de middleware
- Respostas JSON consistentes para sucesso e erro

## 🔧 Como executar

### 1. Instale o Rust

```bash
rustup toolchain install stable
```

### 2. Clone o repositório

```bash
git clone <repo-url>
cd rust-rocket-json
```

### 3. (Opcional) Defina a chave secreta do JWT

Windows (CMD):

```cmd
set SECRET_JWT=seu_segredo
```

Windows (PowerShell):

```powershell
$env:SECRET_JWT="seu_segredo"
```

Linux / macOS / Git Bash:

```bash
export SECRET_JWT=seu_segredo
```

> Caso `SECRET_JWT` não seja definida, será utilizada a chave padrão `your_secret_key`.

### 4. Execute a aplicação

```bash
cargo run
```

### 5. Acesse

```text
http://localhost:8000
```

---

# 🔐 Autenticação

A API utiliza JWT (JSON Web Token) para autenticar usuários e proteger as rotas de recursos.

## Login

Endpoint:

```http
POST /login
```

Body:

```json
{
    "email": "elias@teste.com",
    "senha": "123456"
}
```

Resposta:

```json
{
    "id": 1,
    "nome": "Elias",
    "email": "elias@teste.com",
    "token": "<jwt>"
}
```

Após obter o token, envie-o no cabeçalho das requisições protegidas:

```http
Authorization: Bearer <token>
```

Exemplo:

```bash
curl http://localhost:8000/recursos \
  -H "Authorization: Bearer <token>"
```

Caso o token esteja ausente ou inválido, a API retorna:

Status HTTP:

```text
401 Unauthorized
```

Resposta:

```json
{
    "erro": "Sem autorização para acessar esta área"
}
```

As únicas rotas públicas são:

- `GET /`
- `POST /login`

Todas as rotas em `/recursos` exigem autenticação.

---

# 📌 Endpoints disponíveis

## GET /

Retorna uma mensagem de boas-vindas e os principais endpoints da API.

---

## POST /login

Realiza a autenticação do administrador e retorna um JWT.

Request:

```json
{
    "email": "elias@teste.com",
    "senha": "123456"
}
```

Response:

```json
{
    "id": 1,
    "nome": "Elias",
    "email": "elias@teste.com",
    "token": "<jwt>"
}
```

---

## GET /recursos

Lista todos os recursos.

**Requer autenticação.**

---

## GET /recursos/<id>

Retorna um recurso pelo ID.

**Requer autenticação.**

---

## POST /recursos

Cria um novo recurso.

**Requer autenticação.**

Request:

```json
{
    "titulo": "Título do recurso",
    "descricao": "Descrição do recurso"
}
```

---

## PUT /recursos/<id>

Atualiza um recurso existente.

**Requer autenticação.**

---

## DELETE /recursos/<id>

Remove um recurso.

**Requer autenticação.**

---

# 🧪 Testes com curl

A pasta `tests_curls/` contém scripts para testar a API.

- `get_home.sh`
- `login.sh`
- `get_recurso.sh`
- `get_por_id.sh`
- `post_recurso.sh`
- `put_recurso.sh`
- `delete_recurso.sh`

O script `login.sh` demonstra todo o fluxo de autenticação:

1. Tenta acessar uma rota protegida sem token (deve retornar 401).
2. Realiza o login.
3. Extrai automaticamente o JWT da resposta.
4. Utiliza o token para acessar uma rota protegida com sucesso.

---

# 📋 Observações

- O login utiliza um administrador fixo para fins de demonstração:
  - Email: `elias@teste.com`
  - Senha: `123456`
- O JWT possui validade de 24 horas.
- O segredo utilizado para assinar o token pode ser definido pela variável de ambiente `SECRET_JWT`.
- Caso `SECRET_JWT` não seja definida, será utilizada a chave padrão `your_secret_key`.
- A autenticação é realizada por meio do middleware `JwtFairing`.

---

# 📚 Dependências principais

```toml
rocket = { version = "0.5.1", features = ["json"] }
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0"
time = "=0.3.35"
chrono = "0.4"
jsonwebtoken = "9.3.1"
```