use anyhow::Result;
use scraper::Html;

fn get_information_link(status_code: u16) -> String {
    format!(
        "https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/{}",
        status_code
    )
}

pub async fn get_document(status_code: u16) -> Result<Html> {
    let link = get_information_link(status_code);

    let raw = reqwest::get(link).await?.text().await?;

    Ok(Html::parse_document(&raw))
}
