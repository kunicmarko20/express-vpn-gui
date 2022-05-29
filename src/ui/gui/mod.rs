use gtk::*;
use self::status_checker::StatusChecker;
use self::indicator::Indicator;
use self::menu::connect::Connect as MenuItemConnect;
use arc_guard::ArcGuard;

pub fn init() {
    gtk::init().unwrap();

    let menu = ArcGuard::new(gtk::Menu::new());
    let indicator = ArcGuard::new(Indicator::new());
    let menu_item_connect = ArcGuard::new(MenuItemConnect::create(indicator.clone()));

    menu_item_connect.execute(|menu_item_connect| {
        menu.execute(|menu| {
            let menu_item_connect = menu_item_connect.lock().expect("Unable to lock menu_item from init.");
            let menu = menu.lock().expect("Unable to lock menu from init.");
            menu.append(&*menu_item_connect);
            menu.append(&gtk::SeparatorMenuItem::new());
            menu.append(&self::menu::quit::Quit::create());
        });
    });

    indicator.execute(|indicator| {
        menu.execute(|menu| {
            let mut indicator = indicator.lock().expect("Unable to lock indicator from init.");
            indicator.append_menu(menu);
        });
    });

    let mut status_checker = StatusChecker::new(indicator.clone(), menu_item_connect.clone());

    std::thread::spawn(move || {
        loop {
            let _ = &status_checker.run();

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
