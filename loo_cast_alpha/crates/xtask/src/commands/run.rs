use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Command;
use xshell::Shell;

use crate::commands::package::package;
use crate::utils::build_target::BuildTarget;
use crate::utils::fs::{executable_name, staged_build_dir};
use crate::utils::profile::Profile;
use crate::utils::runtime_binary::RuntimeBinary;

pub fn run(sh: &Shell, root: &Path, profile: Profile, binary: RuntimeBinary) -> Result<()> {
    package(sh, root, profile, BuildTarget::Host, binary)?;
    let build_dir = staged_build_dir(root, profile, BuildTarget::Host);
    let executable = build_dir.join(executable_name(binary, BuildTarget::Host));

    let status = Command::new(&executable)
        .current_dir(&build_dir)
        .env("RUST_BACKTRACE", "1")
        .env("BUILD_PROFILE", profile.as_str())
        .status()
        .with_context(|| format!("failed to execute '{}'", executable.display()))?;
    if !status.success() {
        bail!("staged executable exited with status {status}");
    }
    Ok(())
}
