use arc_guard::Guard;
use gtk::*;
use crate::expressvpn::*;
use super::indicator::Indicator;
use crate::asset;

pub struct StatusChecker {
    indicator: Guard<Indicator>,
    menu_item: Guard<MenuItem>,
}

unsafe impl Send for StatusChecker {}

impl StatusChecker {
    pub fn new(indicator: Guard<Indicator>, menu_item: Guard<MenuItem>) -> Self {
        StatusChecker {indicator, menu_item}
    }

    pub fn run(&mut self) {
        let output = ExpressVPNCommand::execute(ExpressVPNSubCommand::STATUS);

        let output_string = String::from_utf8(output.stdout)
            .expect("Output wasn't a valid utf8.");

        if output_string.contains("Connecting") || output_string.contains("Connected") {
            self.change_icon(asset::IMAGE_NAME_ON);
            self.change_menu_label("Disconnect");
        }

        if output_string.contains("Not connected") {
            self.change_icon(asset::IMAGE_NAME_OFF);
            self.change_menu_label("Connect");
        }
    }

    fn change_icon(&self, icon: &str) {
        self.indicator.execute(move |indicator| {
            let mut indicator = indicator.lock().expect("Unable to lock indicator from StatusChecker.");
            indicator.change_icon(icon);
        });
    }

    fn change_menu_label(&self, label: &str) {
        self.menu_item.execute(move |menu_item| {
            let menu_item = menu_item.lock().expect("Unable to lock menu_item from StatusChecker.");
            menu_item.set_label(label);
        });
    }
}
