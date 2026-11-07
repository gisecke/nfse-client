use crate::forms::{Dps, Prestador, Servico, Tomador, Tributacao};
use chrono::Local;
use fiscal_crypto::certificate::{load_certificate, sign_dps_xml};
use std::{env, fs};

pub fn remove_period_dash_and_slash(cadastro: &str) -> String {
    let mut result = cadastro.to_string();
    result.retain(|c| c != '.' && c != '-' && c != '/');
    result
}

pub fn extract_pay_load<'a>(resposta_soap: &'a str, nome_tag: &str) -> Option<&'a str> {
    let tag_inicio = format!("<{}>", nome_tag);
    let tag_final = format!("</{}>", nome_tag);

    let start = resposta_soap.find(&tag_inicio)?;
    let end = resposta_soap.find(&tag_final)? + tag_final.len();

    Some(&resposta_soap[start..end])
}

pub fn get_date_and_local_time() -> String {
    // Atrasa 3 minutos do relógio local, para evitar erros com o horário do servidor da prefeitura
    let now = Local::now() - chrono::Duration::minutes(3);
    format!(
        "{}T{}-03:00",
        now.format("%Y-%m-%d"),
        now.format("%H:%M:%S")
    )
}

pub fn format_in_xml_cpf_or_cnpj(cadastro: &str) -> String {
    if cadastro.len() < 14 {
        format!("<CPF>{}</CPF>", cadastro)
    } else {
        format!("<CNPJ>{}</CNPJ>", cadastro)
    }
}

//Formata o CPF ou CNPJ com os pontos e traços
//CPF: xxx.xxx.xxx-xx
//CNPJ: xx.xxx.xxx/xxxx-xx
pub fn format_cadastro(cadastro: &str) -> String {
    let cadastro_string = cadastro.trim().to_string();
    if cadastro_string.len() == 14 {
        format!(
            "{}.{}.{}/{}-{}",
            &cadastro_string[0..2],
            &cadastro_string[2..5],
            &cadastro_string[5..8],
            &cadastro_string[8..12],
            &cadastro_string[12..14]
        )
    } else if cadastro.len() == 11 {
        format!(
            "{}.{}.{}-{}",
            &cadastro_string[0..3],
            &cadastro_string[2..5],
            &cadastro_string[5..8],
            &cadastro_string[8..11]
        )
    } else {
        cadastro.to_string()
    }
}

//Formata o número de telefone (xx) xxxxx-xxxx
pub fn format_phone_number(number: &str) -> String {
    let clean_mumber: String = number
        .trim()
        .to_string()
        .chars()
        .filter(|c| !c.is_alphabetic())
        .collect();
    format!(
        "({}) {}-{}",
        &clean_mumber[0..2],
        &clean_mumber[2..7],
        &clean_mumber[7..11]
    )
}

pub fn xml_versao_101(
    dps: Dps,
    prestador: Prestador,
    tomador: Tomador,
    tributacao: Tributacao,
    servico: Servico,
) -> String {
    let xml = format!(
        r#"<infDPS Id="{}"><tpAmb>{}</tpAmb><dhEmi>{}</dhEmi><verAplic>0.1.0</verAplic><serie>{}</serie><nDPS>{}</nDPS><dCompet>{}</dCompet><tpEmit>1</tpEmit><cLocEmi>{}</cLocEmi><prest><CNPJ>{}</CNPJ><IM>{}</IM><regTrib><opSimpNac>1</opSimpNac><regEspTrib>0</regEspTrib></regTrib></prest><toma>{}<xNome>{}</xNome><end><endNac><cMun>{}</cMun><CEP>{}</CEP></endNac><xLgr>{}</xLgr><nro>{}</nro><xBairro>{}</xBairro></end></toma><serv><locPrest><cLocPrestacao>{}</cLocPrestacao></locPrest><cServ><cTribNac>{}</cTribNac><cTribMun>{}</cTribMun><xDescServ>{}</xDescServ><cNBS>{}</cNBS></cServ></serv><valores><vServPrest><vServ>{}</vServ></vServPrest><trib><tribMun><tribISSQN>1</tribISSQN><tpRetISSQN>{}</tpRetISSQN><pAliq>2.00</pAliq></tribMun><tribFed><piscofins><CST>00</CST><tpRetPisCofins>0</tpRetPisCofins></piscofins></tribFed><totTrib><pTotTrib><pTotTribFed>13.45</pTotTribFed><pTotTribEst>0.00</pTotTribEst><pTotTribMun>2.00</pTotTribMun></pTotTrib></totTrib></trib></valores><IBSCBS><finNFSe>0</finNFSe><indFinal>0</indFinal><cIndOp>030102</cIndOp><indDest>0</indDest><valores><trib><gIBSCBS><CST>200</CST><cClassTrib>200029</cClassTrib></gIBSCBS></trib></valores></IBSCBS></infDPS>"#,
        dps.id,
        dps.homologacao_ou_producao,
        dps.data_hora_local,
        dps.serie,
        dps.numero,
        servico.data,
        dps.localidade,
        prestador.cnpj,
        prestador.im,
        tomador.cadastro,
        tomador.razao_social,
        tomador.codigo_municipal,
        tomador.cep,
        tomador.logradouro,
        tomador.nro_end,
        tomador.bairro,
        tributacao.localidade,
        tributacao.cod_trib_nacional,
        tributacao.cod_trib_municipal,
        servico.descricao,
        tributacao.cod_nbs,
        servico.valor,
        tributacao.tp_ret_issqn,
    );

    xml
}

pub fn assinatura_gerar_nfse(xml: &str) -> String {
    // =========================================================================
    // 1. CARREGAR CERTIFICADO DIGITAL (.PFX / .P12) E CHAVE PRIVADA
    // =========================================================================
    let pfx_bytes =
        fs::read(env::var("CERTIFICATE_FILE").expect("CERTIFICATE_FILE não encontrado no .env"))
            .expect("Erro ao ler PFX");
    let cert_password =
        env::var("PASSWORD_CERTIFICATE").expect("PASSWORD_CERTIFICATE não encontrado no .env");

    let cert_data = load_certificate(&pfx_bytes, &cert_password).expect("Erro ao ler PFX");

    let xml_dps = format!(
        r#"<DPS xmlns="http://www.sped.fazenda.gov.br/nfse" versao="1.01">{}</DPS>"#,
        xml
    );

    sign_dps_xml(&xml_dps, &cert_data.private_key, &cert_data.certificate)
        .expect("Erro ao assinar DPS")
}
