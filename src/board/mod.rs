mod imp;

use glib::Object;
use gtk::{
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};
use std::borrow::BorrowMut;

use cell::BoardCell;

use crate::cell;
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

    pub fn attach_cell(&self, cell: &BoardCell, row: i32, column: i32) {
        cell.set_position((row, column));
        cell.connect_local(
            "cell-clicked",
            false,
            clone!(@weak self as board => @default-return None, move |args| {
                let imp = imp::Board::from_instance(&board);
                let clicked_cell = args[0].get::<BoardCell>().expect("Could not get BoardCell");
                match &*imp.selected.borrow() {
                    Some(s) => s.toggle_selected(),
                    None => {}
                };

                clicked_cell.toggle_selected();
                *imp.selected.borrow_mut() = Some(clicked_cell);


                let row = args[1].get::<i32>().expect("Could not get row");
                let column = args[2].get::<i32>().expect("Could not get column");

                println!("clicked: ({}, {})", row, column);
                None
            }),
        );
        self.attach(cell, column, row, 1, 1)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
