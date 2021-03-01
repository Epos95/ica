
use reqwest;
use select::document::Document;

pub async fn get_dom(url: String) -> Result<Document, NetworkerErrors> {
    let res = match reqwest::get(url.as_str()).await {
        Ok(s) => s,
        Err(_) => { return Err(NetworkerErrors::NetworkError); }
    };

    let html = match res.text().await {
        Ok(s) => s,
        Err(_) => { return Err(NetworkerErrors::ConversionError); }
    };

    let document = Document::from(html.as_str());

    Ok(document)
}

pub enum NetworkerErrors {
    NetworkError,
    ConversionError,
}
