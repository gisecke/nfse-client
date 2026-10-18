use reqwest::{Error, Identity, Response};
use std::fs::File;
use std::io::Read;
use std::env;


pub async fn send_request(xml: String, service: String) -> Result<Response, Error> {

    let certificate_file = env::var("CERTIFICATE_FILE").expect("CERTIFICATE_FILE não encontrado");
    let mut file = File::open(certificate_file).expect("Erro ao abrir o arquivo de certificado");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Erro ao ler o arquivo de certificado");


     let identity = Identity::from_pkcs12_der(&buf, &env::var("PASSWORD_CERTIFICATE").expect("PASSWORD_CERTIFICATE não presente"))
        .expect("Erro identidade");
    let client = reqwest::Client::builder()
        .identity(identity)
        .build().expect("Erro client");

    let cabecalho_str = r#"<cabecalho versao="1.01" xmlns="http://www.sped.fazenda.gov.br/nfse"><versaoDados>1.01</versaoDados></cabecalho>"#;


    let soap_body = format!(
        r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:nfse="http://www.sped.fazenda.gov.br/nfse">
            <soapenv:Header/>
            <soapenv:Body>
                <nfse:{service}>
                    <nfse:nfseCabecMsg>{}</nfse:nfseCabecMsg>
                    <nfse:nfseDadosMsg>{}</nfse:nfseDadosMsg>
                </nfse:{service}>
            </soapenv:Body>
        </soapenv:Envelope>"#,
        cabecalho_str,
        xml
    );

    let url = env::var("URL").expect("URL não encontrado");

    // 4. Enviar requisição
    let response = client
        .post(&url)
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", format!("http://www.sped.fazenda.gov.br/nfse/{}", service))
        .body(soap_body)
        .send()
        .await?;
    println!("{}", response.status());
    Ok(response)
}