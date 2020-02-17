use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorkIndex {
    pub entries: Vec<StorkEntry>,
    pub queries: HashMap<String, StorkResultsAndAliases>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StorkEntry {
    pub contents: String,
    pub meta: StorkEntryMetadata
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StorkEntryMetadata {
    pub title: String,
    pub url: String,
    pub fields: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorkResultsAndAliases {
    pub results: HashMap<u16 /* Entry Index */, StorkResult>,
    pub aliases: HashMap<String /* Alias Target */, u8 /* score */>
}

impl StorkResultsAndAliases {
    pub fn new() -> StorkResultsAndAliases {
        StorkResultsAndAliases {
            results: HashMap::new(),
            aliases: HashMap::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorkResult {
    pub indices_within_entry: Vec<u32>,
    pub score: u8,
    pub fields: Option<HashMap<String, String>>,
}

impl StorkResult {
    pub fn new() -> StorkResult {
        StorkResult {
            indices_within_entry: Vec::new(),
            score: 127,
            fields: None
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorkOutput {
    pub entry: StorkEntry,
    pub result: StorkResult,
}