use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RecursoDto {
    pub titulo: String,
    pub descricao: String,
}