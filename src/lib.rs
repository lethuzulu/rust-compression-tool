use serde::{Deserialize, Serialize};
use std::cmp::Eq;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::os::windows::fs::MetadataExt;
use std::path::Path;



const DELIMITER: u8 = 0xFF; // Delimiter to separate tree from compressed data

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
    let _ = file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn generate_frequency_table(string: &String) -> Result<HashMap<char, i32>, &'static str> {
    if string.is_empty() {
        return Err("Input String is Empty.");
    }
    let mut frequency_table: HashMap<char, i32> = HashMap::new();
    for char in string.chars() {
        let entry = frequency_table.entry(char);
        let value = entry.or_insert(0);
        *value += 1;
    }
    Ok(frequency_table)
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

pub fn generate_huffman_tree(
    frequency_table: &HashMap<char, i32>,
) -> Result<HuffmanTree, &'static str> {
    if frequency_table.is_empty() {
        return Err("Frequency table is empty.");
    }
    let mut binary_heap: BinaryHeap<HuffmanTree> = BinaryHeap::new(); //priority queue

    for (char, count) in frequency_table.iter() {
        let huffman_tree_leaf = HuffmanTree::new_leaf(*count, *char);
        binary_heap.push(huffman_tree_leaf);
    }

    build_tree(&mut binary_heap);

    let huffman_tree = binary_heap.pop();
    huffman_tree.ok_or("Failed to build Huffma Tree")
}

pub fn generate_huffman_code(huffman_tree: &HuffmanTree) -> BTreeMap<char, String> {
    fn recurse(
        huffman_tree: &HuffmanTree,
        encoding_table: &mut BTreeMap<char, String>,
        prefix: String,
    ) {
        match huffman_tree {
            HuffmanTree::LeafNode { element, .. } => {
                encoding_table.insert(*element, prefix);
            }
            HuffmanTree::InternalNode { right, left, .. } => {
                recurse(&left, encoding_table, prefix.clone() + "0");
                recurse(&right, encoding_table, prefix + "1");
            }
        }
    }

    let mut encoding_table: BTreeMap<char, String> = BTreeMap::new();

    recurse(huffman_tree, &mut encoding_table, String::new());
    encoding_table
}

pub fn encode_text(string: &String, encoding_table: &BTreeMap<char, String>) -> (String, usize) {
    let mut encoded_string = String::new();

    for char in string.chars() {
        let s = encoding_table.get(&char).unwrap();
        encoded_string.push_str(s);
    }
    let length = encoded_string.len();

    (encoded_string, length)
}

pub fn pack_bits(bit_str: &str, length: usize) -> Vec<u8> {
    // Calculate the number of bytes needed
    let num_bytes = (bit_str.len() + 7) / 8;

    // Initialize a vector to store the bytes
    let mut bytes = vec![0u8; num_bytes + 4]; //extra 4 bytes for length

    //store length as 4 bytes
    let length_bytes = (length as u32).to_be_bytes();
    bytes[..4].copy_from_slice(&length_bytes);

    // Iterate over the bit string and fill the bytes vector
    for (i, c) in bit_str.chars().enumerate() {
        if c == '1' {
            // Determine the byte index and bit position within the byte
            let byte_index = (i + 32) / 8;
            let bit_position = 7 - ((i + 32) % 8);
            bytes[byte_index] |= 1 << bit_position;
        }
    }
    bytes
}

pub fn unpack_bits(packed_bytes: &[u8]) -> (String, usize) {
    let length = u32::from_be_bytes(packed_bytes[..4].try_into().unwrap()) as usize;
    let packed_data = &packed_bytes[4..];

    let mut bit_str = String::new();

    for byte in packed_data {
        for bit_pos in (0..8).rev() {
            let bit = (byte >> bit_pos) & 1;
            bit_str.push(if bit == 1 { '1' } else { '0' });
        }
    }

    // Adjust the length of the bit string
    bit_str.truncate(length);

    (bit_str, length)
}

pub fn serialize_tree(huffman_tree: &HuffmanTree) -> Vec<u8> {
    let serialized_tree = bincode::serialize(huffman_tree).unwrap();
    serialized_tree
}

pub fn serialize_prefix_table(encoding_table: &BTreeMap<char, String>) -> Vec<u8> {
    let serialized_table = bincode::serialize(encoding_table).unwrap();
    serialized_table
}

pub fn deserialize_tree(serialized_tree: &[u8]) -> HuffmanTree {
    let deserialized_tree: HuffmanTree = bincode::deserialize(serialized_tree).unwrap();
    deserialized_tree
}

pub fn deserialize_prefix_table(serialized_prefix_table: &[u8]) -> HashMap<char, String> {
    let deserialized_tree: HashMap<char, String> =
        bincode::deserialize(serialized_prefix_table).unwrap();
    deserialized_tree
}

pub fn write_output(
    output: &String,
    huffman_tree: &HuffmanTree,
    input_string: &String,
    encoding_table: &BTreeMap<char, String>,
) -> Result<(), std::io::Error> {

    //TODO: Consider separtaing encoding part and writing part into two different fns.
    let output_path = Path::new(output);
    let mut output_file = File::create(output_path)?;

    let serialized_tree = serialize_tree(huffman_tree);

    // let serialized_prefix_table = serialize_prefix_table(encoding_table);

    let (encoded_string, length) = encode_text(&input_string, &encoding_table);
    let encoded_string_bytes = pack_bits(&encoded_string, length);

    output_file.write_all(&serialized_tree)?;
    // output_file.write_all(&serialized_prefix_table)?;
    output_file.write_all(&[DELIMITER])?;

    output_file.write_all(&encoded_string_bytes)?;
    Ok(())
}

pub fn read_output(output: &String) -> Vec<u8> {
    let output_path = Path::new(output);
    let mut output_file = File::open(output_path).unwrap();

    let mut contents: Vec<u8> = Vec::new();
    output_file.read_to_end(&mut contents).unwrap();

    let delimiter_pos = contents.iter().position(|&x| x == DELIMITER).unwrap();

    let tree_bytes = &contents[..delimiter_pos];
    let encoded_bytes = &contents[delimiter_pos + 1..];


    decode_data(&tree_bytes, &encoded_bytes);
    contents
}

fn decode_data(tree_bytes: &[u8], encoded_bytes: &[u8]) -> String {

    let huffman_tree: HuffmanTree = deserialize_tree(tree_bytes);

    let (bit_string, _) = unpack_bits(encoded_bytes);

    let mut decoded_string = String::new();
    let mut current_code = String::new();

    let encoding_table = generate_huffman_code(&huffman_tree);

    let reverse_table: HashMap<String, char> =
        encoding_table.into_iter().map(|(k, v)| (v, k)).collect();

    for bit in bit_string.chars() {
        current_code.push(bit);
        if let Some(character) = reverse_table.get(&current_code) {
            decoded_string.push(*character);
            current_code.clear();
        }
    }
    decoded_string
}

pub fn compare_sizes(input: &str, output: &str){
    let input_metadata = fs::metadata(input).unwrap();
    let input_size = input_metadata.file_size();
    println!("Input File Size {:.2} KB", input_size as f64 / 1024.0);


    let output_metadata = fs::metadata(output).unwrap();
    let output_size = output_metadata.file_size();
    println!("Compressed File Size {:.2} KB", output_size as f64 / 1024.0);

    let size_saved = input_size as f64 - output_size as f64;

    let percentage_saved = (size_saved / input_size as f64) * 100.0;

    println!("Size saved: {:.2} KB", size_saved / 1024.0);
    println!("Percentage saved: {:.2}%", percentage_saved);
}
