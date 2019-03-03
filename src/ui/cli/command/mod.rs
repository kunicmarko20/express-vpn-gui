pub mod install;
pub mod run;
pub mod run_detached;
pub mod uninstall;

pub trait Command {
    fn execute(&self);
}
