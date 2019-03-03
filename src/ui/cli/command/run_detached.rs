use structopt::StructOpt;
use super::Command;
use std::os::unix::process::CommandExt;
use libc;
use crate::asset;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct RunDetached {}

impl Command for RunDetached {
    fn execute(&self) {
        std::process::Command::new(asset::EXECUTABLE_NAME)
            .before_exec(Self::owned_pgid)
            .arg("run")
            .output()
            .expect("failed to execute process");
    }
}

impl RunDetached {
    // We want the command to spawn processes in their own process group
    // Otherwise if a child process sends SIGTERM to the group, the command could be terminated.
    fn owned_pgid() -> std::result::Result<(), std::io::Error> {
        unsafe {
            libc::setpgid(0, 0);
        }
        Ok(())
    }
}
