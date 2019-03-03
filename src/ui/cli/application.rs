use structopt::StructOpt;
use super::command;

pub struct Application;

impl Application {
    pub fn run() {
        match ExpressVPNGui::from_args().command {
            ExpressVPNGuiSubCommand::Install(command) => Self::execute(command),
            ExpressVPNGuiSubCommand::Run(command) => {
                if !command.detached {
                    Self::execute(command);
                    return;
                }

                Self::execute(command::run_detached::RunDetached{})
            },
            ExpressVPNGuiSubCommand::Uninstall(command) => Self::execute(command),
        }
    }

    fn execute<T: command::Command>(command: T) {
        command.execute()
    }
}

#[derive(Debug, StructOpt)]
struct ExpressVPNGui {
    #[structopt(subcommand)]
    pub command: ExpressVPNGuiSubCommand
}

#[derive(Debug, StructOpt)]
enum ExpressVPNGuiSubCommand {
    #[structopt(name = "install")]
    /// Installs needed assets.
    Install(command::install::Install),
    #[structopt(name = "run")]
    /// Starts the Application
    Run(command::run::Run),
    #[structopt(name = "uninstall")]
    /// Removes everything
    Uninstall(command::uninstall::Uninstall),
}