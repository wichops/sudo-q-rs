mod imp;

use glib::Object;
use gtk::{
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};

use cell::BoardCell;

use crate::{cell, sudoku};
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

    pub fn set_grid(&self, grid: sudoku::Sudoku) {
        let imp = imp::Board::from_instance(self);

        imp.set_grid(grid);
    }

    pub fn highlight_number(&self, number: i32) {
        let imp = imp::Board::from_instance(self);
        let mut positions: Vec<(i32, i32)> = vec![];

        let sudoku = imp.grid.borrow_mut();

        if let Some(s) = sudoku.as_ref() {
            positions = s.get_number_positions(number);
        }

        for i in 0..9 {
            for j in 0..9 {
                let child = self.child_at(j, i).unwrap();
                if positions.contains(&(i as i32, j as i32)) {
                    child.add_css_class("grid-item--highlighted");
                } else {
                    child.remove_css_class("grid-item--highlighted");
                }
            }
        }
    }

    pub fn attach_cell(&self, cell: &BoardCell, row: i32, column: i32) {
        cell.set_position((row, column));

        cell.connect_local(
            "cell-clicked",
            false,
            clone!(@weak self as board => @default-return None, move |args| {
                let imp = imp::Board::from_instance(&board);
                let clicked_cell = args[0].get::<BoardCell>().expect("Could not get BoardCell");

                if let Some(s) = imp.selected.borrow().as_ref() { s.toggle_selected() };

                board.highlight_number(clicked_cell.get_number());
                clicked_cell.toggle_selected();
                *imp.selected.borrow_mut() = Some(clicked_cell);

                let row = args[1].get::<i32>().expect("Could not get row");
                let column = args[2].get::<i32>().expect("Could not get column");

                println!("clicked: ({}, {})", row, column);
                None
            }),
        )
        .unwrap();

        self.attach(cell, column, row, 1, 1)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
