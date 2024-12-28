use std::sync::LazyLock;

use scraper::{Html, Selector};
use anyhow::{anyhow, Result};

static HEADER_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".main-page-content > header:nth-child(1)").unwrap());

static DESCRIPTION_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".main-page-content > div:nth-child(2)").unwrap());

pub fn extract_infomation(document: Html) -> Result<String> {
    let header = document
        .select(&HEADER_SELECTOR)
        .next()
        .ok_or(anyhow!("No header"))?
        .html();
    let description = document
        .select(&DESCRIPTION_SELECTOR)
        .next()
        .ok_or(anyhow!("No description"))?
        .html();

    Ok(format!("{}\n{}", header, description))
}
