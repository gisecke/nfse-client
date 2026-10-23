use askama::Template;
use chrono::{ NaiveDate};

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    
}

#[derive(Template)]
#[template(path = "consultar-dados-cadastrais.html")]
pub struct ConsultarDadosCadastraisTemplate {
    
}

#[derive(Template)]
#[template(path = "resultado-consultar-dados-cadastrais.html")]
pub struct ResultadoConsultarDadosCadastraisTemplate {
    pub documento: String,
    pub im: String,
    pub status_cadastro: String,
    pub nome: String,
    pub telefone: String,
    pub email: String,
    pub rua: String,
    pub numero: String,
    pub bairro: String,
    pub municipio: String,
    pub estado: String,
    pub cep: String,
}

#[derive(Template)]
#[template(path = "gerar-nfse.html")]
pub struct GerarNFSETemplate {
    pub data_local: NaiveDate,
    pub rua: String,
    pub bairro: String,
    pub cidade: String,
    pub uf: String,
    pub estado: String,
    pub razao_social: String,
}

#[derive(Template)]
#[template(path = "cep.html")]
pub struct CepTemplate {
    pub rua: String,
    pub bairro: String,
    pub cidade: String,
    pub uf: String,
    pub estado: String,
}

#[derive(Template)]
#[template(path = "cnpj.html")]
pub struct CnpjTemplate {
    pub razao_social: String,
}