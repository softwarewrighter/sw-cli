use crate::command::Command;
use crate::config::CliConfig;
use crate::version::{BuildInfo, Version};
use std::error::Error;

pub struct VersionCommand;

impl Command for VersionCommand {
    fn can_handle(&self, config: &dyn CliConfig) -> bool {
        config.wants_version()
    }

    fn execute(&self, _config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
        // Include the generated version_info.rs
        mod version_info {
            include!(concat!(env!("OUT_DIR"), "/version_info.rs"));
        }

        let build_info = BuildInfo::new(
            version_info::BUILD_HOST.to_string(),
            version_info::GIT_COMMIT_SHA.to_string(),
            version_info::BUILD_TIMESTAMP,
        );

        let version_obj = Version::new(
            version_info::VERSION.to_string(),
            version_info::COPYRIGHT.to_string(),
            version_info::LICENSE_NAME.to_string(),
            version_info::LICENSE_URL.to_string(),
            build_info,
        );

        println!("{version_obj}");
        Ok(())
    }

    fn priority(&self) -> u8 {
        0
    }
}
