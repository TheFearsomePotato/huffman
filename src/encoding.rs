use std::{collections::HashMap};

pub type Encoding = HashMap<char, Vec<bool>>;

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

pub fn encode_text(text: &str, encoding: Encoding) -> String {
    let output: String = text
        .chars()
        .map(|s| {
            encoding
                .get(&s)
                .unwrap()
                .iter()
                .map(|b| if *b { '1' } else { '0' })
                .collect::<String>()
        })
        .collect();

    return output;
}
