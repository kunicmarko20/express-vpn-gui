use structopt::StructOpt;
use super::Command;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Run {
    #[structopt(short = "d", long = "detached")]
    /// Start application in its own process group
    pub detached: bool,
}

impl Command for Run {
    fn execute(&self) {
        crate::ui::gui::init();
    }
}
