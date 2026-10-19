use crate::handlers::events::extract_pay_load;
use crate::models::public::*;
use actix_web::{HttpResponse, Responder, get, post, web};
use askama::Template;
use crate::forms::forms::*;
use crate::services::services::*;
use crate::deserialize_data::deserialize::*;

use quick_xml::de;

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
   match cosultar_dados_cadastrais(form.into_inner()).await {
        Ok(resposta) => {
            
            let resultado: ConsultarDadosCadastraisResposta = de::from_str(extract_pay_load(&resposta, "ConsultarDadosCadastraisResposta").expect("Não encontrado dados")).unwrap();
            match resultado.cadastro {
                Some(cad) => {
                    let endereco = cad.endereco.unwrap();
                    let template = ResultadoConsultarDadosCadastraisTemplate {
                        documento: cad.cnpj.or(cad.cpf).unwrap(),
                        im: cad.im.unwrap(),
                        status_cadastro: cad.status_cadastro.unwrap(),
                        nome: cad.x_nome.unwrap(),
                        telefone: cad.telefone.unwrap(),
                        email: cad.email.unwrap(),
                        rua: endereco.rua.unwrap(),
                        numero: endereco.numero.unwrap(),
                        bairro: endereco.bairro.unwrap(),
                        municipio: endereco.cidade.unwrap(),
                        estado: endereco.estado.unwrap(),
                        cep: endereco.cep.unwrap(),
                    };

                    return HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8").body(template.render().unwrap());
                },
                None => {
                    HttpResponse::InternalServerError().body("Erro interno")
                },

                
            };
             match resultado.lista_mensagem_retorno {
                Some(erros) => {
                    for err in erros.mensagem_retorno {
                        println!("Erro do WebService [{}]: {}", err.codigo, err.mensagem);
                    }
                },
                None => {}

            }
        },
        Err(e) => panic!("Erro na resposta do xml {}", e),
   };

    // Retornar uma resposta simples
    HttpResponse::Ok().body("Dados recebidos com sucesso!")
}
//---------------------- Consultar Dados Cadastrais
