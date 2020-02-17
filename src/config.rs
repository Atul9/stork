use std::collections::HashMap;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Config {
    pub input: ConfigInput,
    pub output: ConfigOutput,
}

#[derive(Deserialize)]
pub struct ConfigInput {
    pub surrounding_word_count: Option<u8>,
    pub base_directory: String,
    pub files: Option<Vec<StorkFile>>,
}

#[derive(Deserialize)]
pub struct StorkFile {
    pub path: String,
    pub url: String,
    pub title: String,
    pub fields: Option<HashMap<String, String>>
}

#[derive(Deserialize)]
pub struct ConfigOutput {
    pub filename: String,
    pub debug: Option<bool>,
}