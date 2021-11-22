use std::cell::Cell;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use gtk4 as gtk;

#[derive(Default, CompositeTemplate)]
#[template(file = "cell.ui")]
pub struct BoardCell {
    #[template_child]
    pub label: TemplateChild<gtk::Label>,

    pub number: Cell<i32>,
    pub position: Cell<(i32, i32)>,
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
}
// Trait shared by all GObjects
impl ObjectImpl for BoardCell {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        self.label.set_label(&self.number.get().to_string());
    }
}

// Trait shared by all widgets
impl WidgetImpl for BoardCell {}

// Trait shared by all buttons
impl ButtonImpl for BoardCell {
    fn clicked(&self, _button: &Self::Type) {
        println!("{:?}", self.position.get());
    }
}
