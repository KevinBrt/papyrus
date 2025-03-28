use papyrus_cli::args::parse_args;
use papyrus_parser::parser;
use papyrus_renderer::renderer;

fn main() {
    // Démarrer le chronomètre
    let start_time = std::time::Instant::now();

    // Parse args and perform checks
    let args = parse_args();

    println!("Source: {}", args.source);
    println!("Output: {}", args.output);

    // Parse XML file to build Taffy layout
    let parser = parser::Parser::new(args.source);
    parser.parse();

    // Call papyrus-renderer to render the PDF
    let pdf = renderer::render();

    // Calculer et afficher le temps d'exécution
    let duration = start_time.elapsed();
    println!("Temps d'exécution: {:?}", duration);
}
