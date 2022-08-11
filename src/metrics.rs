use crate::encoding::{Encoding, LanguageSymbolProbability};

pub fn minimal_entropy<S>(symbols_probabilities: &[(S, f32)]) -> f32 {

    let sum: f32 = symbols_probabilities.iter().map(|(_, p)| *p * f32::log2(*p)).sum();

    return 0.0 - sum;
}

pub fn code_entropy(symbols_probabilities: &LanguageSymbolProbability, encoding: &Encoding) -> f32 {
    return symbols_probabilities.iter().map(|(s, p)| *p * encoding.get(&s).unwrap().len() as f32).sum();
}