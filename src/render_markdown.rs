use std::process::Stdio;

use anyhow::Result;
use tokio::{io::AsyncWriteExt, process::Command};

pub async fn render_markdown(markdown: String) -> Result<()> {
    let mut child = Command::new("glow").stdin(Stdio::piped()).spawn()?;

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(markdown.as_bytes())
        .await?;

    child.wait().await?;

    Ok(())
}
