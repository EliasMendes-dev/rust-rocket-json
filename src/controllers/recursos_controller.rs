use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use crate::models::recurso::Recurso;
use crate::models::erro_json::ErroJson;
use crate::servicos::recurso_servico;
use crate::dtos::recurso_dto::RecursoDto;

#[get("/recursos")]
pub fn index() -> Json<Vec<Recurso>> {
    let recursos = recurso_servico::lista_de_recursos();
    Json(recursos)
}

// ROTA: GET por ID
#[get("/recursos/<id>")]
pub fn mostrar(id: u32) -> Result<Json<Recurso>, status::Custom<Json<ErroJson>>> {
    match recurso_servico::busca_por_id(id) {
        Some(recurso) => Ok(Json(recurso)),
        None => {
            Err(status::Custom(
                Status::NotFound,
                Json(ErroJson {
                    erro: format!("Recurso com ID {} não encontrado", id)
                })
            ))
        }
    }
}

#[post("/recursos", data = "<recurso_dto>")]
pub fn criar(recurso_dto: Json<RecursoDto>) -> Result<Json<Recurso>, status::Custom<Json<ErroJson>>> {
    let recurso_dto = recurso_dto.into_inner();

    match recurso_servico::cadastrar_recurso(recurso_dto) {
        Ok(recurso) => Ok(Json(recurso)),
        Err(erro) => {
            let erro_json = ErroJson {
                erro: erro.to_string(),
            };
            Err(status::Custom(Status::BadRequest, Json(erro_json)))
        }
    }
}

#[put("/recursos/<id>", data = "<recurso_dto_json>")]
pub fn alterar(id: u32, recurso_dto_json: Json<RecursoDto>) -> Result<Json<Recurso>, status::Custom<Json<ErroJson>>> {
    let recurso_dto = recurso_dto_json.into_inner();

    match recurso_servico::alterar_recurso(id, recurso_dto) {
        Ok(recurso) => Ok(Json(recurso)),
        Err(erro) => {
            Err(status::Custom(
                Status::BadRequest,
                Json(ErroJson { erro })  // 👈 Campo "erro"
            ))
        }
    }
}

#[delete("/recursos/<id>")]
pub fn excluir(id: u32) -> Result<status::Custom<Json<()>>, status::Custom<Json<ErroJson>>> {
    match recurso_servico::apagar_recurso_por_id(id) {
        Ok(()) => Ok(status::Custom(Status::NoContent, Json(()))),
        Err(erro) => {
            Err(status::Custom(
                Status::BadRequest,
                Json(ErroJson { erro: erro.to_string() })
            ))
        }
    }
}