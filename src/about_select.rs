use std::fmt::DebugList;
use std::fs::read;
use std::process::exit;
use std::{io, process};
use ansi_term::Colour;
use sled;
use std::result::Result;
use rustyline::error::ReadlineError; 
use rustyline::DefaultEditor;
use tabled::{settings::Style, Table, Tabled};

pub fn show_box(){
    let db = sled::open("MtrBin.db").unwrap();
    let tree_names = db.tree_names();
    for i in tree_names {
        let t = String::from_utf8(i.to_vec()).unwrap();
        if &t[0..=2] == "box"{
            println!("{}", &t[3..]);
        }
    }
}

pub fn show_BOM(){
    let db = sled::open("MtrBin.db").unwrap();
    let tree_names = db.tree_names();
    for i in tree_names {
        let t = String::from_utf8(i.to_vec()).unwrap();
        if  &t[0..=2] == "BOM"{
            println!("{}", &t[3..]);
        }
    }
}

pub fn clear_tree(){
    let db = sled::open("MtrBin.db").unwrap();
    let tree_names = db.tree_names();
    for i in tree_names {
        let t = String::from_utf8(i.to_vec()).unwrap();
        db.drop_tree(t);
    }
}


//show in detail
#[derive(Tabled)]
struct Vendor {
    name: String,
    comment: String,
    #[tabled(display_with = "display_distribution")]
    inside: Distribution,
}

impl Vendor {
    fn new(name: String, comment: String, inside: Distribution) -> Self {
        Self {
            name,
            comment,
            inside,
        }
    }
}

fn display_distribution(d: &Distribution) -> String {
    Table::new([d]).with(Style::extended()).to_string()
}

#[derive(Tabled)]
struct Distribution {
    one: String,
    two: String,
    three: String,
    four: String,
}

impl Distribution {
    fn display_based_on(o: &Option<&'static str>) -> String {
        match o {
            &Some(s) => s.into(),
            None => "Independent".into(),
        }
    }
}

impl Distribution {
    fn new(
        one: String,
        two: String,
        three: String,
        four: String,
    ) -> Self {
        Self {
            one,
            two,
            three,
            four,
        }
    }
}


pub fn show_boxd() {
    let mut data: Vec<Vendor> = Vec::new();
    let db = sled::open("MtrBin.db").unwrap();
    let tree_names = db.tree_names();
    for i in tree_names {
        let t = String::from_utf8(i.to_vec()).unwrap();
        if  &t[0..=2] == "box"{
            let mut d: Distribution = Distribution::new("".to_string(), "".to_string(), "".to_string(), "".to_string());
            let tree = db.open_tree(t.clone()).unwrap();
            
            match tree.get("one").unwrap(){
                Some(v) => d.one = String::from_utf8(tree.get("one").unwrap().unwrap().to_vec()).unwrap(),
                None => d.one = "".trim().to_string()
            }

            match tree.get("two").unwrap(){
                Some(v) => d.one = String::from_utf8(tree.get("two").unwrap().unwrap().to_vec()).unwrap(),
                None => d.one = "".trim().to_string()
            }

            match tree.get("three").unwrap(){
                Some(v) => d.one = String::from_utf8(tree.get("three").unwrap().unwrap().to_vec()).unwrap(),
                None => d.one = "".trim().to_string()
            }

            match tree.get("four").unwrap(){
                Some(v) => d.one = String::from_utf8(tree.get("four").unwrap().unwrap().to_vec()).unwrap(),
                None => d.one = "".trim().to_string()
            }
            data.push(Vendor::new(t[3..].strip_prefix('[').map(|s| s.to_string()).unwrap().strip_suffix(']').map(|s| s.to_string()).unwrap().trim().to_string(), String::from_utf8(tree.get("comment").unwrap().unwrap().to_vec()).unwrap().trim().to_string(), d));
        }
    }
    let table = Table::new(data).with(Style::modern()).to_string();
    println!("{}", table);
}


pub fn show_BOMd() {
    let mut data: Vec<Vendor> = Vec::new();
    let db = sled::open("MtrBin.db").unwrap();
    let tree_names = db.tree_names();
    for i in tree_names {
        let t = String::from_utf8(i.to_vec()).unwrap();
        if  &t[0..=2] == "BOM"{
            let mut d: Distribution = Distribution::new("".to_string(), "".to_string(), "".to_string(), "".to_string());
            let tree = db.open_tree(t.clone()).unwrap();
            
            match tree.get("one").unwrap(){
                Some(v) => d.one = String::from_utf8(tree.get("one").unwrap().unwrap().to_vec()).unwrap(),
                None => d.one = "".trim().to_string()
            }

            match tree.get("two").unwrap(){
                Some(v) => d.one = String::from_utf8(tree.get("two").unwrap().unwrap().to_vec()).unwrap(),
                None => d.one = "".trim().to_string()
            }

            match tree.get("three").unwrap(){
                Some(v) => d.one = String::from_utf8(tree.get("three").unwrap().unwrap().to_vec()).unwrap(),
                None => d.one = "".trim().to_string()
            }

            match tree.get("four").unwrap(){
                Some(v) => d.one = String::from_utf8(tree.get("four").unwrap().unwrap().to_vec()).unwrap(),
                None => d.one = "".trim().to_string()
            }
            data.push(Vendor::new(t[3..].strip_prefix('[').map(|s| s.to_string()).unwrap().strip_suffix(']').map(|s| s.to_string()).unwrap().trim().to_string(), String::from_utf8(tree.get("comment").unwrap().unwrap().to_vec()).unwrap().trim().to_string(), d));
        }
    }
    let table = Table::new(data).with(Style::modern()).to_string();
    println!("{}", table);
}   