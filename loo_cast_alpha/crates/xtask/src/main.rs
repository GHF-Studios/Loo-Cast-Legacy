//! Workspace task runner for Loo-Cast alpha.
//!
//! `xtask` is the canonical command surface for setup, build, package, run, audit,
//! and support utilities used by the current execution rails.

mod commands;
mod utils;

use anyhow::{Context, Result, bail};
use std::env;
use std::path::PathBuf;
use xshell::Shell;

use crate::commands::audit::audit;
use crate::commands::build::build;
use crate::commands::clean_sdk::clean_sdk;
use crate::commands::cloc::run_cloc;
use crate::commands::deploy::deploy_stub;
use crate::commands::docs::build_docs;
use crate::commands::gource::run_gource;
use crate::commands::help::print_help;
use crate::commands::package::package;
use crate::commands::run::run;
use crate::commands::setup_sdk::setup_sdk;
use crate::utils::build_target::BuildTarget;
use crate::utils::profile::Profile;
use crate::utils::runtime_binary::RuntimeBinary;

const XTASK_CRATE: &str = "xtask";
const LINUX_RELEASE_TARGET: &str = "x86_64-unknown-linux-gnu";
const WINDOWS_RELEASE_TARGET: &str = "x86_64-pc-windows-msvc";

fn workspace_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .map(PathBuf::from)
        .with_context(|| format!("failed to resolve workspace root from CARGO_MANIFEST_DIR='{}'", manifest_dir.display()))
}

fn parse_task_and_flags() -> Result<Option<(String, bool)>> {
    let mut task: Option<String> = None;
    let mut launcher = false;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--launcher" => launcher = true,
            _ if arg.starts_with("--") => bail!("unknown xtask argument '{arg}'. Use `cargo xtask help`."),
            _ => {
                if task.replace(arg).is_some() {
                    bail!("xtask takes exactly one task argument");
                }
            }
        }
    }

    Ok(task.map(|task| (task, launcher)))
}

fn main() -> Result<()> {
    let sh = Shell::new().context("failed to create shell")?;
    let root = workspace_root()?;
    sh.change_dir(&root);

    let Some((task, launcher_mode)) = parse_task_and_flags()? else {
        print_help();
        return Ok(());
    };

    if launcher_mode {
        match task.as_str() {
            "build" => build(&sh, Profile::Fastdev, BuildTarget::Host, RuntimeBinary::Launcher)?,
            "package" => package(&sh, &root, Profile::Fastdev, BuildTarget::Host, RuntimeBinary::Launcher)?,
            "run" => run(&sh, &root, Profile::Fastdev, RuntimeBinary::Launcher)?,
            _ => bail!("'--launcher' is only supported for 'build', 'package', and 'run' default tasks"),
        }
        return Ok(());
    }

    match task.as_str() {
        "help" => print_help(),
        "build" => build(&sh, Profile::Fastdev, BuildTarget::Host, RuntimeBinary::CoreEngine)?,
        "build_dev" => build(&sh, Profile::Dev, BuildTarget::Host, RuntimeBinary::CoreEngine)?,
        "build_fastdev" => build(&sh, Profile::Fastdev, BuildTarget::Host, RuntimeBinary::CoreEngine)?,
        "build_release" => build(&sh, Profile::Release, BuildTarget::Host, RuntimeBinary::CoreEngine)?,
        "build_linux_release" => build(&sh, Profile::Release, BuildTarget::LinuxRelease, RuntimeBinary::CoreEngine)?,
        "build_windows_release" => build(&sh, Profile::Release, BuildTarget::WindowsRelease, RuntimeBinary::CoreEngine)?,
        "package" => package(&sh, &root, Profile::Fastdev, BuildTarget::Host, RuntimeBinary::CoreEngine)?,
        "package_dev" => package(&sh, &root, Profile::Dev, BuildTarget::Host, RuntimeBinary::CoreEngine)?,
        "package_fastdev" => package(&sh, &root, Profile::Fastdev, BuildTarget::Host, RuntimeBinary::CoreEngine)?,
        "package_release" => package(&sh, &root, Profile::Release, BuildTarget::Host, RuntimeBinary::CoreEngine)?,
        "package_linux_release" => package(&sh, &root, Profile::Release, BuildTarget::LinuxRelease, RuntimeBinary::CoreEngine)?,
        "package_windows_release" => package(&sh, &root, Profile::Release, BuildTarget::WindowsRelease, RuntimeBinary::CoreEngine)?,
        "run" => run(&sh, &root, Profile::Fastdev, RuntimeBinary::CoreEngine)?,
        "run_dev" => run(&sh, &root, Profile::Dev, RuntimeBinary::CoreEngine)?,
        "run_fastdev" => run(&sh, &root, Profile::Fastdev, RuntimeBinary::CoreEngine)?,
        "run_release" => run(&sh, &root, Profile::Release, RuntimeBinary::CoreEngine)?,
        "audit" => audit(&root)?,
        "build_docs" => build_docs(&sh, false)?,
        "open_docs" => build_docs(&sh, true)?,
        "setup_sdk" => setup_sdk(&root)?,
        "clean_sdk" => clean_sdk(&root)?,
        "cloc" => run_cloc(&root)?,
        "gource" => run_gource(&root)?,
        "deploy" => deploy_stub(),
        other => bail!("unknown xtask command '{other}'. Use `cargo xtask help`."),
    }

    Ok(())
}
