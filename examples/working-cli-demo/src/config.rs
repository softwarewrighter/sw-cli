use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BaseConfig {
    pub verbose: bool,
    pub dry_run: bool,
    pub quiet: bool,
    pub help: bool,
    pub version: bool,
    pub input: Option<Vec<PathBuf>>,
    pub output: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct CliConfig {
    pub base: BaseConfig,
    pub pattern: Option<String>,
    pub count: bool,
    pub reverse: bool,
}

impl CliConfig {
    pub fn verbosity(&self) -> u8 {
        if self.base.quiet { 0 } else if self.base.verbose { 1 } else { 0 }
    }

    pub fn is_dry_run(&self) -> bool {
        self.base.dry_run
    }
}
