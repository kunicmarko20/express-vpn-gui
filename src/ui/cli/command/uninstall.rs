use structopt::StructOpt;
use super::Command;
use crate::asset;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Uninstall {}

impl Command for Uninstall {
    fn execute(&self) {
        let mut local_data_path = dirs::data_local_dir()
            .expect("Failed to fetch local data directory.");
        let mut desktop_entry_path = local_data_path.clone();

        local_data_path.push(asset::DATA_DIRECTORY);

        if local_data_path.as_path().exists() {
            std::fs::remove_dir_all(local_data_path)
                .expect("Failed to remove local data directory.");
        }

        desktop_entry_path.push(asset::PATH_DESKTOP_ENTRY);

        if desktop_entry_path.as_path().exists() {
            std::fs::remove_file(desktop_entry_path)
                .expect("Failed to remove desktop entry.");
        }
    }
}
