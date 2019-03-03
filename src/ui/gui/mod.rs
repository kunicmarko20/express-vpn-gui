use gtk::*;
use std::sync::{Arc, Mutex};
use self::status_checker::StatusChecker;
use self::indicator::Indicator;
use self::menu::connect::Connect as MenuItemConnect;

pub fn init() {
    gtk::init().unwrap();

    let menu = gtk::Menu::new();
    let indicator = Arc::new(Mutex::new(Indicator::new()));
    let menu_item_connect = Arc::new(Mutex::new(MenuItemConnect::create(indicator.clone())));

    let menu_item_connect_clone = menu_item_connect.clone();
    let menu_item_connect_clone = menu_item_connect_clone.lock().unwrap();
    menu.append(&*menu_item_connect_clone);
    drop(menu_item_connect_clone);
    
    menu.append(&gtk::SeparatorMenuItem::new());
    menu.append(&self::menu::quit::Quit::create());

    let indicator_clone = indicator.clone();
    let mut indicator_clone = indicator_clone.lock().unwrap();
    indicator_clone.append_menu(menu);
    drop(indicator_clone);

    let mut status_checker = StatusChecker::new(indicator.clone(), menu_item_connect.clone());

    std::thread::spawn(move || {
        loop {
            &status_checker.run();

            std::thread::sleep(
                std::time::Duration::from_secs(5)
            );
        }
    });
    gtk::main();
}

mod menu;
mod indicator;
mod status_checker;
