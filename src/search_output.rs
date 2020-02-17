use std::collections::HashMap;
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Highlight {
    pub offset: u32,
    pub length: u32
}
 
#[derive(Serialize, Debug)]
pub struct Excerpt {
    pub text: String,
    pub highlights: Vec<Highlight>,
    pub score: u8
}

#[derive(Serialize, Debug)]
pub struct Entry {
    pub url: String,
    pub title: String,
    pub fields: Option<HashMap<String, String>>,
    pub excerpts: Vec<Excerpt>,
    pub score: u8
}