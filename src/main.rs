use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use dotenvy::dotenv;
use reqwest::Identity;
use std::fs::File;
use std::env;
use std::io::Read;
use openssl::{hash::{hash, MessageDigest}, pkcs12::Pkcs12};
use base64::{Engine as _, engine::general_purpose};

use nfse_client::handlers::handlers::*;

#[warn(dead_code)]
fn calcular_digest_value(xml: &str) -> Result<String, Box<dyn std::error::Error>> {
	let xml_bytes = xml.as_bytes();
	let digest_bytes = hash(MessageDigest::sha1(), xml_bytes)?;
	let digest_base64 = general_purpose::STANDARD.encode(digest_bytes);
	Ok(digest_base64)
}


#[get("/")]
async fn home() -> impl Responder {
    let certificate_file = env::var("CERTIFICATE_FILE").expect("CERTIFICATE_FILE não encontrado");
    let mut file = File::open(certificate_file).expect("Erro ao abrir o arquivo de certificado");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Erro ao ler o arquivo de certificado");

    let pkcs12 = Pkcs12::from_der(&buf).expect("Erro ao criar o objeto Pkcs12");
    let parsed_cert = pkcs12.parse2(
        &env::var("PASSWORD_CERTIFICATE")
        .expect("PASSWORD_CERTIFICATE não presente"))
        .expect("Erro ao analisar o certificado");

    let _x509_cert = parsed_cert.cert.expect("Certificado público não encontrado no PFX");

    // 1. XML de dados limpo (sem assinatura, pois a consulta é autenticada via mTLS)
    let xml_dados_limpo = r#"<ConsultarNfseServicoPrestadoEnvio xmlns="http://www.sped.fazenda.gov.br/nfse"><Prestador><CNPJ>68356802000172</CNPJ><IM>133468</IM></Prestador><NumeroNfse>1</NumeroNfse><Tomador><CNPJ>63554067000198</CNPJ></Tomador><Pagina>1</Pagina></ConsultarNfseServicoPrestadoEnvio>"#;

    // 2. Configurar o Cliente mTLS (o certificado na camada de transporte já autentica a chamada)
    let identity = Identity::from_pkcs12_der(&buf, &env::var("PASSWORD_CERTIFICATE").expect("PASSWORD_CERTIFICATE não presente"))
        .expect("Erro identidade");
    let client = reqwest::Client::builder()
        .identity(identity)
        .build().expect("Erro client");

    let cabecalho_str = r#"<cabecalho versao="1.00" xmlns="http://www.sped.fazenda.gov.br/nfse"><versaoDados>1.00</versaoDados></cabecalho>"#;

    // 3. Montar o Envelope SOAP enviando o XML limpo (sem a tag Signature)
    let soap_body = format!(
        r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:nfse="http://www.sped.fazenda.gov.br/nfse"><soapenv:Header/><soapenv:Body><nfse:ConsultarNfseServicoPrestado><nfse:nfseCabecMsg>{}</nfse:nfseCabecMsg><nfse:nfseDadosMsg>{}</nfse:nfseDadosMsg></nfse:ConsultarNfseServicoPrestado></soapenv:Body></soapenv:Envelope>"#,
        cabecalho_str,
        xml_dados_limpo
    );

    let url = env::var("URL").expect("URL não encontrado");

    // 4. Enviar requisição
    let response = client
        .post(&url)
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "http://www.sped.fazenda.gov.br/nfse/ConsultarNfseServicoPrestado")
        .body(soap_body)
        .send()
        .await.unwrap();

    let status = response.status();
    let response_text = response.text().await.unwrap();

    println!("Status: {}", status);
    println!("Response: {}", response_text);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response_text)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
        .service(home)
        .service(consultar_dados_cadastrais)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

   
}
