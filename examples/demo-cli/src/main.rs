fn main() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Handle -V or --version flags
    if args.len() > 1 && (args[1] == "-V" || args[1] == "--version") {
        println!("{}", sw_cli::version!());
        return;
    }

    // Normal CLI operation
    println!("Demo CLI Application");
    println!("Run with -V or --version to see version information");
}
