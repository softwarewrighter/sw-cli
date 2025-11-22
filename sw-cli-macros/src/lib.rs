use proc_macro::TokenStream;
use quote::quote;

/// Creates a Version instance with build information captured at compile time.
///
/// # Usage
///
/// ```ignore
/// use sw_cli::create_version;
///
/// let version = create_version!(
///     copyright: "Copyright (c) 2025 Your Name",
///     license_name: "MIT",
///     license_url: "https://github.com/yourusername/repo/blob/main/LICENSE"
/// );
///
/// println!("{}", version);
/// ```
///
/// The macro automatically captures:
/// - Build hostname from the `BUILD_HOST` environment variable
/// - Git commit SHA from the `GIT_COMMIT_SHA` environment variable
/// - Build timestamp from the `BUILD_TIMESTAMP` environment variable
///
/// These should be set in your build.rs script.
#[proc_macro]
pub fn create_version(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    // Parse the input to extract copyright, license_name and license_url
    let mut copyright = None;
    let mut license_name = None;
    let mut license_url = None;

    for pair in input_str.split(',') {
        let pair = pair.trim();
        if let Some(value) = pair.strip_prefix("copyright:") {
            copyright = Some(value.trim().trim_matches('"'));
        } else if let Some(value) = pair.strip_prefix("license_name:") {
            license_name = Some(value.trim().trim_matches('"'));
        } else if let Some(value) = pair.strip_prefix("license_url:") {
            license_url = Some(value.trim().trim_matches('"'));
        }
    }

    let copyright = copyright.expect("copyright field is required");
    let license_name = license_name.expect("license_name field is required");
    let license_url = license_url.expect("license_url field is required");

    let expanded = quote! {
        {
            let build_info = ::sw_cli::version::BuildInfo::new(
                ::std::env!("BUILD_HOST").to_string(),
                ::std::env!("GIT_COMMIT_SHA").to_string(),
                ::std::env!("BUILD_TIMESTAMP").parse().expect("BUILD_TIMESTAMP must be a valid i64"),
            );

            ::sw_cli::version::Version::new(
                ::std::env!("CARGO_PKG_VERSION").to_string(),
                #copyright.to_string(),
                #license_name.to_string(),
                #license_url.to_string(),
                build_info,
            )
        }
    };

    TokenStream::from(expanded)
}

/// Defines build-time environment variables for version information.
///
/// This macro should be called in your build.rs file to capture build metadata.
///
/// # Usage in build.rs
///
/// ```ignore
/// use sw_cli::define_build_info;
///
/// fn main() {
///     define_build_info!();
/// }
/// ```
///
/// This will automatically set:
/// - `BUILD_HOST` - hostname where the build occurred
/// - `GIT_COMMIT_SHA` - current git commit SHA
/// - `BUILD_TIMESTAMP` - milliseconds since epoch
#[proc_macro]
pub fn define_build_info(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        {
            use std::process::Command;

            // Get hostname
            let hostname = Command::new("hostname")
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            println!("cargo:rustc-env=BUILD_HOST={}", hostname);

            // Get git commit SHA
            let commit_sha = Command::new("git")
                .args(["rev-parse", "HEAD"])
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            println!("cargo:rustc-env=GIT_COMMIT_SHA={}", commit_sha);

            // Get build timestamp
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

            // Re-run if git HEAD changes
            println!("cargo:rerun-if-changed=.git/HEAD");
        }
    };

    TokenStream::from(expanded)
}
