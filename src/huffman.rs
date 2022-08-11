use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap}
};
use crate::encoding::{Encoding, LanguageSymbolProbability};

#[derive(Debug)]
struct HuffmanNode{
    symbol: Option<char>,
    probability: f32,
    child0: Option<Box<HuffmanNode>>,
    child1: Option<Box<HuffmanNode>>,
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.probability == other.probability
    }
}

impl Eq for HuffmanNode {}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.probability.partial_cmp(&other.probability)
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.probability.partial_cmp(&other.probability).unwrap()
    }
}

fn huffman_tree_to_map(root: &HuffmanNode, current_prefix: Vec<bool>) -> Encoding{
    if root.child0 == None && root.child1 == None {
        let mut code_map = HashMap::new();
        code_map.insert(root.symbol.unwrap(), current_prefix);
        return code_map;
    }
    
    let mut map = HashMap::new();
    
    if let Some(child) = &root.child0 {
        let mut child_prefix = current_prefix.clone();
        child_prefix.push(false);
        let child_map = huffman_tree_to_map(&child, child_prefix);
        map.extend(child_map);
    }

    if let Some(child) = &root.child1 {
        let mut child_prefix = current_prefix;
        child_prefix.push(true);
        let child_map = huffman_tree_to_map(&child, child_prefix);
        map.extend(child_map);
    }

    return map;
}

pub fn get_huffman_code(symbol_probabilities: &LanguageSymbolProbability) -> Encoding {
    // convert slice to heap
    let mut huffman_nodes: BinaryHeap<Reverse<HuffmanNode>> =
        BinaryHeap::from_iter(symbol_probabilities.iter().map(|(c, p)| {
            Reverse(HuffmanNode {
                symbol: Some(*c),
                probability: *p,
                child0: None,
                child1: None,
            })
        }));

    while huffman_nodes.len() > 1 {
        let node0 = huffman_nodes.pop().unwrap().0;
        let node1 = huffman_nodes.pop().unwrap().0;

        let new_node = Reverse(HuffmanNode {
            symbol: None,
            probability: node0.probability + node1.probability,
            child0: Some(Box::new(node0)),
            child1: Some(Box::new(node1)),
        });

        huffman_nodes.push(new_node);
    }

    let root_node = huffman_nodes.pop().unwrap().0;

    return huffman_tree_to_map(&root_node, vec![]);
}