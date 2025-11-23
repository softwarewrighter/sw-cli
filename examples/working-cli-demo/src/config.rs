use std::path::PathBuf;
use sw_cli::{BaseConfig, CliConfig as CliConfigTrait};

#[derive(Debug, Clone)]
pub struct DemoConfig {
    pub base: BaseConfig,
    pub input: Option<Vec<PathBuf>>,
    pub output: Option<PathBuf>,
    pub pattern: Option<String>,
    pub count: bool,
    pub reverse: bool,
}

impl CliConfigTrait for DemoConfig {
    fn base(&self) -> &BaseConfig {
        &self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
