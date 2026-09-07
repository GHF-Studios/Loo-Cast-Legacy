use anyhow::{Result, bail};
use xshell::{Shell, cmd};

use crate::utils::build_target::BuildTarget;
use crate::utils::profile::Profile;
use crate::utils::runtime_binary::RuntimeBinary;
use crate::{LINUX_RELEASE_TARGET, WINDOWS_RELEASE_TARGET, XTASK_CRATE};

pub fn build(sh: &Shell, profile: Profile, target: BuildTarget, binary: RuntimeBinary) -> Result<()> {
    match binary {
        RuntimeBinary::CoreEngine => match (profile, target) {
            (Profile::Dev, BuildTarget::Host) => cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE}").run()?,
            (Profile::Fastdev, BuildTarget::Host) => cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE} --profile fastdev").run()?,
            (Profile::Release, BuildTarget::Host) => cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE} --release").run()?,
            (Profile::Release, BuildTarget::LinuxRelease) => {
                cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE} --release --target {LINUX_RELEASE_TARGET}").run()?
            }
            (Profile::Release, BuildTarget::WindowsRelease) => cmd!(
                sh,
                "cargo build --workspace --exclude {XTASK_CRATE} --release --target {WINDOWS_RELEASE_TARGET}"
            )
            .run()?,
            _ => bail!("only release profile supports explicit target tasks"),
        },
        RuntimeBinary::Launcher => {
            let launcher_crate = binary.crate_name();
            match (profile, target) {
                (Profile::Dev, BuildTarget::Host) => cmd!(sh, "cargo build -p {launcher_crate}").run()?,
                (Profile::Fastdev, BuildTarget::Host) => cmd!(sh, "cargo build -p {launcher_crate} --profile fastdev").run()?,
                (Profile::Release, BuildTarget::Host) => cmd!(sh, "cargo build -p {launcher_crate} --release").run()?,
                (Profile::Release, BuildTarget::LinuxRelease) => cmd!(sh, "cargo build -p {launcher_crate} --release --target {LINUX_RELEASE_TARGET}").run()?,
                (Profile::Release, BuildTarget::WindowsRelease) => {
                    cmd!(sh, "cargo build -p {launcher_crate} --release --target {WINDOWS_RELEASE_TARGET}").run()?
                }
                _ => bail!("only release profile supports explicit target tasks"),
            }
        }
    }
    Ok(())
}
