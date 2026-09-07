use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::build_target::BuildTarget;
use crate::utils::profile::Profile;
use crate::utils::runtime_binary::RuntimeBinary;

pub fn staged_build_dir(root: &Path, profile: Profile, target: BuildTarget) -> PathBuf {
    let mut path = root.join("build").join(profile.as_str());
    if let Some(triple) = target.triple() {
        path.push(triple);
    }
    path
}

pub fn cargo_artifact_dir(root: &Path, profile: Profile, target: BuildTarget) -> PathBuf {
    let mut path = root.join("target");
    if let Some(triple) = target.triple() {
        path.push(triple);
    }
    path.join(profile.artifact_dir_name())
}

pub fn executable_name(binary: RuntimeBinary, target: BuildTarget) -> String {
    let stem = binary.executable_stem();
    if target.is_windows() { format!("{stem}.exe") } else { stem.to_string() }
}

pub fn clean_dir(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).with_context(|| format!("failed to remove '{}'", path.display()))?;
    }
    fs::create_dir_all(path).with_context(|| format!("failed to create '{}'", path.display()))?;
    Ok(())
}

pub fn copy_file(source: &Path, destination: &Path) -> Result<()> {
    if !source.is_file() {
        bail!("missing file '{}'", source.display());
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).with_context(|| format!("failed to create '{}'", parent.display()))?;
    }
    fs::copy(source, destination).with_context(|| format!("failed to copy '{}' -> '{}'", source.display(), destination.display()))?;
    Ok(())
}

pub fn copy_optional_symbol(source: &Path, destination: &Path) -> Result<()> {
    if source.is_file() {
        copy_file(source, destination)?;
    }
    Ok(())
}

pub fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination).with_context(|| format!("failed to create '{}'", destination.display()))?;

    for entry in fs::read_dir(source).with_context(|| format!("failed to read '{}'", source.display()))? {
        let entry = entry.with_context(|| format!("failed to read entry in '{}'", source.display()))?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to get file type for '{}'", source_path.display()))?;

        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            copy_file(&source_path, &destination_path)?;
        }
    }
    Ok(())
}

pub fn discover_workspace_crates(root: &Path) -> Result<Vec<PathBuf>> {
    let crates_root = root.join("crates");
    let mut crates = Vec::new();
    for entry in fs::read_dir(&crates_root).with_context(|| format!("failed to read '{}'", crates_root.display()))? {
        let entry = entry.with_context(|| format!("failed to read directory entry in '{}'", crates_root.display()))?;
        let path = entry.path();
        if path.is_dir() && path.join("Cargo.toml").is_file() {
            crates.push(path);
        }
    }
    crates.sort();
    Ok(crates)
}
