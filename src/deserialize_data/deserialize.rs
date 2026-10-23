use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename = "ConsultarDadosCadastraisResposta")]
pub struct ConsultarDadosCadastraisResposta {
    #[serde(rename = "Cadastro")]
    pub cadastro: Option<Cadastro>,

    #[serde(rename = "ListaMensagemRetorno")]
    pub lista_mensagem_retorno: Option<ListaMensagemRetorno>,
}

#[derive(Debug, Deserialize)]
pub struct Cadastro {
    #[serde(rename = "CNPJ")]
    pub cnpj: Option<String>,
    
    #[serde(rename = "CPF")]
    pub cpf: Option<String>,

    #[serde(rename = "IM")]
    pub im: Option<String>,

    #[serde(rename = "xNome")]
    pub x_nome: Option<String>,

    #[serde(rename = "StatusCadastro")]
    pub status_cadastro: Option<String>,

    #[serde(rename="enderNac")]
    pub endereco: Option<Endereco>,

    #[serde(rename="fone")]
    pub telefone: Option<String>,

    #[serde(rename="email")]
    pub email: Option<String>,

}

#[derive(Debug, Deserialize)]
pub struct Endereco {
    #[serde(rename="xLgr")]
    pub rua: Option<String>,

    #[serde(rename="nro")]
    pub numero: Option<String>,

    #[serde(rename="xBairro")]
    pub bairro: Option<String>,

    #[serde(rename="cMun")]
    pub cidade: Option<String>,

    #[serde(rename="UF")]
    pub estado: Option<String>,

    #[serde(rename="CEP")]
    pub cep: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListaMensagemRetorno {
    #[serde(rename = "MensagemRetorno")]
    pub mensagem_retorno: Vec<MensagemRetorno>,
}

#[derive(Debug, Deserialize)]
pub struct MensagemRetorno {
    #[serde(rename = "Codigo")]
    pub codigo: String,

    #[serde(rename = "Mensagem")]
    pub mensagem: String,
}

#[derive(Deserialize)]
pub struct MunicipioIbgeRaw {
    #[serde(rename = "municipio-id")]
    pub id: i32,
    #[serde(rename = "municipio-nome")]
    pub nome: String,
    #[serde(rename = "UF-sigla")]
    pub uf:String,
}


#[derive(Clone, Serialize)]
pub struct Municipio {
    pub ibge: String,
    pub nome: String,
    pub uf: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryBusca {
    pub q: Option<String>,
}