use reqwest::{Client, Response};

pub async fn get_values_json(url: String) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let resp = client
        .get(url)
        .send()
        .await?;
    Ok(resp)
}