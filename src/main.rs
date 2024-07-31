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
use std::path::Path;
use std::process::Output;

mod config;
use config::Config;

#[derive(Debug)]
enum HuffmanTree {
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

fn read_file(config: &Config) -> Result<String, Error> {
    let input = &config.input;

    let input_path = Path::new(input);
    let mut file = OpenOptions::new().read(true).open(input_path)?;

    let mut contents = String::new();

    let contents_length = file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn generate_frequency_table(string: String) -> HashMap<char, i32> {
    let mut frequency_table: HashMap<char, i32> = HashMap::new();

    for char in string.chars() {
        let entry = frequency_table.entry(char);
        let value = entry.or_insert(0);
        *value += 1;
    }
    frequency_table
}

fn generate_huffman_tree(binary_heap: &mut BinaryHeap<HuffmanTree>) {
    while binary_heap.len() > 1 {
        let right_node = binary_heap.pop().unwrap();
        let left_node = binary_heap.pop().unwrap();

        let node_weight = right_node.weight() + left_node.weight();

        let internal_node =
            HuffmanTree::new_internal(node_weight, Box::new(right_node), Box::new(left_node));
        binary_heap.push(internal_node);
    }
}

fn generate_huffman(frequency_table: HashMap<char, i32>) -> Option<HuffmanTree> {
    let mut binary_heap: BinaryHeap<HuffmanTree> = BinaryHeap::new(); //priority queue

    for (char, count) in frequency_table.iter() {
        let huffman_tree_leaf = HuffmanTree::new_leaf(*count, *char);
        binary_heap.push(huffman_tree_leaf);
    }

    generate_huffman_tree(&mut binary_heap);

    let huffman_tree = binary_heap.pop();
    huffman_tree
}

// fn generate_huffman_code(huffman_tree: HuffmanTree, ) {
//     let prefix = String::new();
//     let mut codes: BTreeMap<char, String> = BTreeMap::new();

//     match huffman_tree {
//         HuffmanTree::LeafNode { element, .. } => {
//             codes.insert(*element, prefix);
//         }
//         HuffmanTree::InternalNode {
//             weight,
//             right,
//             left,
//         } => {
//             generate_huffman_code(left, prefix.clone() + "0", codes);
//             generate_huffman_code(right, prefix + "1", codes);
//         }
//     }
// }

fn generate_huffman_code(
    huffman_tree: HuffmanTree,
    codes: &mut BTreeMap<char, String>,
    prefix: String,
) {
    match huffman_tree {
        HuffmanTree::LeafNode { weight, element } => {
            codes.insert(element, prefix);
        }
        HuffmanTree::InternalNode {
            weight,
            right,
            left,
        } => {
            generate_huffman_code(*left, prefix.clone() + "0");
            generate_huffman_code(*right, prefix + "1");
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::build_config(&args).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1)
    });

    let string = read_file(&config).unwrap_or_else(|err| {
        println!("{:?}", err);
        std::process::exit(1)
    });

    let frequency_table = generate_frequency_table(string);

    let huffman_tree = generate_huffman(frequency_table).unwrap();

    let mut codes: BTreeMap<char, String> = BTreeMap::new();
    generate_huffman_code(huffman_tree, &mut codes, String::new());
    // println!("{:?}", codes);

    // let huffman_tree = heap.pop().unwrap();
    // let mut codes: BTreeMap<char, String> = BTreeMap::new();
    // let prefix = String::new();

    // generate_huffman_code(&huffman_tree, prefix, &mut codes);

    // println!("{:?}", codes);
}
