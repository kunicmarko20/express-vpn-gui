use gtk::*;

pub struct Quit;

impl Quit {
    pub fn create() -> gtk::MenuItem {
        let menu_item = gtk::MenuItem::with_label("Quit");

        menu_item.connect_activate(|_| {
            gtk::main_quit();
        });

        menu_item
    }
}
