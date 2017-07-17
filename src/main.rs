extern crate gtk;
#[macro_use]
extern crate lazy_static;
extern crate pcap_file;
extern crate gdk;
extern crate rand;
extern crate cast;
extern crate ini;

use gtk::prelude::*;
use gtk::{Builder, Button, FileChooserDialog, MenuItem, TextView, Window};

use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use std::sync::RwLock;
use pcap_file::Packet;

mod ui;
mod utils;

use std::path::PathBuf;


lazy_static! {
    static ref PCAP: RwLock<Vec<Packet<'static>>> = RwLock::new(Vec::new());

}


fn main() {

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    ui::main_window::init();
    gtk::main();
}

