
use colored_json::ToColoredJson;
use reqwest::{Method, Url};

pub async fn request(
    url: Url,
    method: Method,
    body: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();

    let response = client.request(method, url)
        .header("Content-Type", "application/json")
        .body(body.unwrap_or_default())
        .send()
        .await?;

    println!("{:?}", response.status());

    if response.status().is_success() {
        let text = response.text().await?;
        let json = text.to_colored_json_auto()?;
        println!("{json}" );
    } else {
        let status = response.status().to_string();
        let json = status.to_colored_json_auto()?;
        println!("Error: {json}");
    }

    Ok(())
}
