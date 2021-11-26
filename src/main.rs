use gdk::Display;
use gtk::{
    gdk, prelude::*, Application, ApplicationWindow, CssProvider, Label, Stack,
    StackTransitionType, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk4 as gtk;

mod board;
mod cell;
mod sudoku;

use board::Board;
use cell::BoardCell;
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

    let grid: Board = builder
        .object::<Board>("board")
        .expect("Could not get object `board` from builder.");

    let sudoku = Sudoku::new();
    grid.set_grid(sudoku.clone());

    for (row_index, row) in sudoku.board.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            let board_cell = BoardCell::with_number(*col);

            grid.attach_cell(&board_cell, row_index as i32, col_index as i32);
        }
    }

    let stack: Stack = builder
        .object::<Stack>("stack")
        .expect("Could not get `stack` from builder.");

    stack.set_transition_duration(300);
    stack.set_transition_type(StackTransitionType::SlideDown);

    grid.connect_local("board-solved", false, move |_| {
        stack.set_visible_child_name("win-screen");

        None
    });

    // let controls: gtk::FlowBox = builder
    //     .object("controls")
    //     .expect("Could not get object `controls` from builder.");

    // for n in 0..=9 {
    //     let label = Label::new(Some(&n.to_string()));
    //     label.add_css_class("control");
    //     controls.insert(&label, n);
    // }

    let window: ApplicationWindow = builder
        .object("window")
        .expect("Could not get object `window` from builder.");

    window.set_application(Some(app));
    window.set_default_size(600, 640);

    window.present();
}
