use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::env::args;
mod config;
use config::Config;
use lib::encode_text;

mod lib;

const DELIMITER: &[u8] = b"__END__";

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::build_config(&args).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1)
    });

    let input_string = lib::read_file(&config.input).unwrap_or_else(|err| {
        println!("{:?}", err);
        std::process::exit(1)
    });

    let frequency_table = lib::generate_frequency_table(&input_string);

    let huffman_tree = lib::generate_huffman_tree(&frequency_table).unwrap();

    let mut encoding_table: BTreeMap<char, String> = BTreeMap::new();
    lib::generate_huffman_code(&huffman_tree, &mut encoding_table, String::new());


    let encoded_text = encode_text(&input_string, &encoding_table);

    let serialized_data = lib::serialize_tree(&huffman_tree);
    lib::write_output(&config.output, &serialized_data, &encoded_text);


    let deserialized_data = lib::read_output(&config.output);

    let deserialized_tree = lib::deserialize_tree(&deserialized_data);      
}
