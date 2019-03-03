use std::process::{Command, Output};

pub struct ExpressVPNCommand;

impl ExpressVPNCommand {
    const COMMAND_NAME: &'static str = "expressvpn";

    pub fn execute(subcommand: ExpressVPNSubCommand) -> Output {
        Command::new(Self::COMMAND_NAME)
            .arg(subcommand.as_str())
            .output()
            .expect("failed to execute process")
    }
}

pub enum ExpressVPNSubCommand {
    CONNECT,
    DISCONNECT,
    STATUS,
}

impl ExpressVPNSubCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExpressVPNSubCommand::CONNECT => "connect",
            ExpressVPNSubCommand::DISCONNECT => "disconnect",
            ExpressVPNSubCommand::STATUS => "status",
        }
    }
}
