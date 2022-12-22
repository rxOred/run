use gtk::prelude::*;
use gtk::{gdk, glib};
use gtk::{Application, ApplicationWindow, Entry, EntryCompletion};
use std::fs;
use std::process::Command;

extern crate gtk;

const APP_ID: &str = "org.rxored.run";

fn create_list_model() -> gtk::ListStore {
    let col_types: [gtk::glib::Type; 1] = [gtk::glib::Type::STRING];
    let mut data: Vec<String> = Vec::new();
    let path = fs::read_dir("/usr/bin/").unwrap();
    for file in path {
        data.push(file.as_ref().unwrap().file_name().into_string().unwrap());
    }
    let path = fs::read_dir("/usr/local/bin/").unwrap();
    for file in path {
        data.push(file.as_ref().unwrap().file_name().into_string().unwrap());
    }
    let store = gtk::ListStore::new(&col_types);
    for d in data.iter() {
        let values: [(u32, &dyn ToValue); 1] = [(0, &d)];
        store.set(&store.append(), &values);
    }
    store
}

fn build_ui(app: &Application) {
    let store = create_list_model();

    let completion = EntryCompletion::new();
    completion.set_text_column(0);
    completion.set_minimum_key_length(1);
    completion.set_popup_completion(false);
    completion.set_inline_selection(true);
    completion.set_inline_completion(true);
    completion.set_model(Some(&store));

    let entry = Entry::new();
    entry.set_max_width_chars(50);
    entry.set_text("search in /usr/bin");
    entry.set_completion(Some(&completion));
    entry.connect_key_press_event(
        gtk::glib::clone!(@weak entry => @default-return Inhibit(false), move |e, key| {
            if key.keyval() == gdk::keys::constants::Return {
                let text = e.text().to_string();
				Command::new(text).spawn().unwrap();
				println!("c");
				std::process::exit(0);
            }
            Inhibit(false)
        }),
    );

    let window = ApplicationWindow::new(app);
    window.set_default_width(30);
    window.set_default_height(40);
    window.set_resizable(false);
    window.add(&entry);
    window.set_position(gtk::WindowPosition::CenterAlways);

    window.connect_key_press_event(
        gtk::glib::clone!(@weak entry => @default-return Inhibit(false), move |_, key| {
            if key.keyval() == gdk::keys::constants::Escape {
                gtk::main_quit();
            }
            Inhibit(false)
        }),
    );
    window.show_all();
}

fn main() {
    let app = Application::new(Some(APP_ID), Default::default());
    app.connect_activate(build_ui);
    app.run();
}
