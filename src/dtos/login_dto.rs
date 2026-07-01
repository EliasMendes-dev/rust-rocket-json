use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginDto {
    pub email: String,
    pub senha: String,
}