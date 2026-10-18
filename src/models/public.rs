use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    
}

#[derive(Template)]
#[template(path = "consultar-dados-cadastrais.html")]
pub struct ConsultarDadosCadastraisTemplate {
    
}