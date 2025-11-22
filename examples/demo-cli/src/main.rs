use sw_cli::create_version;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Handle -V or --version flags
    if args.len() > 1 && (args[1] == "-V" || args[1] == "--version") {
        let version = create_version!(
            copyright: "Copyright (c) 2025 Software Wrighter",
            license_name: "MIT",
            license_url: "https://github.com/softwarewrighter/sw-cli/blob/main/LICENSE"
        );

        println!("{}", version);
        return;
    }

    // Normal CLI operation
    println!("Demo CLI Application");
    println!("Run with -V or --version to see version information");
}
