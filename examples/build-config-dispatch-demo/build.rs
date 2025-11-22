fn main() {
    // Generate version information at build time
    // This will be available via the version!() macro
    sw_cli::define_build_info!();
}
