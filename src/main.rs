use actix_files as fs;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use openssl::{hash::{hash, MessageDigest}};
use base64::{Engine as _, engine::general_purpose};

use nfse_client::handlers::handlers::*;

#[warn(dead_code)]
fn calcular_digest_value(xml: &str) -> Result<String, Box<dyn std::error::Error>> {
	let xml_bytes = xml.as_bytes();
	let digest_bytes = hash(MessageDigest::sha1(), xml_bytes)?;
	let digest_base64 = general_purpose::STANDARD.encode(digest_bytes);
	Ok(digest_base64)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(home)
            .service(consultar_dados_cadastrais_get)
            .service(consultar_dados_cadastrais_post)
            .service(numero_dps_disponivel)
            .service(gerar_nfse_get)
            .service(buscar_cep)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

   
}
