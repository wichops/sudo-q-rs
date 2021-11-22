use gtk::glib;
use {gtk::prelude::*, gtk::subclass::prelude::*};

use gtk4 as gtk;
// Object holding the state
#[derive(Default)]
pub struct Board;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Board {
    const NAME: &'static str = "SudokuBoard";
    type Type = super::Board;
    type ParentType = gtk::Grid;
}

// Trait shared by all GObjects
impl ObjectImpl for Board {}

// Trait shared by all widgets
impl WidgetImpl for Board {}

// Trait shared by all buttons
impl GridImpl for Board {}
