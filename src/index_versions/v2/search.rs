// use crate::index_versions::v1::index_models::StorkOutput;
use crate::index_versions::v2::index_helpers::*;
use crate::index_versions::v2::index_models::*;
use crate::search_output::*;
use std::collections::HashMap;

#[derive(Debug)]
struct StorkResultWithEntryIndex {
    entry_index: u16,
    pub indices_within_entry: Vec<u32>,
    pub score: u8,
    pub fields: Option<HashMap<String, String>>,
}

impl StorkResultWithEntryIndex {
    pub fn new(entry_index: u16, stork_result: StorkResult) -> StorkResultWithEntryIndex {
        StorkResultWithEntryIndex {
            entry_index: entry_index,
            indices_within_entry: stork_result.indices_within_entry,
            score: stork_result.score,
            fields: stork_result.fields,
        }
    }
}

pub fn perform_search(index: &[u8], query: &String) -> Vec<Entry> {
    let index_entries = get_index_entries(index);
    let normalized_query = query.to_lowercase();
    let words_in_query: Vec<String> = normalized_query.split(" ").map(|s| s.to_string()).collect();

    let mut output_entries: Vec<Entry> = Vec::new();

    let mut containers = vec![lookup_word(index, &words_in_query[0])];
    let first_container = &containers[0];
    let aliases: Vec<String> = first_container
        .aliases
        .keys()
        .map(|s| s.to_string())
        .collect();
    let mut entries: Vec<StorkResultWithEntryIndex> = Vec::new();

    for a in &aliases {
        // let container = lookup_word(index, &a);
        // aliases.append(&mut container.aliases.keys().map(|s| s.to_string()).collect());
        containers.push(lookup_word(index, &a));
    }

    for c in &containers {
        entries.append(
            &mut first_container
                .results
                .keys()
                .map(|idx| {
                    StorkResultWithEntryIndex::new(
                        *idx,
                        first_container.results.get(idx).unwrap().to_owned(),
                    )
                })
                .collect::<Vec<StorkResultWithEntryIndex>>(),
        );
    }
    println!("{:?}", entries);
    return vec![];

    // for e in first_entries_keys {
    //     let index_entry = &index_entries[*e as usize];
    //     let entry_results = first_container.results.get(e).unwrap();
    //     let excerpts: Vec<Excerpt> = entry_results
    //         .indices_within_entry
    //         .iter()
    //         .map(|idx| {
    //             let uidx = *idx as usize;
    //             let excerpt_vec = &index_entry
    //                 .contents
    //                 .split_whitespace()
    //                 .map(|w| w.to_string())
    //                 .collect::<Vec<String>>()[(uidx - 8)..(uidx + 8)]
    //                 .to_owned();
    //             let excerpt_text = excerpt_vec.join(" ");

    //             return Excerpt {
    //                 text: excerpt_text,
    //                 highlights: vec![],
    //                 score: entry_results.score,
    //             };
    //         })
    //         .collect();
    //     println!("40");

    //     output_entries.push(Entry {
    //         url: index_entry.meta.url.clone(),
    //         title: index_entry.meta.title.clone(),
    //         fields: index_entry.meta.fields.clone(),
    //         excerpts: excerpts,
    //         score: 0,
    //     });
    //     println!("48");
    // }

    // println!("{:?}", top_level_aliases);

    // println!("OE {:?}", output_entries.len());

    // return output_entries;
}

fn lookup_word(index: &[u8], query: &String) -> StorkResultsAndAliases {
    return get_index_results(index).get(query).unwrap().to_owned();
}

// use serde::{Deserialize, Serialize};
//     let entries = get_index_entries(index);

//     let normalized_query = query.to_lowercase();
//     let words_in_query: Vec<String> = normalized_query.split(" ").map(|s| s.to_string()).collect();

//     // Get all the containers

//     let first_word_container = get_container_from_word(index, &words_in_query[0]);

//     let mut aliased_containers_w_score: Vec<(StorkResultsAndAliases, u8)> = first_word_container
//         .aliases
//         .keys()
//         .map(|k| {
//             (
//                 get_container_from_word(index, &k),
//                 first_word_container.aliases.get(k).unwrap().to_owned(),
//             )
//         })
//         .collect::<Vec<(StorkResultsAndAliases, u8)>>();

//     aliased_containers_w_score.sort_by(|a, b| a.1.cmp(&b.1));
//     let aliased_containers: Vec<StorkResultsAndAliases> = aliased_containers_w_score.iter().map(|t| t.0.clone()).collect();

//     // Extract results from first container (aliases from first container will come later)

//     let mut results: Vec<(StorkEntry, StorkResult)> = first_word_container
//         .results
//         .keys()
//         .map(|k| {
//             (
//                 entries[*k as usize].to_owned(),
//                 first_word_container.results.get(k).unwrap().to_owned(),
//             )
//         })
//         .collect::<Vec<(StorkEntry, StorkResult)>>();
//     results.sort_by(|a, b| a.1.score.cmp(&b.1.score));

//     while &aliased_containers.len() > &0 {
//         for alias_container in &aliased_containers {
//             let mut alias_results: Vec<(StorkEntry, StorkResult)> = alias_container
//                 .results
//                 .keys()
//                 .map(|k| {
//                     (
//                         entries[*k as usize].to_owned(),
//                         alias_container.results.get(k).unwrap().to_owned(),
//                     )
//                 })
//                 .collect::<Vec<(StorkEntry, StorkResult)>>();
//             alias_results.sort_by(|a, b| a.1.score.cmp(&b.1.score));
//             results.append(&mut alias_results)
//         }
//     }

//     return vec![];
// }

// fn get_container_from_word(index: &[u8], query: &String) -> StorkResultsAndAliases {
//     return get_index_results(index).get(query).unwrap().to_owned();
// }
