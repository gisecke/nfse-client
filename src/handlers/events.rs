use std::{env, fs};
use chrono::Local;
use fiscal_crypto::certificate::{load_certificate, sign_dps_xml};

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
   format!("{}T{}-03:00", now.format("%Y-%m-%d"), now.format("%H:%M:%S"))
}

pub fn format_in_xml_cpf_or_cnpj(cadastro: &str) -> String {
   if cadastro.len() < 14 {
      format!("<CPF>{}</CPF>", cadastro)
   } else {
      format!("<CNPJ>{}</CNPJ>", cadastro)
   }
}

pub fn xml_versao_101(
   id_dps: String,
   homologacao_ou_producao: String,
   data_hora_local: String,
   serie_dps: String,
   numero_dps: String,
   data_servico: String,
   localidade_emissora: String,
   cnpj_prest: String,
   im_prest: String,
   documento_formatado_xml: String,
   razao_social_tomador: String,
   codigo_municipio_tomador: String,
   cep_tomador: String,
   logradouro: String,
   numero_end: String,
   bairro: String,
   localidade_prest_servico: String,
   cod_trib_nacional: String,
   cod_trib_municipal: String,
   descricao: String,
   cod_nbs: String,
   valor_ser: String,
   tp_ret_issqn: String,
) -> String {
   let xml = format!(r#"<infDPS Id="{id_dps}"><tpAmb>{homologacao_ou_producao}</tpAmb><dhEmi>{data_hora_local}</dhEmi><verAplic>0.1.0</verAplic><serie>{serie_dps}</serie><nDPS>{numero_dps}</nDPS><dCompet>{data_servico}</dCompet><tpEmit>1</tpEmit><cLocEmi>{localidade_emissora}</cLocEmi><prest><CNPJ>{cnpj_prest}</CNPJ><IM>{im_prest}</IM><regTrib><opSimpNac>1</opSimpNac><regEspTrib>0</regEspTrib></regTrib></prest><toma>{documento_formatado_xml}<xNome>{razao_social_tomador}</xNome><end><endNac><cMun>{codigo_municipio_tomador}</cMun><CEP>{cep_tomador}</CEP></endNac><xLgr>{logradouro}</xLgr><nro>{numero_end}</nro><xBairro>{bairro}</xBairro></end></toma><serv><locPrest><cLocPrestacao>{localidade_prest_servico}</cLocPrestacao></locPrest><cServ><cTribNac>{cod_trib_nacional}</cTribNac><cTribMun>{cod_trib_municipal}</cTribMun><xDescServ>{descricao}</xDescServ><cNBS>{cod_nbs}</cNBS></cServ></serv><valores><vServPrest><vServ>{valor_ser}</vServ></vServPrest><trib><tribMun><tribISSQN>1</tribISSQN><tpRetISSQN>{tp_ret_issqn}</tpRetISSQN></tribMun><tribFed><piscofins><CST>00</CST><tpRetPisCofins>0</tpRetPisCofins></piscofins></tribFed><totTrib><pTotTrib><pTotTribFed>13.45</pTotTribFed><pTotTribEst>0.00</pTotTribEst><pTotTribMun>2.00</pTotTribMun></pTotTrib></totTrib></trib></valores><IBSCBS><finNFSe>0</finNFSe><indFinal>0</indFinal><cIndOp>030102</cIndOp><indDest>0</indDest><valores><trib><gIBSCBS><CST>200</CST><cClassTrib>200029</cClassTrib></gIBSCBS></trib></valores></IBSCBS></infDPS>"#);

   xml
}

pub fn assinatura_gerar_nfse(xml: &str) -> String {

   // =========================================================================
   // 1. CARREGAR CERTIFICADO DIGITAL (.PFX / .P12) E CHAVE PRIVADA
   // =========================================================================
   let pfx_bytes = fs::read(env::var("CERTIFICATE_FILE").expect("CERTIFICATE_FILE não encontrado no .env")).expect("Erro ao ler PFX");
   let cert_password = env::var("PASSWORD_CERTIFICATE").expect("PASSWORD_CERTIFICATE não encontrado no .env");

   let cert_data = load_certificate(&pfx_bytes, &cert_password).expect("Erro ao ler PFX");

   let xml_dps = format!(r#"<DPS xmlns="http://www.sped.fazenda.gov.br/nfse" versao="1.01">{}</DPS>"#, xml);

   let xml_assinado = sign_dps_xml(
                                    &xml_dps,
                                    &cert_data.private_key,
                                    &cert_data.certificate,
).expect("Erro ao assinar DPS");

   xml_assinado


   /*
   let mut cert_file = File::open(&cert_file_path).expect("Erro ao abrir o arquivo do certificado");
   let mut cert_der_bytes = Vec::new();
   cert_file.read_to_end(&mut cert_der_bytes).expect("Erro ao ler os dados do Certificado");

   let pkcs12 = Pkcs12::from_der(&cert_der_bytes).expect("Erro ao ao deserializar PKCS#12");
   let parsed_cert = pkcs12.parse2(&cert_password).expect("Erro ao decodificar com a senha fornecida");

   let private_key = parsed_cert.pkey.expect("Chave privada não encontrada no certificado");
   let x509_cert = parsed_cert.cert.expect("Chave X.509 Cert não encontrada");

//    Certificado público X509 em Base64
   let x509_certificate_base64 = general_purpose::STANDARD.encode(
      x509_cert.to_der().expect("Erro ao converter certificado para DER")
   );

   // =========================================================================
   // 2. CÁLCULO DO DIGESTVALUE (HASH SHA-1 DA TAG <infDPS>)
   // =========================================================================
   // O hash SHA-1 é calculado sobre o conteúdo da tag <infDPS Id="...">...</infDPS>
   let digest_bytes = hash(MessageDigest::sha1(), xml.trim().as_bytes()).expect("Falha ao calcular SHA-1 do infDPS");
   let digest_value = general_purpose::STANDARD.encode(digest_bytes);

   // =========================================================================
   // 3. CONSTRUÇÃO DO <SignedInfo> CANONICALIZADO (C14N)
   // =========================================================================
   // O SignedInfo deve estar exatamente no padrão canônico (sem espaços extras nos atributos)
   let signed_info_xml = format!(
      r#"<SignedInfo xmlns="http://www.w3.org/2000/09/xmldsig#"><CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/><SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><Reference URI="{hash_id_dps}"><Transforms><Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature"/><Transform Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/></Transforms><DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><DigestValue>{digest_value}</DigestValue></Reference></SignedInfo>"#
   );

   // =========================================================================
   // 4. CÁLCULO DO SIGNATUREVALUE (ASSINATURA RSA-SHA1 DO <SignedInfo>)
   // =========================================================================
   let mut signer = Signer::new(MessageDigest::sha1(), &private_key).expect("Erro ao instanciar Signer");
   signer.set_rsa_padding(Padding::PKCS1).expect("Erro ao definir padding PKCS#1");
   signer.update(signed_info_xml.as_bytes()).expect("Erro ao carregar SignedInfo no assinador");

   let signature_bytes = signer.sign_to_vec().expect("Falha ao assinar SignedInfo");
   let signature_value = general_purpose::STANDARD.encode(signature_bytes);

   // =========================================================================
   // 5. MONTAGEM DA TAG <Signature>
   // =========================================================================
   let assinatura_xml = format!(
      r#"<Signature xmlns="http://www.w3.org/2000/09/xmldsig#">{signed_info_xml}<SignatureValue>{signature_value}</SignatureValue><KeyInfo><X509Data><X509Certificate>{x509_certificate_base64}</X509Certificate></X509Data></KeyInfo></Signature>"#
   );

   assinatura_xml

    */
}