use std::env::args;
mod config;
use config::Config;

mod lib;

fn main() {
    //Collect command
    let args: Vec<String> = args().collect();

    //Build config and handle errors
    let config = Config::build_config(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1)
    });
    // Read input file and handle errors
    let input_string = lib::read_file(&config.input).unwrap_or_else(|err| {
        eprintln!("{:?}", err);
        std::process::exit(1)
    });

    let frequency_table = lib::generate_frequency_table(&input_string).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    let huffman_tree = lib::generate_huffman_tree(&frequency_table).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    let encoding_table = lib::generate_huffman_code(&huffman_tree);
 
    lib::write_output(
        &config.output,
        &huffman_tree,
        &input_string,
        &encoding_table,
    )
    .unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    lib::read_output(&config.output);

    lib::compare_sizes(&config.input, &config.output);
}
