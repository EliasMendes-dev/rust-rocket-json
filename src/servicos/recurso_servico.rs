use crate::dtos::recurso_dto::RecursoDto;
use crate::models::recurso::Recurso;
use crate::models::app_error::AppError;

pub fn lista_de_recursos() -> Vec<Recurso> {
    vec![
        Recurso {
            id: 1,
            titulo: "Recurso 1".to_string(),
            descricao: "Descrição do recurso 1".to_string(),
        },
        Recurso {
            id: 2,
            titulo: "Recurso 2".to_string(),
            descricao: "Descrição do recurso 2".to_string(),
        },
    ]
}

pub fn busca_por_id(id: u32) -> Option<Recurso> {
    println!("Buscando recurso com ID: {}", id);
    
    let recursos = lista_de_recursos();
    recursos.into_iter().find(|r| r.id == id)
}

pub fn apagar_recurso_por_id(id: u32) -> Result<(), AppError> {
    // Usar repositório para apagar no DB

    println!("Id: {}", id);


    // Validação
    if true {
        Ok(())
    } else {
        Err(AppError::ValidationError("Id é obrigatório".to_string()))
    }
}

pub fn cadastrar_recurso(recurso_dto: RecursoDto) -> Result<Recurso, AppError> {
    // Converter DTO para Recurso
    let novo_recurso = Recurso {
        id: 3, // Simulando ID gerado
        titulo: recurso_dto.titulo,
        descricao: recurso_dto.descricao,
    };

    println!("Id: {}", novo_recurso.id);
    println!("Titulo: {}", novo_recurso.titulo);
    println!("Descricao: {}", novo_recurso.descricao);

    // Validação
    if !novo_recurso.titulo.is_empty() && !novo_recurso.descricao.is_empty() {
        Ok(novo_recurso)
    } else {
        Err(AppError::ValidationError("Título e descrição são obrigatórios".to_string()))
    }
}

pub fn alterar_recurso(id: u32, recurso_dto: RecursoDto) -> Result<Recurso, String> {
    println!("Id: {}", id);
    println!("Titulo: {}", recurso_dto.titulo);
    println!("Descricao: {}", recurso_dto.descricao);

    if recurso_dto.titulo.is_empty() {
        return Err("O título não pode ser vazio".to_string());
    }

    // Simula atualização
    Ok(Recurso {
        id,
        titulo: recurso_dto.titulo,
        descricao: recurso_dto.descricao,
    })
}