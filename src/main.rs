mod parser;
mod methods;

use std::path::PathBuf;
use clap::Parser;
use crate::methods::find_s::FindS;
use crate::methods::concern::trainable::Trainable;
use crate::methods::concern::predicable::Predicable;
use crate::parser::parse_csv;

#[derive(Parser)]
#[clap(name = "Machine Learning Example - Rust")]
struct Cli {
    #[clap(parse(from_os_str), value_name = "CSV FILE")]
    file_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    println!("using file {:?} as dataset.", cli.file_path);

    let dataset = parse_csv(cli.file_path);

    let mut method = FindS::new(dataset);

    method.train();

    let result = method.predict();

    // println!("{}", style(result).green());
}
