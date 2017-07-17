use gtk::prelude::*;
use gtk::{Builder, Button, ColorButton, ComboBoxText, Dialog, Entry, FileChooserDialog, FileFilter, Label, MessageDialog, MenuItem,  TextBuffer, TextIter, TextTag, TextTagTable,  TextView, Window};
use gdk::RGBA;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::time::Duration;

use std::rc::Rc;
use std::cell::RefCell;

use utils;
use cast::i32;

use ui::main_window::INI_FILE;

use std::{thread,time};

static EMPTY_RGBA: RGBA = RGBA { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 };


//pub fn init(text_view : &mut TextView) {

pub fn init(text_view : Rc<TextView>) {

    let glade = include_str!("../test.glade");
    let builder = Builder::new_from_string(glade);
    let apply_button: Button = builder.get_object("window_find_button_apply").unwrap();
    let button_load_tag_file: Button = builder.get_object("button_load_tag_file").unwrap();
    let search_motif_entry: Entry = builder.get_object("window_find_entry").unwrap();
    let choosed_fg_color: ColorButton = builder.get_object("button_choose_fg").unwrap();
    let choosed_bg_color: ColorButton = builder.get_object("button_choose_fg").unwrap();
    let motif_name_textbox : ComboBoxText = builder.get_object("motif_name_textbox").unwrap();
    let modify_motif_action: ComboBoxText = builder.get_object("modify_motif_action").unwrap();
    let button_modify_a_tag: Button = builder.get_object("button_modify_a_tag").unwrap();
    let choosed_modify_fg_color: ColorButton = builder.get_object("button_choose_modify_fg").unwrap();
    let choosed_modify_bg_color: ColorButton = builder.get_object("button_choose_modify_bg").unwrap();

    let window: Window = builder.get_object("window_find").unwrap();
    window.show_all();

    //println!("plip");


    let text_view_shared = Rc::new(text_view);
    let text_view_shared_copy = text_view_shared.clone();

    //let choosed_fg_color_shared = Rc::from(choosed_fg_color);
    //let choosed_fg_color_shared_copy = choosed_fg_color_shared.clone();
    //let choosed_bg_color_shared = Rc::from(choosed_bg_color);
    //let choosed_bg_color_shared_copy = choosed_bg_color_shared.clone();
    let motif_name_textbox_shared = Rc::from(motif_name_textbox);
    let motif_name_textbox_shared_copy = motif_name_textbox_shared.clone();


    apply_button.connect_clicked(move |_|{

        //println!("dans la recherche + color");
        let search_motif = search_motif_entry.get_buffer().get_text();
        let text_buffer = text_view_shared_copy.get_buffer().unwrap();
        let (text_start,text_stop) = text_buffer.get_bounds();

        let mut tag_table = text_buffer.get_tag_table().unwrap();
        let mut tag_motif_bg = TextTag::new((search_motif.clone() + "_bg").as_ref());
        let mut tag_motif_fg = TextTag::new((search_motif.clone() + "_fg").as_ref());
        tag_table.add(&tag_motif_bg);
        tag_table.add(&tag_motif_fg);

        if choosed_bg_color.get_rgba() == EMPTY_RGBA{
            tag_motif_bg.set_property_background_rgba(Some(&utils::random_color()));
        }else{
            tag_motif_bg.set_property_background_rgba(Some(&choosed_bg_color.get_rgba()));
        }
        if choosed_fg_color.get_rgba() == EMPTY_RGBA {

           // println!("plop");
            tag_motif_fg.set_property_foreground_rgba(Some(&utils::random_color()));
        }else{
            tag_motif_fg.set_property_foreground_rgba(Some(&choosed_fg_color.get_rgba()));
        }

        search(&text_buffer,&search_motif,&tag_motif_bg, &tag_motif_fg);

        motif_name_textbox_shared_copy.append_text(&search_motif);
        motif_name_textbox_shared_copy.set_active(0);
        search_motif_entry.set_text("");


    });

    //////NOT WORKING FOR NOW !!!!
    let text_view_shared_copy = text_view_shared.clone();
    button_load_tag_file.connect_clicked(move |_|{

    });


    ////////LET'S RECOLOR //////////
    let text_view_shared_copy = text_view_shared.clone();
    let motif_name_textbox_shared_copy = motif_name_textbox_shared.clone();
    button_modify_a_tag.connect_clicked(move |_|{

        let text_buffer = text_view_shared_copy.get_buffer().unwrap();
        let mut tag_table = text_buffer.get_tag_table().unwrap();
        let action = modify_motif_action.get_active_text().unwrap();

        match action.as_ref(){
            "Remove" =>{
                let name_motif = motif_name_textbox_shared_copy.clone().get_active_text().unwrap();

                /////SUPR/////
                let text_tag = tag_table.lookup((name_motif.clone() + "_bg").as_ref()).unwrap();
                tag_table.remove(&text_tag);
                let text_tag = tag_table.lookup((name_motif.clone() + "_fg").as_ref()).unwrap();
                tag_table.remove(&text_tag);
                motif_name_textbox_shared_copy.remove(motif_name_textbox_shared_copy.get_active());
            },
            "Recolor"=>{
                let name_motif = motif_name_textbox_shared_copy.clone().get_active_text().unwrap();
                let text_tag = tag_table.lookup((name_motif.clone() + "_bg").as_ref()).unwrap();
                tag_table.remove(&text_tag);
                let text_tag = tag_table.lookup((name_motif.clone() + "_fg").as_ref()).unwrap();
                tag_table.remove(&text_tag);

                //////Recolor//////

                let mut tag_motif_bg = TextTag::new((motif_name_textbox_shared_copy.get_active_text().unwrap() + "_bg").as_ref());
                let mut tag_motif_fg = TextTag::new((motif_name_textbox_shared_copy.get_active_text().unwrap() + "_fg").as_ref());
                tag_table.add(&tag_motif_bg);
                tag_table.add(&tag_motif_fg);
                tag_motif_bg.set_property_background_rgba(Some(&choosed_modify_bg_color.get_rgba()));
                tag_motif_fg.set_property_foreground_rgba(Some(&choosed_modify_fg_color.get_rgba()));
                //println!("{:?}", tag_motif_fg);

                search(&text_buffer,&name_motif,&tag_motif_bg, &tag_motif_fg);

            },
            "Count"=>{

                let name_motif = motif_name_textbox_shared_copy.clone().get_active_text().unwrap();

                if let Some(nb) = utils::count_word(&text_buffer,&name_motif){
                    let message_box : MessageDialog = builder.get_object("message_box").unwrap();
                    let message_box_msg : Label = builder.get_object("message_box_msg").unwrap();
                    let msg = "Nombre d'occurence : ".to_owned() + &nb.to_string();
                    message_box_msg.set_label(&msg);
                    message_box.show_all();

                }
            },
            _=> println!("action imprévue :{}", action),
        }

    });


}

fn search(text_buffer: &TextBuffer, motif: &str, tag_bg: &TextTag, tag_fg: &TextTag) {

    let begin = 0;
    let (mut text_start,text_stop) = text_buffer.get_bounds();
    let taille_motif = i32(motif.clone().len()).unwrap();
    let mut end_word = text_start.clone();
    end_word.set_offset(taille_motif);

    let mut i=0;
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

            ////// applique le tag
            //println!("motif vu");
            text_buffer.apply_tag(&tag_bg,&text_start, &end_word);
            text_buffer.apply_tag(&tag_fg,&text_start, &end_word);

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

}