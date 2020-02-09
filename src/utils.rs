use std::convert::TryInto;

pub fn remove_surrounding_punctuation(input: &String) -> String {
    let mut chars: Vec<char> = input.chars().collect();

    while chars.first().unwrap().is_ascii_punctuation() {
        chars.remove(0);
    }

    while chars.last().unwrap().is_ascii_punctuation() {
        chars.pop();
    }

    return chars.into_iter().collect();
}

pub fn get_index_version(index: &[u8]) -> String {
    let (version_size_bytes, rest) = index.split_at(std::mem::size_of::<u64>());
    let version_size = u64::from_be_bytes(version_size_bytes.try_into().unwrap());
    let (version_bytes, _rest) = rest.split_at(version_size as usize);
    let version = String::from_utf8(version_bytes.to_vec()).unwrap();
    return version;
}