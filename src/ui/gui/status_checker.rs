use std::sync::{Arc, Mutex};
use gtk::*;
use crate::expressvpn::*;
use super::indicator::Indicator;

pub struct StatusChecker {
    indicator: Arc<Mutex<Indicator>>,
    menu_item: Arc<Mutex<MenuItem>>,
}

unsafe impl Send for StatusChecker {}

impl StatusChecker {
    pub fn new(indicator: Arc<Mutex<Indicator>>, menu_item: Arc<Mutex<MenuItem>>) -> Self {
        StatusChecker {indicator, menu_item}
    }

    pub fn run(&mut self) {
        let output = ExpressVPNCommand::execute(ExpressVPNSubCommand::STATUS);

        let output_string = String::from_utf8(output.stdout)
            .expect("Output wasn't a valid utf8.");

        let indicator = self.indicator.clone();
        let mut indicator = indicator.lock().unwrap();

        let menu_item = self.menu_item.clone();
        let menu_item = menu_item.lock().unwrap();

        if output_string.contains("Connecting") || output_string.contains("Connected") {
            indicator.change_icon("on.png");
            menu_item.set_label("Disconnect");
        }

        if output_string.contains("Not connected") {
            indicator.change_icon("logo.png");
            menu_item.set_label("Connect");
        }
    }
}
