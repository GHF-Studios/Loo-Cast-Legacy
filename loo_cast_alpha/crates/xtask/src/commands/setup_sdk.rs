use anyhow::{Context, Result, bail};
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::utils::git::find_git_dir;
use crate::utils::string::shell_str;

pub(crate) const PRE_COMMIT_HOOK_CONTENTS: &str = r#"
    #!/usr/bin/env sh
    set -e
    cargo fmt --manifest-path loo_cast_alpha/Cargo.toml --all
"#;
pub(crate) const PRE_PUSH_HOOK_CONTENTS: &str = r#"
    #!/usr/bin/env sh
    set -e
    cargo run --manifest-path loo_cast_alpha/Cargo.toml -p xtask -- audit
"#;

/// # Purpose:
/// Installs the SDK git hooks defined by this repository.
///
/// # Input assumptions:
/// - `root` points at a path inside this workspace.
/// - A `.git` directory can be found from `root`.
///
/// # Output guarantees:
/// - `.git/hooks/pre-commit` and `.git/hooks/pre-push` are recreated from the canonical hook templates.
/// - On Unix targets, both hooks are marked executable (`0o755`).
///
/// # Example:
/// Works even from root:
/// ```text
/// cargo run --manifest-path loo_cast_alpha/Cargo.toml -p xtask -- setup_sdk
/// ```
///
/// Or simpler, but only works from loo_cast_alpha:
/// ```text
/// cd loo_cast_alpha (only once ofc)
/// cargo xtask setup_sdk
/// ```
pub fn setup_sdk(root: &Path) -> Result<()> {
    SetupSdk { root }.run()
}

struct SetupSdk<'a> {
    root: &'a Path,
}

impl SetupSdk<'_> {
    fn run(&self) -> Result<()> {
        println!("Running SDK setup steps");
        println!();
        println!("[1/1] Install git hooks");
        println!();
        self.setup_git_hooks()?;
        println!("SDK setup complete.");
        Ok(())
    }

    fn setup_git_hooks(&self) -> Result<()> {
        let Some(git_dir) = find_git_dir(self.root) else {
            bail!("failed to find a .git directory from '{}'", self.root.display());
        };
        let hooks_dir = git_dir.join("hooks");
        fs::create_dir_all(&hooks_dir).with_context(|| format!("failed to create '{}'", hooks_dir.display()))?;

        self.setup_git_hook(&hooks_dir, "pre-commit", PRE_COMMIT_HOOK_CONTENTS)?;
        self.setup_git_hook(&hooks_dir, "pre-push", PRE_PUSH_HOOK_CONTENTS)?;
        Ok(())
    }

    fn setup_git_hook(&self, hooks_dir: &Path, hook_name: &str, raw_hook_contents: &str) -> Result<()> {
        let hook_path = hooks_dir.join(hook_name);
        if hook_path.exists() {
            fs::remove_file(&hook_path).with_context(|| format!("failed to remove '{}'", hook_path.display()))?;
        }

        let canonical_hook_contents = shell_str::Canonical::new(raw_hook_contents);
        fs::write(&hook_path, &*canonical_hook_contents).with_context(|| format!("failed to write '{}'", hook_path.display()))?;

        #[cfg(unix)]
        {
            let mut permissions = fs::metadata(&hook_path)
                .with_context(|| format!("failed to read permissions for '{}'", hook_path.display()))?
                .permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&hook_path, permissions).with_context(|| format!("failed to set execute permissions on '{}'", hook_path.display()))?;
        }

        let formatted_hook_contents = shell_str::Formatted::from(canonical_hook_contents);
        println!("installed {hook_name} hook: {}", hook_path.display());
        println!("hook command: {{\n{}\n}}", &*formatted_hook_contents);
        println!();
        Ok(())
    }
}
