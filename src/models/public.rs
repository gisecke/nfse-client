use askama::Template;

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