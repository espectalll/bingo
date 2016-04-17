extern crate core;
extern crate rand;
extern crate gtk;

use std::cell::RefCell;
use rand::distributions::{IndependentSample, Range};
use gtk::prelude::*;
use gtk::{Builder, Window, Label, Button, ListBox};

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn main() {
    if gtk::init().is_err() {
        println!("Ayy lkek y u no Gtk+ m8");
    }
    
    let used: Vec<i32> = Vec::new();
    let between = Range::new(1, 91);
    
    let used_h = RefCell::new(used);
    
    let builder = Builder::new_from_file("bingo.glade");
    let window: Window = builder.get_object("AppWindow").unwrap();
    let button: Button = builder.get_object("NextButton").unwrap();
    let label: Label = builder.get_object("GuessedLabel").unwrap();
    let list_box: ListBox = builder.get_object("UsedListBox").unwrap();
    
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    button.connect_clicked(
        clone!(used_h, between, label, button => move |_| {
        
        let mut guessed: i32;
        let mut used = used_h.borrow_mut();
        let mut rng = rand::thread_rng();
        
        if used.len() < 90 {
            loop {
                guessed = between.ind_sample(&mut rng);
                if !used.contains(&guessed) { break; }
            }
        
            used.push(guessed);
            used.sort();
        
            label.set_label(&guessed.to_string());
         }
         if used.len() == 90 {
            /* for i in used {
                println!("{}", i);
            } */
            button.set_sensitive(false);
         }
    }));
    
    window.show();
    gtk::main();
}
