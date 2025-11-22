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
    /// Create a new BuildInfo instance
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
        writeln!(f, "Build Information:")?;
        writeln!(f, "  Host: {}", self.build_host)?;
        writeln!(f, "  Commit: {}", self.commit_sha)?;
        writeln!(f, "  Timestamp: {} ms", self.build_timestamp_ms)
    }
}

/// Version information for the CLI application
#[derive(Debug, Clone)]
pub struct Version {
    /// Copyright notice
    pub copyright: String,
    /// URL to the LICENSE file on GitHub
    pub license_url: String,
    /// Build information
    pub build_info: BuildInfo,
}

impl Version {
    /// Create a new Version instance
    pub fn new(copyright: String, license_url: String, build_info: BuildInfo) -> Self {
        Self {
            copyright,
            license_url,
            build_info,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.copyright)?;
        writeln!(f, "License: {}", self.license_url)?;
        writeln!(f)?;
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
        assert!(output.contains("Build Information:"));
        assert!(output.contains("Host: builder.local"));
        assert!(output.contains("Commit: abc123def456"));
        assert!(output.contains("Timestamp: 1700000000000 ms"));
    }

    #[test]
    fn test_version_display() {
        let build_info = BuildInfo::new(
            "builder.local".to_string(),
            "abc123def456".to_string(),
            1700000000000,
        );

        let version = Version::new(
            "Copyright (c) 2024 Example Corp".to_string(),
            "https://github.com/example/repo/blob/main/LICENSE".to_string(),
            build_info,
        );

        let output = format!("{}", version);
        assert!(output.contains("Copyright (c) 2024 Example Corp"));
        assert!(output.contains("License: https://github.com/example/repo"));
        assert!(output.contains("Build Information:"));
    }
}
