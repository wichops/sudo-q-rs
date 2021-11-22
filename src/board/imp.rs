use gtk::glib;

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
