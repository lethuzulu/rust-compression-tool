use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::env::args;
mod config;
use config::Config;
use lib::{encode_text, pack_bits, unpack_bits};

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
    println!("huffman tree{:?}", huffman_tree);

    let encoding_table = lib::generate_huffman_code(&huffman_tree);
    println!("encoding_table  beofre {:?}", encoding_table);

    let (encoded_string, length) = encode_text(&input_string, &encoding_table);
    println!("encoded_text   {}", encoded_string);
    println!("length  {}", length);

    lib::write_output(&config.output, &huffman_tree, &encoded_string, length);

    let deserialized_data = lib::read_output(&config.output);

    // let deserialized_tree = lib::deserialize_tree(&deserialized_data);
    // println!("deserialized tree    {:?}", deserialized_tree);
}
