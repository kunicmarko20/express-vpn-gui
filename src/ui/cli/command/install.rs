use super::Command;
use std::path::PathBuf;
use std::fs::OpenOptions;
use structopt::StructOpt;
use std::io::Write;
use crate::asset;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Install {}

const IMAGE_LOGO: &'static [u8] = include_bytes!("../../../../assets/logo.png");
const IMAGE_ON: &'static [u8] = include_bytes!("../../../../assets/on.png");
const DESKTOP_ENTRY: &'static [u8] = include_bytes!("../../../../assets/express-vpn-gui.desktop");

impl Command for Install {
    fn execute(&self) {
        let local_data_path = dirs::data_local_dir()
            .expect("Failed to fetch local data directory.");

        Self::create_data_directory(local_data_path.clone());
        Self::create_images(local_data_path.clone());
        Self::create_desktop_entry( local_data_path.clone());
    }
}

impl Install {
    fn create_data_directory(mut local_data_path: PathBuf) {
        local_data_path.push(asset::DATA_DIRECTORY);
        std::fs::create_dir_all(local_data_path).unwrap();
    }

    fn create_images(local_data_path: PathBuf) {
        Self::create_image(local_data_path.clone(), asset::PATH_IMAGE_LOGO, IMAGE_LOGO);
        Self::create_image(local_data_path.clone(), asset::PATH_IMAGE_ON, IMAGE_ON);
        Self::create_image(local_data_path.clone(), asset::PATH_IMAGE_OFF, IMAGE_LOGO);
    }

    fn create_image(mut local_data_path: PathBuf, image_name: &str, image: &[u8]) {
        local_data_path.push(image_name);
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(local_data_path) {
            file.write(image).unwrap();
        }
    }

    fn create_desktop_entry(mut local_data_path: PathBuf) {
        let mut logo_path = local_data_path.clone();
        logo_path.push(asset::PATH_IMAGE_LOGO);
        local_data_path.push(asset::PATH_DESKTOP_ENTRY);

        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(local_data_path) {
            file.write(DESKTOP_ENTRY).unwrap();
            file.write_all(
                format!(
                    r#"
                    Icon={}
                    Exec={} run
                    "#,
                    logo_path.to_str().unwrap(),
                    std::env::current_exe().unwrap().to_str().unwrap()
                ).as_bytes()
            ).unwrap();
        }
    }
}