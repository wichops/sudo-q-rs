use gdk::Display;
use glib::clone;
use gtk::{
    gdk, glib, prelude::*, Application, ApplicationWindow, Button, CssProvider, Label,
    StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk4 as gtk;
use std::{cell::Cell, rc::Rc};

mod board;
mod sudoku;

use board::Board;
use sudoku::Sudoku;

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("huichops.sudo-q.rs")
        .build();

    app.connect_startup(|_| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));
        // We give the CssProvider to the default screen so the CSS rules we added
        // can be applied to our window.
        StyleContext::add_provider_for_display(
            &Display::default().expect("Error initializing gtk css provider."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });
    app.connect_activate(build_ui);
    Board::static_type();

    app.run();
}

fn build_ui(app: &Application) {
    let builder = gtk::Builder::from_string(include_str!("window.ui"));

    let window: ApplicationWindow = builder
        .object("window")
        .expect("Could not get object `window` from builder.");
    let grid: Board = builder
        .object("board")
        .expect("Could not get object `board` from builder.");

    let sudoku = Sudoku::new();
    let selected = Rc::new(Cell::new((-1, -1)));

    for (row_index, row) in sudoku.board.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            let text = if let 1..=9 = col {
                col.to_string()
            } else {
                " ".to_string()
            };

            let label = Label::new(Some(&text));
            label.add_css_class("grid-item__content");

            let button = Button::new();
            button.add_css_class("grid-item");

            button.set_child(Some(&label));

            button.connect_clicked(clone!(@strong selected, @weak grid => move |b: &Button| {
                let (column, row, _, _) = grid.query_child(b);
                b.add_css_class("grid-item--selected");

                println!("{}, {}", row, column);


                let (r, c) = selected.get();
                if let Some(child) = grid.child_at(c, r) {
                    child.remove_css_class("grid-item--selected");
                }
                selected.set((row, column));
            }));

            grid.attach(&button, col_index as i32, row_index as i32, 1, 1);
        }
    }

    let controls: gtk::FlowBox = builder
        .object("controls")
        .expect("Could not get object `controls` from builder.");

    for n in 0..=9 {
        let label = Label::new(Some(&n.to_string()));
        label.add_css_class("control");
        controls.insert(&label, n);
    }

    window.set_application(Some(app));
    window.set_default_size(400, 620);

    let container: gtk::Box = builder
        .object("container")
        .expect("Could not get object `container` from builder.");

    window.set_child(Some(&container));

    // // Present window to the user
    window.present();
}
