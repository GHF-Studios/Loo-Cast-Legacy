#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RuntimeBinary {
    CoreEngine,
    Launcher,
}

impl RuntimeBinary {
    pub fn crate_name(self) -> &'static str {
        match self {
            RuntimeBinary::CoreEngine => "core_engine",
            RuntimeBinary::Launcher => "launcher",
        }
    }

    pub fn executable_stem(self) -> &'static str {
        self.crate_name()
    }
}
