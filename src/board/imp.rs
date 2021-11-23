use glib::clone;
use gtk::{glib, EventControllerKey, Inhibit};
use {gtk::prelude::*, gtk::subclass::prelude::*};

use gtk4 as gtk;

use crate::cell;
use std::cell::RefCell;
// Object holding the state
#[derive(Default)]
pub struct Board {
    pub selected: RefCell<Option<cell::BoardCell>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Board {
    const NAME: &'static str = "SudokuBoard";
    type Type = super::Board;
    type ParentType = gtk::Grid;
}

// Trait shared by all GObjects
impl ObjectImpl for Board {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        let child = cell::BoardCell::new();

        *self.selected.borrow_mut() = Some(child);

        let controller = EventControllerKey::new();

        controller.connect_key_pressed(
            clone!(@weak obj => @default-return Inhibit(false),  move |_, key, _, _| {
                let imp = Board::from_instance(&obj);
                let key_char = key.to_unicode().unwrap();

                if let Some(number) = key_char.to_digit(10) {

                    if let 1..=9 = number {
                        let selected = imp.selected.borrow();
                        if let Some(s) = &*selected {
                            s.set_number(number as i32);
                        }
                    } else {
                        println!("xd")
                    };
                };

                Inhibit(false)
            }),
        );

        obj.add_controller(&controller);
    }

    fn dispose(&self, _obj: &Self::Type) {
        if let Some(selected) = self.selected.borrow_mut().take() {
            selected.unparent();
        }
    }
}

// Trait shared by all widgets
impl WidgetImpl for Board {}

// Trait shared by all buttons
impl GridImpl for Board {}
