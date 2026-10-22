use crate::handlers::events::{extract_pay_load, remove_period_dash_and_slash};
use crate::models::public::*;
use actix_web::{HttpResponse, Responder, get, post, web};
use askama::Template;
use chrono::Local;
use crate::forms::forms::*;
use crate::services::services::*;
use crate::deserialize_data::deserialize::*;

use quick_xml::de;
use serde_json::Value;
use crate::requests::json::get_cep_json;

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

            println!("{}", resposta);
            
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


//Consulta DPS disponivel
#[get("/dps-disponivel")]
async fn numero_dps_disponivel() -> impl Responder {
    match consultar_dps_disponivel().await {
        Ok(resposta) => {
            HttpResponse::Ok().body(resposta)
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Erro:")
        }
    }

}

#[get("/gerar-nfse")]
async fn gerar_nfse_get() -> impl Responder {


    let template = GerarNFSETemplate {
        data_local: Local::now().date_naive(),
        rua: String::new(),
        bairro: String::new(),
        cidade: String::new(),
        uf: String::new(),
        estado: String::new(),
    };
    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
        .body(template.render().unwrap())
}

#[post("/api/buscar-cep")]
async fn buscar_cep(form: web::Form<CepForm>) -> impl Responder {
    let resultado = remove_period_dash_and_slash(form.cep.as_str());

    match get_cep_json(resultado).await {
        Ok(json) => {
            let value: Value = serde_json::from_str(json.text().await.unwrap().as_str()).unwrap();
            let template = CepTemplate {
                bairro: match value["bairro"].as_str() {
                    Some(bairro) => bairro.to_string(),
                    None => "".to_string(),
                },
                cidade: match value["localidade"].as_str() {
                    Some(localidade) => localidade.to_string(),
                    None => "".to_string(),
                },
                rua: match value["logradouro"].as_str() {
                    Some(logradouro) => logradouro.to_string(),
                    None => "".to_string(),
                },
                uf: match value["uf"].as_str() {
                    Some(uf) => uf.to_string(),
                    None => "".to_string(),
                },
                estado: match value["estado"].as_str() {
                    Some(estado) => estado.to_string(),
                    None => "".to_string(),
                }
            };
            HttpResponse::Ok().body(template.render().unwrap())
        },
        Err(_e) => HttpResponse::InternalServerError().body("Erro json"),
    }
}