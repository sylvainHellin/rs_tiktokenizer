use clap::Parser;

#[derive(Parser)]
struct Args {
    files: Vec<String>, // Positional - no flag
    #[arg(short, long, default_value = "cl100k_base")]
    tokenizer: String, // to pass after the -t or --tokenizer flag
}

fn main() {
    let args = Args::parse();
    let files = args.files;
    let tokenizer = args.tokenizer;

    println!("Files: {files:#?}");
    println!("Tokenizer: {tokenizer}")
}
