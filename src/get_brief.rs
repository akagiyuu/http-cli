use http::StatusCode;

use anyhow::Result;

pub fn get_brief(status_code: u16) -> Result<String> {
    let brief = StatusCode::from_u16(status_code)?.to_string();

    Ok(brief)
}
