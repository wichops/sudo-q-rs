use std::cell::Cell;

use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use gtk4 as gtk;
use once_cell::sync::Lazy;

#[derive(Default, CompositeTemplate)]
#[template(file = "cell.ui")]
pub struct BoardCell {
    #[template_child]
    pub label: TemplateChild<gtk::Label>,

    pub number: Cell<i32>,
    pub position: Cell<(i32, i32)>,
    pub selected: Cell<bool>,
    pub changeable: Cell<bool>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for BoardCell {
    const NAME: &'static str = "SudokuBoardCell";
    type Type = super::BoardCell;
    type ParentType = gtk::Button;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoardCell {
    pub fn set_number(&self, number: i32) {
        self.number.set(number);
        let text = if let 1..=9 = number {
            number.to_string()
        } else {
            " ".to_string()
        };

        self.label.set_label(&text);
    }

    pub fn set_changeable(&self, changeable: bool) {
        self.changeable.set(changeable);
    }
}
// Trait shared by all GObjects
impl ObjectImpl for BoardCell {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        self.label.set_label(&self.number.get().to_string());
    }
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder(
                // Signal name
                "cell-clicked",
                // Types of the values which will be sent to the signal handler
                &[i32::static_type().into(), i32::static_type().into()],
                // Type of the value the signal handler sends back
                <()>::static_type().into(),
            )
            .build()]
        });
        SIGNALS.as_ref()
    }
}

// Trait shared by all widgets
impl WidgetImpl for BoardCell {}

// Trait shared by all buttons
impl ButtonImpl for BoardCell {
    fn clicked(&self, button: &Self::Type) {
        let (row, column) = self.position.get();
        button
            .emit_by_name("cell-clicked", &[&row, &column])
            .expect("Error emiting 'cell-clicked'");
    }
}
