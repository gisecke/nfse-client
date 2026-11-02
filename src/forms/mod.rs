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
    pub serie: String,
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

pub struct Dps {
    pub id: String,
    pub homologacao_ou_producao: String,
    pub data_hora_local: String,
    pub serie: String,
    pub numero: String,
    pub localidade: String,
}

pub struct Prestador {
    pub cnpj: String,
    pub im: String,
}

pub struct Tomador {
    pub cadastro: String,
    pub razao_social: String,
    pub codigo_municipal: String,
    pub cep: String,
    pub logradouro: String,
    pub nro_end: String,
    pub bairro: String,
}

pub struct Tributacao {
    pub localidade: String,
    pub cod_trib_nacional: String,
    pub cod_trib_municipal: String,
    pub cod_nbs: String,
    pub tp_ret_issqn: String,
}

pub struct Servico {
    pub data: String,
    pub descricao: String,
    pub valor: String,
}

