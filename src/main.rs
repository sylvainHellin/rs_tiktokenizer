use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser)]
struct Args {
    #[arg(default_value = ".")]
    files: Vec<String>,
    #[arg(short, long, default_value = "cl100k_base")]
    tokenizer: String,
}

fn main() {
    let args = Args::parse();
    let files = args.files;
    // let tokenizer = args.tokenizer;

    for file in &files {
        let content = read_to_string(file);

        match content {
            Ok(content) => {
                let length = content.len();
                println!("Length of file {file} : {length} characters.");
            }
            Err(err) => {
                eprintln!("Error when trying to read the file {file} : {err}")
            }
        }
    }
}
