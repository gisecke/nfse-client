use std::env;

use crate::forms::forms::{ConsultarDadosCadastraisForm, DadosGerarNfse};
use crate::handlers::events::{assinatura_gerar_nfse, format_in_xml_cpf_or_cnpj, get_date_and_local_time, remove_period_dash_and_slash, xml_versao_101};
use crate::requests::soap::send_request;

pub async fn cosultar_dados_cadastrais(
    dados: ConsultarDadosCadastraisForm,
) -> Result<String, reqwest::Error> {
    let cadastro = dados.cadastro;
    let cadastro_limpo = remove_period_dash_and_slash(&cadastro);
    let im = dados.im;

    let documento_xml = if cadastro.len() > 11 {
        format!("<CNPJ>{}</CNPJ>", cadastro_limpo)
    } else {
        format!("<CPF>{}</CPF>", cadastro_limpo)
    };

    let xml_bruto = format!(
        r#"<ConsultarDadosCadastraisEnvio xmlns="http://www.sped.fazenda.gov.br/nfse" xmlns:ns2="http://www.w3.org/2000/09/xmldsig#">
    <!--  Grupo de informações obrigatório. IDENTIFICAÇÃO DO PRESTADOR DE SERVIÇOS.  -->
    <Prestador>{}
    <IM>{}</IM>
    </Prestador>
    </ConsultarDadosCadastraisEnvio>"#,
        documento_xml, im
    );

    println!("xml: {}", xml_bruto);

    match send_request(xml_bruto, "ConsultarDadosCadastrais".to_string()).await {
        Ok(response) => response.text().await,
        Err(e) => Err(e),
    }
}

pub async fn consultar_dps_disponivel() -> Result<String, reqwest::Error> {
    let cnpj = "68356802000172";
    let im = "133468";
    let num_dps = 33;
    let serie_dps = 8;
    let pagina = 1;

    let xml = format!(
        r#"<ConsultarDpsDisponivelEnvio xmlns="http://www.sped.fazenda.gov.br/nfse" xmlns:ns2="http://www.w3.org/2000/09/xmldsig#">
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
        cnpj, im, num_dps, serie_dps, pagina
    );

    match send_request(xml, String::from("ConsultarDpsDisponivel")).await {
        Ok(resposta) => resposta.text().await,
        Err(e) => Err(e),
    }
}

pub async fn gerar_nfse(dados: DadosGerarNfse) -> Result<String, reqwest::Error> {

    let cnpj_prest = env::var("CNPJ_PREST").expect("CNPJ não encontrado");
    let im_prest = env::var("IM").expect("Inscrição Municipal do prestador não encontrado");

    let tipo_inscricao = if dados.documento.len() > 11 {
        "2" } else {
            "1"
        };

    let documento_limpo = remove_period_dash_and_slash(&dados.documento);
    let documento_formatado_xml = format_in_xml_cpf_or_cnpj(&documento_limpo);
    let localidade_prest_servico = dados.municipio_serv_prestado;
    
    let id_dps = format!(
        "DPS{:0>7}{}{:0>14}{:0>5}{:0>15}",
        localidade_prest_servico,
        tipo_inscricao,
        cnpj_prest,
        dados.serie,
        dados.numero_dps,
    );

    let serie_dps = dados.serie;
    let numero_dps = dados.numero_dps;
    let data_servico = dados.data_local;
    let razao_social_tomador = dados.razao_social;
    let cep_tomador = remove_period_dash_and_slash(&dados.cep);
    let codigo_municipio_tomador = dados.codigo_ibge_mun_tomador;
    let logradouro = dados.logradouro;
    let numero_end = dados.nro;
    let bairro = dados.xbairro;
    let cod_trib_nacional = remove_period_dash_and_slash(&dados.ctribnac);
    let cod_trib_municipal = dados.ctribmun;
    let cod_nbs = remove_period_dash_and_slash(&dados.cnbs);
    let descricao = dados.desc_serv;
    let valor_ser = dados.vserv;
    let tipo_retencao = dados.tp_ret_issqn;

    let homologacao_ou_producao = env::var("HOMOLOGACAO_OU_PRODUCAO").expect("Valor de chave não encontrado");


    let data_hora_local = get_date_and_local_time();



    let xml = xml_versao_101(id_dps,
                             homologacao_ou_producao,
                             data_hora_local,
                             serie_dps,
                             numero_dps,
                             data_servico,
                             localidade_prest_servico.clone(),
                             cnpj_prest,
                             im_prest,
                             documento_formatado_xml,
                             razao_social_tomador,
                             codigo_municipio_tomador,
                             cep_tomador,
                             logradouro,
                             numero_end,
                             bairro,
                             localidade_prest_servico,
                             cod_trib_nacional,
                             cod_trib_municipal,
                             descricao,
                             cod_nbs,
                             valor_ser,
                             tipo_retencao,
    );


    let dps_assinado = assinatura_gerar_nfse(&xml);

    let xml_completo = format!(r#"<GerarNfseEnvio xmlns="http://www.sped.fazenda.gov.br/nfse">{}</GerarNfseEnvio>"#, dps_assinado);
    println!("{}", xml_completo);


    match send_request(xml_completo, "GerarNfse".to_string()).await {
        Ok(response) => {
            response.text().await
        },
        Err(e) => Err(e),
    }

    
}
