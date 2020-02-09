use console_error_panic_hook;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::{Read, Write};
use std::path::Path;

pub mod config;
use config::*;

mod utils;
use utils::{get_index_version, remove_surrounding_punctuation};

mod index_versions;
use index_versions::v2::index_models::{
    StorkEntry, StorkEntryMetadata, StorkFieldable, StorkIndex, StorkResult, StorkResultsAndAliases,
};

use serde::Serialize;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn search(index: &[u8], query: String) -> String {
    console_error_panic_hook::set_once();
    return serde_json::to_string(&internal_search(index, &query)).unwrap();
}

pub fn internal_search(
    index: &[u8],
    query: &String,
) -> Vec<index_versions::v1::index_models::StorkOutput> {
    let v = get_index_version(&index);
    let function: fn(&[u8], &String) -> Vec<index_versions::v1::index_models::StorkOutput> =
        match v.as_str() {
            "stork-1.0.0" => index_versions::v1::search::perform_search,
            "stork-2" => index_versions::v2::search::perform_search,
            _ => panic!("Unknown index version"),
        };
    return function(index, query);
}

#[wasm_bindgen]
pub fn extract_index_version(index: &[u8]) -> String {
    console_error_panic_hook::set_once();
    return serde_json::to_string(&get_index_version(&index)).unwrap();
}

pub fn build_index(config: &ConfigInput) -> StorkIndex {
    let mut entries: Vec<StorkEntry> = Vec::new();
    let mut queries: HashMap<String, StorkResultsAndAliases> = HashMap::new();

    let base_directory = Path::new(&config.base_directory);

    for stork_file in config.files.as_ref().unwrap_or(&vec![]).iter() {
        let full_pathname = &base_directory.join(&stork_file.path);
        let file = File::open(&full_pathname).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        let _bytes_read = buf_reader.read_to_string(&mut contents);
        let stork_fields = stork_file.fields.to_stork_fields();

        let entry = StorkEntry {
            contents: contents,
            meta: StorkEntryMetadata {
                title: stork_file.title.clone(),
                url: stork_file.url.clone(),
                fields: None, // fields: stork_file.fields,
            },
        };

        entries.push(entry);
    }

    for (entry_index, entry) in entries.iter().enumerate() {
        let words_in_contents: Vec<String> = entry
            .contents
            .split_whitespace()
            .map(|w| w.to_string())
            .collect();

        for (word_index, word) in words_in_contents.iter().enumerate() {
            let normalized_word = remove_surrounding_punctuation(&word.to_lowercase());
            let normalized_word_len = &normalized_word.len();

            let entry_result = queries
                .entry(normalized_word.clone())
                .or_insert(StorkResultsAndAliases::new())
                .results
                .entry(entry_index as u16)
                .or_insert(StorkResult::new());

            entry_result.indices_within_entry.push(word_index as u32);

            for n in 3..*normalized_word_len {
                let substring = &normalized_word.as_str()[0..n].to_string();

                let _alias_score = queries
                    .entry(substring.clone())
                    .or_insert(StorkResultsAndAliases::new())
                    .aliases
                    .entry(normalized_word.clone())
                    .or_insert(127 - (*normalized_word_len - n) as u8);
            }
        }
    }

    return StorkIndex {
        entries: entries,
        queries: queries,
    };
}

pub fn write_index(config: &ConfigOutput, index: StorkIndex) -> usize {
    let file = File::create(&config.filename).unwrap();
    let mut bufwriter = BufWriter::new(file);

    let write_version = b"stork-2";
    if config.debug.unwrap_or(false) {
        let entries_encoded = serde_json::to_string_pretty(&index.entries).unwrap();
        let results_encoded = serde_json::to_string_pretty(&index.queries).unwrap();
        let byte_vectors_to_write = [
            write_version,
            entries_encoded.as_bytes(),
            results_encoded.as_bytes(),
        ];

        for vec in byte_vectors_to_write.iter() {
            let _ = bufwriter.write(vec.len().to_string().as_bytes());
            let _ = bufwriter.write(b"\n");
            let _ = bufwriter.write(vec);
            let _ = bufwriter.write(b"\n\n");
        }

        return 0;
    } else {
        let mut bytes_written: usize = 0;

        let entries_encoded = bincode::serialize(&index.entries).unwrap();
        let results_encoded = bincode::serialize(&index.queries).unwrap();
        let byte_vectors_to_write = [
            write_version,
            entries_encoded.as_slice(),
            results_encoded.as_slice(),
        ];

        for vec in byte_vectors_to_write.iter() {
            bytes_written += bufwriter.write(&(vec.len() as u64).to_be_bytes()).unwrap();
            bytes_written += bufwriter.write(vec).unwrap();
        }

        return bytes_written;
    }
}
