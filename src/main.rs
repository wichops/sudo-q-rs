use gtk::gdk::Display;
use gtk::{prelude::*, CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk::{Application, ApplicationWindow, Box as Box_, Grid, Label, Orientation};
use gtk4 as gtk;

mod sudoku;

use sudoku::Sudoku;

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("huichops.sudo-q.rs")
        .build();

    app.connect_startup(|app| {
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

        // We build the application UI.
        build_ui(app);
    });
    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("SOY EL DIOS DEL CUCEI")
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
            let text = match col {
                1..=9 => col.to_string(),
                _ => " ".to_string(),
            };

            grid.attach(
                &Label::builder()
                    .label(&text)
                    .css_classes(vec![String::from("grid-item")])
                    .build(),
                col_index as i32,
                row_index as i32,
                1,
                1,
            );
        }
    }

    let vbox = Box_::new(Orientation::Vertical, 20);
    vbox.set_margin_top(16);

    let title = Label::builder()
        .label("Sudo-q xd")
        .valign(gtk::Align::Start)
        .css_classes(vec![String::from("title")])
        .build();

    window.set_default_size(400, 480);
    vbox.append(&title);
    vbox.append(&grid);

    window.set_child(Some(&vbox));

    // Present window to the user
    window.present();
}
