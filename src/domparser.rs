use html_parser::{Dom, Result};
use std::fs;

pub struct Viewport {
    pub title: String,
    pub imgurl: String,
    pub coords: String,
    pub comment: String,
    pub status: String,
}

pub fn read_html(file: String) -> Result<String> {
    let html_content: String = fs::read_to_string(&file)
        .expect("Invalid path or file not exists. Check your path and retry.\n");
    let json = Dom::parse(&html_content)?.to_json_pretty()?;
    Ok(json)
}
