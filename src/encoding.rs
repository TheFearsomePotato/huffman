use std::{collections::HashMap, thread::current};

pub type Encoding = HashMap<char, String>;

pub type LanguageSymbolProbability = Vec<(char, f32)>;

pub fn get_letter_probabilities(text: &str) -> LanguageSymbolProbability {
    let mut count_map: HashMap<char, u32> = HashMap::new();

    text.chars().for_each(|c| {
        if let Some(num) = count_map.get_mut(&c) {
            *num = *num + 1;
        } else {
            count_map.insert(c, 1);
        }
    });

    let total: f32 = count_map.values().sum::<u32>() as f32;

    let result:LanguageSymbolProbability = count_map.iter().map(|(k, v)| (*k, *v as f32 / total)).collect();

    return result;
}

pub fn encode_text(text: &str, encoding: &Encoding) -> String {
    let output: String = text
        .chars()
        .map(|s| {
            encoding
                .get(&s)
                .unwrap().clone()
        })
        .collect();

    return output;
}

pub fn decode_text(coded_text: &str, encoding: &Encoding) -> String {
    let decode_map: HashMap<String, char> = encoding.iter().map(|(k, v)| (v.clone(), *k)).collect();

    let mut decoded_text = String::new();
    let mut current_code = String::new();

    for bit in coded_text.chars() {
        current_code.push(bit);
        if let Some(c) = decode_map.get(&current_code) {
            decoded_text.push(*c);
            current_code = String::new();
        }
    }

    return decoded_text;
}
