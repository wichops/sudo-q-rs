mod imp;

use glib::Object;
use gtk::glib;

use gtk4 as gtk;

glib::wrapper! {
    pub struct Board(ObjectSubclass<imp::Board>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Board {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `Board`.")
    }

    pub fn with_label(label: &str) -> Self {
        Object::new(&[("label", &label)]).expect("Failed to create `Board`.")
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
