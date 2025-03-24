use clap::Parser;
use std::process;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long)]
    pub source: String,

    #[arg(short, long)]
    pub output: String,
}

// Parse the command line arguments
// Check if required args are provided
pub fn parse_args() -> Args {
    let args: Args = Args::parse();

    // check if source file is xml and exist
    if !args.source.ends_with(".xml") {
        eprintln!("Error: Source file must be a valid XML file");
        process::exit(1);
    }

    // check if source file exist
    if !Path::new(&args.source).exists() {
        eprintln!("Error: Source file does not exist");
        process::exit(1);
    }

    // check if output file is a valid file
    if !args.output.ends_with(".pdf") {
        eprintln!("Error: Output file must be a PDF file");
        process::exit(1);
    }

    args
}