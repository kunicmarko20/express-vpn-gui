use crate::expressvpn::*;
use std::sync::{Arc, Mutex};
use gtk::*;
use super::super::Indicator;

pub struct Connect;

impl Connect {
    pub fn create(indicator: Arc<Mutex<Indicator>>) -> MenuItem {
        let menu_item = MenuItem::new_with_label("");

        menu_item.connect_activate(move |menu_item| {
            let result_by_label = Self::result_by_label(menu_item.get_label());
            let output = ExpressVPNCommand::execute(result_by_label.0);

            if output.status.success() {
                menu_item.set_label(result_by_label.2);
                let indicator_clone = indicator.clone();
                let mut indicator_clone = indicator_clone.lock().unwrap();
                indicator_clone.change_icon(result_by_label.1);
                return;
            }

            println!("{}", String::from_utf8(output.stderr).unwrap());
        });

        menu_item
    }

    fn result_by_label(label: Option<String>) -> (ExpressVPNSubCommand, &'static str, &'static str) {
        match label.unwrap().as_ref() {
            "Connect" => (
                ExpressVPNSubCommand::CONNECT,
                "on.png",
                "Disconnect"
            ),
            "Disconnect" => (
                ExpressVPNSubCommand::DISCONNECT,
                "logo.png",
                "Connect"
            ),
            _ => panic!("Unrecognized label.")
        }
    }
}
