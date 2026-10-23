use std::fs::File;
use std::io::BufReader;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use openssl::{hash::{hash, MessageDigest}};
use base64::{Engine as _, engine::general_purpose};
use nfse_client::deserialize_data::deserialize::{Municipio, MunicipioIbgeRaw};
use nfse_client::handlers::handlers::*;

#[warn(dead_code)]
fn calcular_digest_value(xml: &str) -> Result<String, Box<dyn std::error::Error>> {
	let xml_bytes = xml.as_bytes();
	let digest_bytes = hash(MessageDigest::sha1(), xml_bytes)?;
	let digest_base64 = general_purpose::STANDARD.encode(digest_bytes);
	Ok(digest_base64)
}

pub fn carregar_municipios() -> Vec<Municipio> {
    let file = File::open("files/municipios.json").expect("file municipios.json not found");
    let reader = BufReader::new(file);
    let raw_list: Vec<MunicipioIbgeRaw> = serde_json::from_reader(reader).expect("Falha ao deserializar JSON municipios");


    raw_list
        .into_iter()
        .map(|m| Municipio {
            ibge: m.id.to_string(),
            nome: m.nome,
            uf: m.uf
        })
        .collect()
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let municipios = carregar_municipios();
    let dados_municipios = web::Data::new(municipios);


    dotenv().ok();
    HttpServer::new(move || {
        App::new()
            .app_data(dados_municipios.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(home)
            .service(consultar_dados_cadastrais_get)
            .service(consultar_dados_cadastrais_post)
            .service(numero_dps_disponivel)
            .service(gerar_nfse_get)
            .service(buscar_cep)
            .service(buscar_cnpj)
            .service(buscar_municipios)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

   
}
