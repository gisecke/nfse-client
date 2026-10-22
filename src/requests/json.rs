use reqwest::{Client, Response};

pub async fn get_cep_json(cep: String) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let resp = client
        .get(&format!("https://viacep.com.br/ws/{}/json/", cep))
        .send()
        .await?;
    Ok(resp)
}