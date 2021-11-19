use gdk::Display;
use glib::clone;
use gtk::{gdk, glib, Application, ApplicationWindow, Box as Box_, Grid, Label, Orientation};
use gtk::{
    prelude::*, Button, CssProvider, FlowBox, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk4 as gtk;

mod sudoku;

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
    app.run();
}

fn build_ui(app: &Application) {
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
            let (text, class) = match col {
                1..=9 => (col.to_string(), "not-selected"),
                _ => (" ".to_string(), "empty"),
            };

            let label = Label::new(Some(&text));
            label.add_css_class(class);

            let button = Button::new();
            button.add_css_class("grid-item");

            button.set_child(Some(&label));

            button.connect_clicked(clone!(@weak grid => move |b: &Button| {

                let (column, row, _, _) = grid.query_child(b);
                println!("{}, {}", row, column);

                let label = b.child().unwrap();
                label.add_css_class("selected");
            }));

            grid.attach(&button, col_index as i32, row_index as i32, 1, 1);
        }
    }

    let vbox = Box_::new(Orientation::Vertical, 16);
    vbox.set_margin_top(16);

    let title = Label::builder()
        .label("Sudo-q xd")
        .valign(gtk::Align::Start)
        .css_classes(vec![String::from("title")])
        .build();

    let controls = FlowBox::new();

    controls.set_column_spacing(8);
    controls.set_margin_start(32);
    controls.set_margin_end(32);
    controls.set_margin_bottom(16);

    for n in 0..=9 {
        let label = Label::new(Some(&n.to_string()));
        label.add_css_class("control");
        controls.insert(&label, n);
    }

    window.set_default_size(400, 620);
    vbox.append(&title);
    vbox.append(&grid);
    vbox.append(&controls);

    window.set_child(Some(&vbox));

    // Present window to the user
    window.present();
}
