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