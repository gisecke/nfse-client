use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConsultarDadosCadastraisForm {
    pub cadastro: String,
    pub im: String,
}

#[derive(Deserialize)]
pub struct CepForm {
    pub cep: String,
}

#[derive(Deserialize)]
pub struct CadastroForm {
    pub documento: String,
}

#[derive(Debug, Deserialize)]
pub struct DadosGerarNfse {
    pub serie : String,
    pub numero_dps: String,
    pub municipio_serv_prestado: String,
    pub data_local: String,
    pub documento: String,
    pub razao_social: String,
    pub cep: String,
    pub logradouro: String,
    pub nro: String,
    pub xbairro: String,
    pub codigo_ibge_mun_tomador: String,
    pub uf: String,
    pub ctribnac: String,
    pub ctribmun: String,
    pub cnbs: String,
    pub vserv: String,
    pub desc_serv: String,
    pub tp_ret_issqn: String,

}