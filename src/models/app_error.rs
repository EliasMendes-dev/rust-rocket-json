#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    DatabaseError(String),
    ValidationError(String),
    NotFound(String),      // ← Adicione para recurso não encontrado
    AuthError(String),     // ← Adicione para erros de autenticação
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Erro de banco: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Erro de validação: {}", msg),
            AppError::NotFound(msg) => write!(f, "Não encontrado: {}", msg),
            AppError::AuthError(msg) => write!(f, "Erro de autenticação: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}