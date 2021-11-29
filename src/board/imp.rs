use glib::clone;
use gtk::{
    glib::{self, subclass::Signal},
    EventControllerKey, Inhibit,
};
use {gtk::prelude::*, gtk::subclass::prelude::*};

use gtk4 as gtk;
use once_cell::sync::Lazy;

use crate::{
    cell,
    sudoku::{self, Sudoku},
};
use std::cell::RefCell;
// Object holding the state
#[derive(Default)]
pub struct Board {
    pub selected: RefCell<Option<cell::BoardCell>>,
    pub grid: RefCell<Option<Sudoku>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Board {
    const NAME: &'static str = "SudokuBoard";
    type Type = super::Board;
    type ParentType = gtk::Grid;
}

impl Board {
    pub fn set_grid(&self, grid: sudoku::Sudoku) {
        *self.grid.borrow_mut() = Some(grid);
    }
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

                let key_char = key.to_unicode().unwrap_or('0');

                if let Some(number) = key_char.to_digit(10) {

                    if let 1..=9 = number {
                        let selected = imp.selected.borrow();
                        if let Some(s) = selected.as_ref() {
                            let (row, column) = s.get_position();
                            if !s.is_changeable() {
                                println!("nope!");
                                return Inhibit(false);
                            }

                            s.set_number(number as i32);
                            obj.highlight_number(number as i32);

                            let mut sudoku = imp.grid.borrow_mut();
                            if let Some(g) = sudoku.as_mut() {
                                g.set_cell(number as i32, row, column);
                            }
                        }

                        let is_solved = imp.grid.borrow().as_ref().unwrap().is_solved();
                        if is_solved {
                            println!("EMITTING: board-solved");
                            obj.emit_by_name("board-solved", &[]).expect("Error emitting signal 'board-solved'");
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

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder(
                // Signal name
                "board-solved",
                // Types of the values which will be sent to the signal handler
                &[],
                // Type of the value the signal handler sends back
                <()>::static_type().into(),
            )
            .build()]
        });
        SIGNALS.as_ref()
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
