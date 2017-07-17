use gtk;
use gtk::prelude::*;
use gtk::{Builder, Button, Dialog, FileChooserDialog, FileFilter, Label, MenuItem, TextBuffer, TextTag, TextView, Window};

use pcap_file::{Packet, PcapReader};

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

use std::rc::Rc;
use std::sync::Mutex;
use std::cell::RefCell;

use utils;
use ui::find_window;

use ::PCAP;
use ini::Ini;

/*
struct INI_FILE{
    ini_path: PathBuf,
}

impl INI_FILE{
    fn new() -> INI_FILE{
        INI_FILE{
            ini_path: PathBuf::from("/"),
        }
    }
}

static mut ini_file: INI_FILE = INI_FILE::new();
*/

lazy_static!{
    static ref INI_FILE: Mutex<PathBuf> = Mutex::new(PathBuf::new());
}

pub fn init() {
     PathBuf::new();

    let glade = include_str!("../test.glade");
    let builder = Builder::new_from_string(glade);

    let window: Window = builder.get_object("window_main").expect(&format!("Error line : {}", line!()));
    let menu_open: MenuItem = builder.get_object("open_file").unwrap();
    let text_view: TextView = builder.get_object("text1").unwrap();
    let menu_find: MenuItem = builder.get_object("menu_item_find").unwrap();
    //let menu_test_color: MenuItem = builder.get_object("test_color").unwrap();
    let carret_pos_label: Label = builder.get_object("carret_pos_label").unwrap();


    let mut conf = Ini::new();
    conf.with_section(Some("Encodage".to_owned())).set("encoding","utf-8");


    window.show_all();


    let text_view_shared = Rc::new(text_view);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let text_view_shared_copy = text_view_shared.clone();
    menu_open.connect_activate(move |_| {

        if let Some(pcap_str) = load_file_to_hex_str(&window, &mut conf.clone()) {
            text_view_shared_copy.get_buffer().unwrap().set_text(&pcap_str);


        }
    });

    let text_view_shared_copy = text_view_shared.clone();
    menu_find.connect_activate(move |_| {
        find_window::init(text_view_shared_copy.clone());

    });

    /*
    let text_view_shared_copy = text_view_shared.clone();
    text_view.connect_move_cursor(move |_|{
            carret_pos_label.set_text("plop");

    });
    */

}



fn open_file_dialog(window: &Window, ) -> Option<PathBuf> {

    //Create FileChooserDialog
    let file_chooser = gtk::FileChooserDialog::new(Some("Ouvrir..."), Some(window), gtk::FileChooserAction::Open);
    file_chooser.add_buttons(&[("Ouvrir", gtk::ResponseType::Ok.into()), ("Annuler", gtk::ResponseType::Cancel.into())]);

    let filter_pcap = FileFilter::new();
    filter_pcap.set_name("Pcap (*.pcap)");
    filter_pcap.add_pattern("*.pcap");

    let filter_all = FileFilter::new();
    filter_all.set_name("Tous les fichiers (*)");
    filter_all.add_pattern("*");

    file_chooser.add_filter(&filter_pcap);
    file_chooser.add_filter(&filter_all);

    if file_chooser.run() == gtk::ResponseType::Ok.into() {
        let filename = file_chooser.get_filename().unwrap();
        file_chooser.destroy();
        return Some(filename);
    }
        else {
            file_chooser.destroy();
            return None;
        }
}


fn load_file_to_hex_str(window: &Window,conf: &mut Ini) -> Option<String> {

    if let Some(filename) = open_file_dialog(window) {

        let file = File::open(&filename).unwrap();
        let len = fs::metadata(&filename).unwrap().len() as usize;

        ////Gestion du INI
        let mut ini_file = INI_FILE.lock().unwrap();
        ini_file.with_file_name(filename.clone());
        ini_file.set_extension("themistocle");
/*
        unsafe {
            ini_file.ini_path = filename.clone();
            let ini_file_to_pass = ini_file.ini_path.clone();
            utils::set_and_write_kv(conf, &ini_file_to_pass, "Fichier".to_owned(), "file_path".to_owned(), filename.to_str().unwrap().to_owned());
        }
*/
        conf.with_section(Some("Fichier".to_owned())).set("file_name".to_owned(), filename.to_str().unwrap());
        conf.write_to_file(ini_file.to_str().unwrap()).unwrap();

        /////////////

        let reader = PcapReader::new(BufReader::new(file)).unwrap();
        let mut pcap = PCAP.write().expect("global PCAP var unreadable.");
        *pcap = reader.collect();

        let mut str = String::with_capacity(len*2);
        for packet in &*pcap {
            utils::bytes_to_string(&mut str, &packet.data);
            str.push('\n');
        }

        return Some(str);
    }

    None
}