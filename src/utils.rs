use gdk::RGBA;
use rand::{Rng, thread_rng};
use std::path::PathBuf;
use ini::Ini;
use cast::i32;
use gtk::{Builder, Button, Dialog, FileChooserDialog, FileFilter, Label, MenuItem, TextBuffer, TextTag, TextView, Window};

static NIBBLE_STR: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];




pub struct INI_FILE{
    ini_path: PathBuf,
}

impl INI_FILE{
    pub fn new() -> INI_FILE{
        INI_FILE{
            ini_path: PathBuf::from("/"),
        }
    }

}




pub fn bytes_to_string(str: &mut String, bytes: &[u8]) {

    for &byte in bytes {

        let left = (byte >> 4) as usize;
        let right = (byte & 0x0F) as usize;

        str.push(NIBBLE_STR[left]);
        str.push(NIBBLE_STR[right]);
    }
}

pub fn random_color()-> RGBA{
    RGBA{
        red:   thread_rng().gen_range(0.0, 1.0),
        green: thread_rng().gen_range(0.0, 1.0),
        blue:  thread_rng().gen_range(0.0, 1.0),
        alpha: 1.0,
    }
}

pub fn set_and_write_kv(conf: &mut Ini, ini_file: &PathBuf,section_name: String, k: String, v: String ){

    conf.with_section(Some(section_name.to_owned())).set(k, v);
    conf.write_to_file(ini_file.to_str().unwrap()).unwrap();

}

pub fn count_word(text_buffer: &TextBuffer, motif: &str )-> Option<i32> {


    let (mut text_start,text_stop) = text_buffer.get_bounds();
    let taille_motif = i32(motif.clone().len()).unwrap();
    let mut end_word = text_start.clone();
    end_word.set_offset(taille_motif);


    let mut count=0;
    while(text_stop != end_word){
        let chaine_test =&text_start.get_text(&end_word).unwrap();
        let current_offset = text_start.get_offset();

        if chaine_test.starts_with("\n"){
            text_start.set_offset(current_offset + 1);
            end_word = text_start.clone();
            end_word.set_offset(current_offset+1 +taille_motif);
            continue;
        }else if chaine_test.contains("\n") {
            text_start.set_offset(current_offset + 2);
            end_word = text_start.clone();
            end_word.set_offset(current_offset +2 +taille_motif);
            continue;
        }

        if motif.eq(chaine_test) {
            ////on compte
            count = count+1;
            //////
            let current_offset = text_start.get_offset();
            text_start.set_offset(current_offset +(taille_motif));
            end_word = text_start.clone();
            end_word.set_offset(taille_motif);
        }else {
            //println!("motif pas vu");
            let current_offset = text_start.get_offset();
            text_start.set_offset(current_offset +2);
            end_word = text_start.clone();
            end_word.set_offset(current_offset +2+taille_motif);

        }
    }
    return Some(count);
}
