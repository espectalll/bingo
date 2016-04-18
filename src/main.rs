extern crate core;
extern crate libc;
extern crate rand;
extern crate gtk;

use std::cell::RefCell;
use std::ffi::CString;
use libc::system;
use rand::distributions::{IndependentSample, Range};
use gtk::prelude::*;
use gtk::{
    Builder, Window, Label, Button, Entry,
    ListStore, CellRendererText, TreeView, TreeViewColumn };

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
    
    let used: Vec<i32> = Vec::new(); /* Should consider implementing
                                        my own GtkTreeModel class instead */
    let between = Range::new(1, 91);
    
    let used_h = RefCell::new(used);
    
    let builder = Builder::new_from_file("bingo.glade");
    let window: Window = builder.get_object("AppWindow").unwrap();
    let button: Button = builder.get_object("NextButton").unwrap();
    let label: Label = builder.get_object("GuessedLabel").unwrap();
    let list: ListStore = builder.get_object("UsedListStore").unwrap();
    let list_view: TreeView = builder.get_object("UsedListView").unwrap();
    let search: Entry = builder.get_object("SearchEntry").unwrap();
    let column = TreeViewColumn::new();
    let cell_r = CellRendererText::new();

    column.pack_start(&cell_r, true);
    column.add_attribute(&cell_r, "text", 0);
    list_view.append_column(&column);
    list_view.set_search_entry(Some(&search));
    
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    button.connect_clicked(
        clone!(used_h, between, label, button, list => move |_| {
        
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
            
            list.insert_with_values(
                None, &[0], &[&ToValue::to_value(&guessed.to_string())]);
            
            label.set_label(&guessed.to_string());
            
            /* This is obviously a temporal, crappy patch
               Must be replaced with Festival via C API */
            unsafe {
                let cmd = CString::new(
                    format!("espeak -v spanish {}", guessed.to_string()))
                    .unwrap().as_ptr();
                system(cmd);
            }
         }
         if used.len() == 90 {
            button.set_sensitive(false);
         }
    }));
    
    window.show();
    gtk::main();
}
