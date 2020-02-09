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

pub type OptionalConfigFieldList = Option<Vec<ConfigField>>;

#[derive(Deserialize)]
pub struct StorkFile {
    pub path: String,
    pub url: String,
    pub title: String,
    pub fields: OptionalConfigFieldList
}

#[derive(Deserialize)]
pub struct ConfigOutput {
    pub filename: String,
    pub debug: Option<bool>,
}

#[derive(Deserialize)]
pub struct ConfigField {
    pub key: String,
    pub val: String,
}