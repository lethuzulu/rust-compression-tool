use serde::{Deserialize, Serialize};
use std::cmp::Eq;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::binary_heap;
use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::env::args;
use std::fmt::Debug;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Output;

const DELIMITER: &[u8] = b"__END__";

#[derive(Serialize, Deserialize, Debug)]
pub enum HuffmanTree {
    InternalNode {
        weight: i32,
        right: Box<HuffmanTree>,
        left: Box<HuffmanTree>,
    },
    LeafNode {
        weight: i32,
        element: char,
    },
}

impl HuffmanTree {
    fn new_leaf(weight: i32, element: char) -> Self {
        HuffmanTree::LeafNode { weight, element }
    }

    fn new_internal(weight: i32, right: Box<HuffmanTree>, left: Box<HuffmanTree>) -> Self {
        HuffmanTree::InternalNode {
            weight,
            right,
            left,
        }
    }

    fn weight(&self) -> i32 {
        match self {
            HuffmanTree::InternalNode { weight, .. } => *weight,
            HuffmanTree::LeafNode { weight, .. } => *weight,
        }
    }

    fn value(&self) -> Option<char> {
        match self {
            HuffmanTree::LeafNode { element, .. } => Some(*element),
            HuffmanTree::InternalNode { .. } => None,
        }
    }
}

impl Ord for HuffmanTree {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.weight() < other.weight() {
            Ordering::Greater
        } else if self.weight() == other.weight() {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for HuffmanTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffmanTree {
    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

impl Eq for HuffmanTree {}

pub fn read_file(input: &String) -> Result<String, Error> {
    let input_path = Path::new(input);
    let mut file = OpenOptions::new().read(true).open(input_path)?;

    let mut contents = String::new();

    let contents_length = file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn generate_frequency_table(string: &String) -> HashMap<char, i32> {
    let mut frequency_table: HashMap<char, i32> = HashMap::new();
    for char in string.chars() {
        let entry = frequency_table.entry(char);
        let value = entry.or_insert(0);
        *value += 1;
    }
    frequency_table
}

fn build_tree(binary_heap: &mut BinaryHeap<HuffmanTree>) {
    while binary_heap.len() > 1 {
        let right_node = binary_heap.pop().unwrap();
        let left_node = binary_heap.pop().unwrap();

        let node_weight = right_node.weight() + left_node.weight();

        let internal_node =
            HuffmanTree::new_internal(node_weight, Box::new(right_node), Box::new(left_node));
        binary_heap.push(internal_node);
    }
}

pub fn generate_huffman_tree(frequency_table: &HashMap<char, i32>) -> Option<HuffmanTree> {
    let mut binary_heap: BinaryHeap<HuffmanTree> = BinaryHeap::new(); //priority queue

    for (char, count) in frequency_table.iter() {
        let huffman_tree_leaf = HuffmanTree::new_leaf(*count, *char);
        binary_heap.push(huffman_tree_leaf);
    }

    build_tree(&mut binary_heap);

    let huffman_tree = binary_heap.pop();
    huffman_tree
}

pub fn generate_huffman_code(
    huffman_tree: &HuffmanTree,
    encoding_table: &mut BTreeMap<char, String>,
    prefix: String,
) {
    match huffman_tree {
        HuffmanTree::LeafNode { weight, element } => {
            encoding_table.insert(*element, prefix);
        }
        HuffmanTree::InternalNode {
            weight,
            right,
            left,
        } => {
            generate_huffman_code(&left, encoding_table, prefix.clone() + "0");
            generate_huffman_code(&right, encoding_table, prefix + "1");
        }
    }
}

pub fn encode_text(string: &String, encoding_table: &BTreeMap<char, String>) -> String {
    let mut encoded_string = String::new();

    for char in string.chars() {
        let s = encoding_table.get(&char).unwrap();
        encoded_string.push_str(s);
    }
    encoded_string
}

fn pack_bits(bit_str: &str) -> Vec<u8> {
    // Calculate the number of bytes needed
    let num_bytes = (bit_str.len() + 7) / 8;

    // Initialize a vector to store the bytes
    let mut bytes = vec![0u8; num_bytes];

    // Iterate over the bit string and fill the bytes vector
    for (i, c) in bit_str.chars().enumerate() {
        if c == '1' {
            // Determine the byte index and bit position within the byte
            let byte_index = i / 8;
            let bit_position = 7 - (i % 8);
            bytes[byte_index] |= 1 << bit_position;
        }
    }
    bytes
}

pub fn unpack_bits(packed_bytes: &[u8]) -> Vec<u8> {
    // This will hold the unpacked bytes
    let mut unpacked_bytes = Vec::new();

    // Calculate the total number of bits to process
    let total_bits = packed_bytes.len() * 8;

    // Iterate over all bits in packed bytes
    for bit_pos in 0..total_bits {
        // Determine the index of the byte and bit position within that byte
        let byte_index = bit_pos / 8;
        let bit_index = bit_pos % 8;

        // Extract the bit from the packed bytes
        let bit = (packed_bytes[byte_index] >> (7 - bit_index)) & 1;

        // Append the bit to the appropriate byte in unpacked_bytes
        if bit_pos % 8 == 0 {
            // Start a new byte if necessary
            unpacked_bytes.push(0);
        }

        // Calculate the position within the current byte in unpacked_bytes
        let unpacked_byte_index = unpacked_bytes.len() - 1;
        unpacked_bytes[unpacked_byte_index] |= bit << (7 - (bit_pos % 8));
    }

    unpacked_bytes
}

pub fn serialize_tree(huffman_tree: &HuffmanTree) -> Vec<u8> {
    let serialized_tree = bincode::serialize(huffman_tree).unwrap();
    serialized_tree
}

pub fn deserialize_tree(serialized_tree: &Vec<u8>) -> HuffmanTree {
    let deserialized_tree: HuffmanTree = bincode::deserialize(serialized_tree).unwrap();

    deserialized_tree
}

pub fn write_output(output: &String, serialized_tree: &Vec<u8>, encoded_text: &String) {
    let output_path = Path::new(output);

    let mut output_file = File::create(output_path).unwrap();

    output_file.write_all(&serialized_tree).unwrap();

    output_file.write_all(DELIMITER).unwrap();

    output_file.write_all(encoded_text.as_bytes()).unwrap();
}

pub fn read_output(output: &String) -> Vec<u8> {
    let output_path = Path::new(output);
    let mut output_file = File::open(output_path).unwrap();
    let mut v: Vec<u8> = Vec::new();
    output_file.read_to_end(&mut v).unwrap();
    v
}
