/// Help type requested by user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HelpType {
    #[default]
    None,
    Short, // -h
    Long,  // --help
}

/// Standard flags common to all Software Wrighter CLIs
#[derive(Debug, Clone, Default)]
pub struct BaseConfig {
    pub verbose: bool,
    pub dry_run: bool,
    pub help: HelpType,
    pub version: bool,
}

impl BaseConfig {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn verbosity(&self) -> u8 {
        u8::from(self.verbose)
    }

    #[must_use]
    pub fn is_dry_run(&self) -> bool {
        self.dry_run
    }

    #[must_use]
    pub fn wants_help(&self) -> bool {
        self.help != HelpType::None
    }

    #[must_use]
    pub fn wants_short_help(&self) -> bool {
        self.help == HelpType::Short
    }

    #[must_use]
    pub fn wants_long_help(&self) -> bool {
        self.help == HelpType::Long
    }
}

/// Trait that all CLI configs must implement
pub trait CliConfig {
    fn base(&self) -> &BaseConfig;

    fn wants_help(&self) -> bool {
        self.base().wants_help()
    }

    fn wants_short_help(&self) -> bool {
        self.base().wants_short_help()
    }

    fn wants_long_help(&self) -> bool {
        self.base().wants_long_help()
    }

    fn wants_version(&self) -> bool {
        self.base().version
    }

    fn verbosity(&self) -> u8 {
        self.base().verbosity()
    }

    fn is_dry_run(&self) -> bool {
        self.base().is_dry_run()
    }

    fn as_any(&self) -> &dyn std::any::Any;
}
