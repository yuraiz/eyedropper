use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    glib,
    prelude::{ObjectExt, ToValue},
};

mod imp {
    use std::cell::RefCell;

    use super::*;

    use glib::{subclass::Signal, ParamSpecString};
    use gtk::glib::ParamSpec;
    use once_cell::sync::Lazy;

    // Object holding the state
    #[derive(gtk::CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/hex-entry.ui")]
    pub struct HexEntry {
        #[template_child]
        pub entry: TemplateChild<gtk::Entry>,
        pub color: RefCell<String>,
    }

    #[gtk::template_callbacks]
    impl HexEntry {
        #[template_callback]
        fn on_copy_pressed(&self, _button: &gtk::Button) {
            self.instance().copy_color();
        }
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for HexEntry {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "HexEntry";
        type ParentType = gtk::Box;
        type Type = super::HexEntry;

        fn new() -> Self {
            Self {
                entry: TemplateChild::default(),
                color: RefCell::new(String::new()),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            // Bind the private callbacks
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for HexEntry {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder(
                        // Signal name
                        "copied-color",
                        // Types of the values which will be sent to the signal handler
                        &[String::static_type().into()],
                        // Type of the value the signal handler sends back
                        <()>::static_type().into(),
                    )
                    .build(),
                    Signal::builder(
                        // Signal name
                        "color-changed",
                        // Types of the values which will be sent to the signal handler
                        &[String::static_type().into()],
                        // Type of the value the signal handler sends back
                        <()>::static_type().into(),
                    )
                    .build(),
                ]
            });
            SIGNALS.as_ref()
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> =
                Lazy::new(|| vec![ParamSpecString::builder("color").build()]);
            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &ParamSpec,
        ) {
            match pspec.name() {
                "color" => {
                    let input_value = value.get::<String>().unwrap();
                    self.color.replace(input_value);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "color" => self.color.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.set_direction(gtk::TextDirection::Ltr);
            obj.setup_signals();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for HexEntry {}

    // Trait shared by all boxes
    impl BoxImpl for HexEntry {}
}

glib::wrapper! {
    pub struct HexEntry(ObjectSubclass<imp::HexEntry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl HexEntry {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new::<Self>(&[]).expect("Failed to create a ColorModelEntry")
    }

    pub fn color(&self) -> String {
        self.property("color")
    }

    pub fn set_color(&self, new_color: String) {
        self.set_property("color", &new_color);
    }

    fn setup_signals(&self) {
        self.bind_property("color", &*self.imp().entry, "text")
            .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            .build();

        self.imp()
            .entry
            .connect_changed(glib::clone!(@weak self as hex_entry => move |entry| {
                let text = entry.buffer().text();
                hex_entry.emit_by_name("color-changed", &[&text.to_value()])
            }));
    }

    fn copy_color(&self) {
        log::debug!("Coping selected color");
        let clipboard = self.clipboard();
        let color = self.imp().entry.text().to_string();
        clipboard.set_text(&color);
        self.emit_by_name("copied-color", &[&color.to_value()])
    }
}
