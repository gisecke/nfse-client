use crate::forms::forms::ConsultarDadosCadastraisForm;
use crate::handlers::events::remove_period_dash_and_slash;
use crate::soap::requests::send_request;


pub async fn cosultar_dados_cadastrais(dados: ConsultarDadosCadastraisForm) -> Result<String, reqwest::Error> {
    let cnpj = dados.cnpj;
    let cpf = dados.cpf;
    let im = dados.im;


    let documento_xml = if cpf.is_empty() {
        format!("<CNPJ>{}</CNPJ>", remove_period_dash_and_slash(&cnpj))
    } else {
        format!("<CPF>{}</CPF>", remove_period_dash_and_slash(&cpf))
    };
    
    let xml_bruto = format!(r#"<ConsultarDadosCadastraisEnvio xmlns="http://www.sped.fazenda.gov.br/nfse" xmlns:ns2="http://www.w3.org/2000/09/xmldsig#">
    <!--  Grupo de informações obrigatório. IDENTIFICAÇÃO DO PRESTADOR DE SERVIÇOS.  -->
    <Prestador>{}
    <IM>{}</IM>
    </Prestador>
    </ConsultarDadosCadastraisEnvio>"#,
                documento_xml, im);


    match send_request(xml_bruto, "ConsultarDadosCadastrais".to_string()).await {
        Ok(response) => {
            response.text().await
        },
        Err(e) => Err(e)
    } 
}