#[derive(Debug)]
pub struct Config {
    pub input: String,
    pub output: String,
}

impl Config {
    pub fn build_config(args: &Vec<String>) -> Result<Self, String> {
        if args.len() < 3 {
            return Err("Insufficeint Arguments.".to_string());
        }
        let input = args[1].clone();
        let output = args[2].clone();
        Ok(Self { input, output })
    }
}
