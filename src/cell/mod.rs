mod imp;

use glib::Object;
use gtk::{glib, subclass::prelude::*};

use gtk4 as gtk;

glib::wrapper! {
    pub struct BoardCell(ObjectSubclass<imp::BoardCell>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl BoardCell {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `BoardCell`.")
    }
    pub fn with_number(number: i32) -> Self {
        let object = Object::new(&[]).expect("Failed to create `BoardCell`.");

        let imp = imp::BoardCell::from_instance(&object);
        imp.set_number(number);

        println!("imp.number {:?}", imp.number.get());
        object
    }
}

impl Default for BoardCell {
    fn default() -> Self {
        Self::new()
    }
}
