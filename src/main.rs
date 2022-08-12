mod huffman;
mod encoding;
mod metrics;

use huffman::get_huffman_code;
use metrics::{minimal_entropy, code_entropy};

use crate::encoding::{encode_text, LanguageSymbolProbability, get_letter_probabilities, decode_text};

fn main() {
    let example_text = "This is some text for testing huffman codes";
    println!("Text: {}", example_text);

    let probabilities: LanguageSymbolProbability = get_letter_probabilities(example_text);
    println!("Letter probabilities in text: {:?}", &probabilities);
    let entropy = minimal_entropy(&probabilities);
    println!("Entropy per character: {}", &entropy);

    let huffman_encoding = get_huffman_code(&probabilities);
    println!("Huffman encoding for text: {:?}", &huffman_encoding);
    let code_entropy = code_entropy(&probabilities, &huffman_encoding);
    println!("Entropy per character of code: {}", &code_entropy);
    
    let encoded_text = encode_text(example_text, &huffman_encoding);
    println!("encoded text: {}", encoded_text);

    let decoded_text = decode_text(&encoded_text, &huffman_encoding);
    println!("decoded text: {}", decoded_text);
}
