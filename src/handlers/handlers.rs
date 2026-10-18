use crate::models::public::*;
use actix_web::{HttpResponse, Responder, get, post, web};
use askama::Template;
use crate::forms::forms::*;
use crate::services::services::*;

// Home page
#[get("/")]
async fn home() -> impl Responder {
    let template = HomeTemplate {};

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template.render().unwrap())
}

//Consultar Dados Cadastrais
#[get("/consultar-dados-cadastrais")]
async fn consultar_dados_cadastrais_get() -> impl Responder {
    
    let template = ConsultarDadosCadastraisTemplate {};

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template.render().unwrap())
}

#[post("/consultar-dados-cadastrais")]
async fn consultar_dados_cadastrais_post(form: web::Form<ConsultarDadosCadastraisForm>) -> impl Responder {
    
    // Aqui você pode processar os dados recebidos do formulário
   cosultar_dados_cadastrais(form.into_inner()).await;

    // Retornar uma resposta simples
    HttpResponse::Ok().body("Dados recebidos com sucesso!")
}