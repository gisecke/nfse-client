use crate::models::public::*;
use actix_web::{HttpResponse, Responder, get};
use askama::Template;

#[get("/consultar-dados-cadastrais")]
async fn consultar_dados_cadastrais() -> impl Responder {
    
    let template = ConsultarDadosCadastraisTemplate {};

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template.render().unwrap())
}