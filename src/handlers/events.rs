pub fn remove_period_dash_and_slash(cadastro: &str) -> String {
   let mut result = cadastro.to_string();
   result.retain(|c| c != '.' && c != '-' && c != '/');
   result
}