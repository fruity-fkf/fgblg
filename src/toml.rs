use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub theme: String,
    pub template: String,
    pub highlight_js: String,
}
