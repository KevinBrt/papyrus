use papyrus_cli::args::parse_args;
use papyrus_parser::parser;

fn main() {
    // Parse args and perform checks
    let args = parse_args();

    println!("Source: {}", args.source);
    println!("Output: {}", args.output);

    // Parse XML file to build Taffy layout
    let parser = parser::Parser::new(args.source);
    parser.parse();

    // TODO convert to taffy

    // TODO call renderer with taffy layout and elements map
}
