use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Grid, Orientation, Box as Box_};

const BOARD_SIZE: usize = 9;
type Board = [[u16; BOARD_SIZE]; BOARD_SIZE];

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

    let mut board: Board = [[9; BOARD_SIZE]; BOARD_SIZE];

    let grid = Grid::builder()
        .vexpand(true)
        .margin_start(16)
        .margin_end(16)
        .margin_bottom(16)
        .column_homogeneous(true)
        .row_homogeneous(true)
        .build();

    for (row_index, row) in board.iter_mut().enumerate() {
        for (col_index, col) in row.iter_mut().enumerate() {
            *col = ((row_index + 1) * (col_index + 1)) as u16;
            grid.attach(&Label::new(Some(&col.to_string())), row_index as i32, col_index as i32, 1, 1);
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
