use anyhow::{Context, Result, bail};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::utils::git::git_user_name;

pub fn run_gource(root: &Path) -> Result<()> {
    let gource_exe = root.join("gource.exe");
    let gource_program = if gource_exe.is_file() { gource_exe } else { PathBuf::from("gource") };

    let year_seconds = env::var("YEAR_SECONDS").ok().and_then(|value| value.parse::<f64>().ok()).unwrap_or(30.0);
    let seconds_per_day = format!("{:.6}", year_seconds / 365.2425);
    let auto_skip_seconds = env::var("AUTO_SKIP_SECONDS").unwrap_or_else(|_| "1".to_string());
    let file_idle_time = env::var("FILE_IDLE_TIME").unwrap_or_else(|_| "0".to_string());
    let max_file_lag = env::var("MAX_FILE_LAG").unwrap_or_else(|_| "0.15".to_string());
    let follow_user = env::var("FOLLOW_USER")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| git_user_name(root));

    let mut command = Command::new(&gource_program);
    command
        .current_dir(root)
        .arg("--title")
        .arg("Loo-Cast History")
        .arg("--seconds-per-day")
        .arg(seconds_per_day)
        .arg("--auto-skip-seconds")
        .arg(auto_skip_seconds)
        .arg("--file-idle-time")
        .arg(file_idle_time)
        .arg("--max-file-lag")
        .arg(max_file_lag)
        .arg("--hide")
        .arg("mouse,progress,filenames,dirnames,usernames")
        .arg("--highlight-users")
        .arg("--highlight-dirs")
        .arg("--camera-mode")
        .arg("overview")
        .arg("--multi-sampling")
        .arg("--dont-stop");
    if let Some(follow_user) = follow_user {
        command.arg("--follow-user").arg(follow_user);
    }

    let status = command
        .status()
        .with_context(|| format!("failed to execute '{}'", gource_program.as_path().display()))?;
    if !status.success() {
        bail!("gource exited with status {status}");
    }
    Ok(())
}
