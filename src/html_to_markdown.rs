use std::sync::LazyLock;

use anyhow::Result;
use htmd::{Element, HtmlToMarkdown};

static MARKDOWN_CONVERTER: LazyLock<HtmlToMarkdown> = LazyLock::new(|| {
    HtmlToMarkdown::builder()
        .add_handler(vec!["a"], |e: Element| Some(e.content.to_string()))
        .skip_tags(vec!["script", "style", "button"])
        .build()
});

pub fn html_to_markdown(raw_html: String) -> Result<String> {
    let markdown = MARKDOWN_CONVERTER.convert(&raw_html)?;

    Ok(markdown)
}
