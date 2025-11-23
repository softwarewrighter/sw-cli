use chrono::{DateTime, Utc};
use std::fmt;

/// Build information captured at compile time
#[derive(Debug, Clone)]
pub struct BuildInfo {
    /// Hostname where the binary was built
    pub build_host: String,
    /// Git commit SHA of the build
    pub commit_sha: String,
    /// Build timestamp in milliseconds since epoch
    pub build_timestamp_ms: i64,
}

impl BuildInfo {
    /// Create a new `BuildInfo` instance
    #[must_use]
    pub fn new(build_host: String, commit_sha: String, build_timestamp_ms: i64) -> Self {
        Self {
            build_host,
            commit_sha,
            build_timestamp_ms,
        }
    }
}

impl fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = DateTime::<Utc>::from_timestamp_millis(self.build_timestamp_ms)
            .unwrap_or(DateTime::<Utc>::UNIX_EPOCH);
        let short_sha = if self.commit_sha.len() > 7 {
            &self.commit_sha[..7]
        } else {
            &self.commit_sha
        };
        write!(
            f,
            "Build: {} @ {} ({})",
            short_sha,
            self.build_host,
            datetime.to_rfc3339()
        )
    }
}

/// Version information for the CLI application
#[derive(Debug, Clone)]
pub struct Version {
    /// Semantic version number (e.g., "0.1.0")
    pub version: String,
    /// Copyright notice
    pub copyright: String,
    /// License name (e.g., "MIT", "Apache-2.0")
    pub license_name: String,
    /// URL to the LICENSE file on GitHub
    pub license_url: String,
    /// Build information
    pub build_info: BuildInfo,
}

impl Version {
    /// Create a new Version instance
    #[must_use]
    pub fn new(
        version: String,
        copyright: String,
        license_name: String,
        license_url: String,
        build_info: BuildInfo,
    ) -> Self {
        Self {
            version,
            copyright,
            license_name,
            license_url,
            build_info,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Version: {}", self.version)?;
        writeln!(f, "{}", self.copyright)?;
        writeln!(f, "{} License: {}", self.license_name, self.license_url)?;
        write!(f, "{}", self.build_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_info_display() {
        let build_info = BuildInfo::new(
            "builder.local".to_string(),
            "abc123def456".to_string(),
            1700000000000,
        );

        let output = format!("{}", build_info);
        // 1700000000000ms = 2023-11-14T22:13:20Z, SHA shortened to 7 chars
        assert_eq!(
            output,
            "Build: abc123d @ builder.local (2023-11-14T22:13:20+00:00)"
        );
    }

    #[test]
    fn test_version_display() {
        let build_info = BuildInfo::new(
            "builder.local".to_string(),
            "abc123def456".to_string(),
            1700000000000,
        );

        let version = Version::new(
            "0.1.0".to_string(),
            "Copyright (c) 2025 Example Corp".to_string(),
            "MIT".to_string(),
            "https://github.com/example/repo/blob/main/LICENSE".to_string(),
            build_info,
        );

        let output = format!("{}", version);
        assert!(output.contains("Version: 0.1.0"));
        assert!(output.contains("Copyright (c) 2025 Example Corp"));
        assert!(output.contains("MIT License: https://github.com/example/repo"));
        assert!(output.contains("Build: abc123d @ builder.local"));
    }
}
