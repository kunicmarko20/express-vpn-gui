use crate::expressvpn::*;
use arc_guard::ArcGuard;
use gtk::*;
use super::super::Indicator;
use crate::asset;

pub struct Connect;

impl Connect {
    pub fn create(indicator: ArcGuard<Indicator>) -> MenuItem {
        let menu_item = MenuItem::new_with_label("");

        menu_item.connect_activate(move |menu_item| {
            let result_by_label = Self::result_by_label(menu_item.get_label());
            let output = ExpressVPNCommand::execute(result_by_label.0);

            if output.status.success() {
                menu_item.set_label(result_by_label.2.as_str());

                let image_name = result_by_label.1.clone();
                indicator.execute(move |indicator| {
                    let mut indicator = indicator.lock().expect("Unable to lock indicator from Connect.");
                    indicator.change_icon(image_name);
                });
                return;
            }

            println!("{}", String::from_utf8(output.stderr).unwrap());
        });

        menu_item
    }

    fn result_by_label(label: Option<String>) -> (ExpressVPNSubCommand, &'static str, ConnectLabel) {
        match ConnectLabel::from_string(label.unwrap()) {
            ConnectLabel::CONNECT => (
                ExpressVPNSubCommand::CONNECT,
                asset::IMAGE_NAME_ON,
                ConnectLabel::DISCONNECT
            ),
            ConnectLabel::DISCONNECT => (
                ExpressVPNSubCommand::DISCONNECT,
                asset::IMAGE_NAME_OFF,
                ConnectLabel::CONNECT
            )
        }
    }
}

enum ConnectLabel {
    CONNECT,
    DISCONNECT,
}

impl ConnectLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectLabel::CONNECT => "Connect",
            ConnectLabel::DISCONNECT => "Disconnect",
        }
    }

    pub fn from_string(label: String) -> ConnectLabel {
        match label.as_ref() {
            "Connect" => ConnectLabel::CONNECT,
            "Disconnect" => ConnectLabel::DISCONNECT,
            _ => panic!("Unrecognized label."),
        }
    }
}
