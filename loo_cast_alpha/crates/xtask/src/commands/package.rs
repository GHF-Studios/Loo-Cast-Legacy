use anyhow::Result;
use std::path::Path;
use xshell::Shell;

use crate::commands::build::build;
use crate::utils::build_target::BuildTarget;
use crate::utils::fs::{
    cargo_artifact_dir, clean_dir, copy_dir_recursive, copy_file, copy_optional_symbol, discover_workspace_crates, executable_name, staged_build_dir,
};
use crate::utils::profile::Profile;
use crate::utils::runtime_binary::RuntimeBinary;

pub fn package(sh: &Shell, root: &Path, profile: Profile, target: BuildTarget, binary: RuntimeBinary) -> Result<()> {
    build(sh, profile, target, binary)?;

    let build_dir = staged_build_dir(root, profile, target);
    clean_dir(&build_dir)?;

    let artifact_dir = cargo_artifact_dir(root, profile, target);
    let executable_name = executable_name(binary, target);
    let source_executable = artifact_dir.join(&executable_name);
    let staged_executable = build_dir.join(&executable_name);

    copy_file(&source_executable, &staged_executable)?;
    let executable_stem = binary.executable_stem();
    copy_optional_symbol(
        &artifact_dir.join(format!("{executable_stem}.pdb")),
        &build_dir.join(format!("{executable_stem}.pdb")),
    )?;
    copy_optional_symbol(
        &artifact_dir.join(format!("{executable_stem}.debug")),
        &build_dir.join(format!("{executable_stem}.debug")),
    )?;
    stage_assets(root, &build_dir, binary)?;

    println!("packaged executable: {}", staged_executable.display());
    println!("packaged assets root: {}", build_dir.join("assets").display());
    Ok(())
}

fn stage_assets(root: &Path, build_dir: &Path, binary: RuntimeBinary) -> Result<()> {
    let assets_root = build_dir.join("assets");
    stage_binary_assets(root, &assets_root, binary)?;

    if binary == RuntimeBinary::Launcher {
        return Ok(());
    }

    stage_core_mod_assets(root, &assets_root)?;
    Ok(())
}

fn stage_binary_assets(root: &Path, assets_root: &Path, binary: RuntimeBinary) -> Result<()> {
    let crate_name = binary.crate_name();
    let crate_assets = root.join("crates").join(crate_name).join("assets");
    if crate_assets.is_dir() {
        copy_dir_recursive(&crate_assets, &assets_root.join(crate_name))?;
    }
    Ok(())
}

fn stage_core_mod_assets(root: &Path, assets_root: &Path) -> Result<()> {
    for crate_dir in discover_workspace_crates(root)? {
        let crate_name = crate_dir.file_name().and_then(|name| name.to_str()).unwrap_or_default().to_string();
        if crate_name == RuntimeBinary::CoreEngine.crate_name() || crate_name == RuntimeBinary::Launcher.crate_name() {
            continue;
        }
        if !crate_dir.join(".loo_cast_mod").is_file() {
            continue;
        }
        let crate_assets = crate_dir.join("assets");
        if !crate_assets.is_dir() {
            continue;
        }
        copy_dir_recursive(&crate_assets, &assets_root.join(crate_name))?;
    }
    Ok(())
}
