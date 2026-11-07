use std::env;

use crate::forms::{
    ConsultarDadosCadastraisForm, DadosGerarNfse, Dps, Prestador, Servico, Tomador, Tributacao,
};
use crate::handlers::events::{
    assinatura_gerar_nfse, format_in_xml_cpf_or_cnpj, get_date_and_local_time,
    remove_period_dash_and_slash, xml_versao_101,
};
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
    let tipo_inscricao = if dados.documento.len() > 11 { "2" } else { "1" };

    let prestador = Prestador {
        cnpj: env::var("CNPJ_PREST").expect("CNPJ não encontrado"),
        im: env::var("IM").expect("Inscrição Municipal do prestador não encontrado"),
    };

    let id_dps = format!(
        "DPS{:0>7}{}{:0>14}{:0>5}{:0>15}",
        dados.municipio_serv_prestado,
        tipo_inscricao,
        prestador.cnpj,
        dados.serie,
        dados.numero_dps,
    );

    let dps = Dps {
        id: id_dps,
        homologacao_ou_producao: env::var("HOMOLOGACAO_OU_PRODUCAO")
            .expect("Valor de chave não encontrado"),
        data_hora_local: get_date_and_local_time(),
        serie: dados.serie,
        numero: dados.numero_dps,
        localidade: dados.municipio_serv_prestado.clone(),
    };

    let tomador = Tomador {
        cadastro: format_in_xml_cpf_or_cnpj(&remove_period_dash_and_slash(&dados.documento)),
        razao_social: dados.razao_social,
        codigo_municipal: dados.codigo_ibge_mun_tomador,
        cep: remove_period_dash_and_slash(&dados.cep),
        logradouro: dados.logradouro,
        nro_end: dados.nro,
        bairro: dados.xbairro,
    };

    let tributacao = Tributacao {
        localidade: dados.municipio_serv_prestado.clone(),
        cod_trib_nacional: remove_period_dash_and_slash(&dados.ctribnac),
        cod_trib_municipal: remove_period_dash_and_slash(&dados.ctribmun),
        cod_nbs: remove_period_dash_and_slash(&dados.cnbs),
        tp_ret_issqn: dados.tp_ret_issqn,
    };

    let servico = Servico {
        data: dados.data_local,
        descricao: dados.desc_serv,
        valor: dados.vserv,
    };

    let xml = xml_versao_101(dps, prestador, tomador, tributacao, servico);

    let dps_assinado = assinatura_gerar_nfse(&xml);

    let xml_completo = format!(
        r#"<GerarNfseEnvio xmlns="http://www.sped.fazenda.gov.br/nfse">{}</GerarNfseEnvio>"#,
        dps_assinado
    );
    println!("{}", xml_completo);

    match send_request(xml_completo, "GerarNfse".to_string()).await {
        Ok(response) => response.text().await,
        Err(e) => Err(e),
    }
}

