use crate::forms::forms::ConsultarDadosCadastraisForm;
use crate::handlers::events::remove_period_dash_and_slash;
use crate::requests::soap::send_request;


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

pub async fn consultar_dps_disponivel() -> Result<String, reqwest::Error> {
    let cnpj = "68356802000172";
    let im = "133468";
    let num_dps = 33;
    let serie_dps = 8;
    let pagina = 1;

    let xml = format!(r#"<ConsultarDpsDisponivelEnvio xmlns="http://www.sped.fazenda.gov.br/nfse" xmlns:ns2="http://www.w3.org/2000/09/xmldsig#">
<!--  Grupo de informações obrigatório. IDENTIFICAÇÃO DO PRESTADOR DE SERVIÇOS.  -->
<Prestador>
<!--  Informação obrigatória. Informe CPF ou CNPJ do prestador. NÃO ENVIAR ESTAS TAGS SIMULTANEAMENTE.  -->
<CNPJ>{}</CNPJ>
<!--  Informação obrigatória. Número de inscrição municipal do prestador do serviço.  -->
<IM>{}</IM>
</Prestador>
<!--  Grupo de informações opcional.  -->
<IdentificacaoDps>
<!--  Informação obrigatória.  -->
<NumDPS>{}</NumDPS>
<!--  Informação obrigatória.  -->
<SerieDPS>{}</SerieDPS>
</IdentificacaoDps>
<!--  O resultado da consulta será paginado em grupos de no máximo 50 notas.  -->
<!--  Informação obrigatória. Informe a página a ser consultada.  -->
<Pagina>{}</Pagina>
</ConsultarDpsDisponivelEnvio>"#,
    cnpj,
    im,
    num_dps,
    serie_dps,
    pagina
);

    match send_request(xml, String::from("ConsultarDpsDisponivel")).await {
        Ok(resposta) => {
            resposta.text().await            
        },
        Err(e) => Err(e),
    }
}