use clap::Parser;
use std::fs::File;
use std::io::Read;
use serde_json::Result; 

mod program;
mod cfg;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[clap(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let program = read_file(&args.file);
    cfg::exec(&program)
}

fn read_file(file: &str) -> String {
    match File::open(file) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content

        },
        Err(error) => {
            panic!("error while handling file {}", error)
        },
    }
}
