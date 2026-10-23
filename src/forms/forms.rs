use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConsultarDadosCadastraisForm {
    pub cnpj: String,
    pub cpf: String,
    pub im: String,
}

#[derive(Deserialize)]
pub struct CepForm {
    pub cep: String,
}

#[derive(Deserialize)]
pub struct CadastroForm {
    pub razao_social: String,
}