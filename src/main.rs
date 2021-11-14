use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as Box_, Grid, Label, Orientation};
use gtk4 as gtk;

mod sudoku;

use sudoku::Sudoku;

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Sudo-q xd")
        .build();

    let grid = Grid::builder()
        .vexpand(true)
        .margin_start(16)
        .margin_end(16)
        .margin_bottom(16)
        .column_homogeneous(true)
        .row_homogeneous(true)
        .build();

    let sudoku = Sudoku::new();

    for (row_index, row) in sudoku.board.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            grid.attach(
                &Label::new(Some(&col.to_string())),
                row_index as i32,
                col_index as i32,
                1,
                1,
            );
        }
    }

    let vbox = Box_::new(Orientation::Vertical, 16);
    vbox.set_margin_top(16);

    let title = Label::new(Some("Sudo-q xd"));

    title.set_valign(gtk::Align::Start);
    // Add button
    window.set_default_size(400, 420);
    vbox.append(&title);
    vbox.append(&grid);
    window.set_child(Some(&vbox));

    // Present window to the user
    window.present();
}
