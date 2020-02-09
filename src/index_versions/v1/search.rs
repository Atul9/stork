use std::iter::FromIterator;
use std::collections::HashMap;
use crate::index_versions::v1::index_models::*;
use crate::index_versions::v1::index_helpers::*;
use crate::utils::get_index_version;

pub fn perform_search(index: &[u8], query: &String) -> Vec<StorkOutput> {
    let normalized_query = query.to_lowercase();
    let mut words_in_query = normalized_query.split(" "); // not sure this needs to be mutable

    let mut first_results = perform_word_lookup(index, &words_in_query.next().unwrap().to_string());

    for query_word in words_in_query {
        for result in &mut first_results {
            result.excerpts = result
                .excerpts
                .iter()
                .filter(|e| e.value.contains(query_word))
                .cloned()
                .collect();
        }
    }

    first_results = first_results
        .iter()
        .filter(|&r| !r.excerpts.is_empty())
        .cloned()
        .collect();

    let entries = get_index_entries(index);
    let mut output_map: HashMap<usize, StorkOutput> = HashMap::new();
    for mut result in first_results {
        output_map
            .entry(result.file_index as usize)
            .and_modify(|e| e.result.excerpts.append(&mut result.excerpts))
            // and modify score, too
            .or_insert(StorkOutput {
                entry: entries[result.file_index as usize].clone(),
                result: result,
            });
    }

    let mut output_vector = Vec::from_iter(output_map.values().cloned());
    // eventually sort by score instead
    output_vector.sort_by_key(|o| o.result.file_index);
    // return serde_json::to_string(&output_vector);
    return output_vector;
}

fn perform_word_lookup(index: &[u8], query: &String) -> Vec<StorkResult> {
    let version = get_index_version(index);

    let full_results = get_index_results(index);

        let query_result: Vec<StorkResultOrAlias> = full_results
            .get(query)
            .unwrap_or(&Vec::new())
            .to_owned()
            .to_vec();

        return expand_aliases_to_results(&full_results, &query_result);
}


fn expand_aliases_to_results(
    full_results: &HashMap<String, Vec<StorkResultOrAlias>>,
    results_aliases: &Vec<StorkResultOrAlias>,
) -> Vec<StorkResult> {
    if results_aliases.len() == 0 {
        return vec![];
    }

    let mut output: Vec<StorkResult> = Vec::new();

    for sroa in results_aliases {
        if let StorkResultOrAlias::Result(r) = sroa {
            output.push(r.clone());
        } else if let StorkResultOrAlias::Alias(a) = sroa {
            let empty_vec = Vec::new();
            let alias_pointee = full_results.get(a).unwrap_or(&empty_vec);
            let expanded_inner_results = expand_aliases_to_results(full_results, alias_pointee);
            for inner_result in expanded_inner_results {
                output.push(inner_result);
            }
        }
    }

    return output;
}
