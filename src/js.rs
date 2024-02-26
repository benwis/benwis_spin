use crate::errors::BenwisAppError;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

///Recreate HtmlOutput so we don't have to import it from server only crates
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HtmlOutput {
    pub toc: Option<String>,
    pub content: String,
}

#[wasm_bindgen(raw_module = "/components/femark-wasi/femark.js")]
extern "C" {
    pub fn processMarkdownToHtml(input: String) -> JsValue;
}

pub fn process_markdown_to_html(input: String) -> Result<HtmlOutput, BenwisAppError> {
    let jsval = processMarkdownToHtml(input);
    Ok(serde_wasm_bindgen::from_value(jsval)?)
}
