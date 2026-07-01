#[macro_use] extern crate rocket;

mod controllers;
mod dtos;
mod models;
mod model_views;
mod servicos;

use controllers::{home_controller, login_controller, recursos_controller};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        home_controller::index,
        recursos_controller::index,
        recursos_controller::mostrar,
        recursos_controller::criar,
        recursos_controller::alterar,
        recursos_controller::excluir,
        login_controller::login,  // ← Adicione esta linha
    ])
}