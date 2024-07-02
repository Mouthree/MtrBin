use std::collections::HashMap;
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
            
            match tree.get("1").unwrap(){
                Some(v) => {
                    let t = tree.get("1").unwrap().unwrap();
                    let t = String::from_utf8(t.to_vec()).unwrap();
                    let t: HashMap<String, String> = serde_json::from_str(&t).unwrap();
                    let t = t.get("name").unwrap();
                    d.one = t.clone();
                },
                None => d.one = "".trim().to_string()
            }

            match tree.get("2").unwrap(){
                Some(v) => {
                    let t = tree.get("2").unwrap().unwrap();
                    let t = String::from_utf8(t.to_vec()).unwrap();
                    let t: HashMap<String, String> = serde_json::from_str(&t).unwrap();
                    let t = t.get("name").unwrap();
                    d.two = t.clone();
                },
                None => d.two = "".trim().to_string()
            }

            match tree.get("3").unwrap(){
                Some(v) => {
                    let t = tree.get("3").unwrap().unwrap();
                    let t = String::from_utf8(t.to_vec()).unwrap();
                    let t: HashMap<String, String> = serde_json::from_str(&t).unwrap();
                    let t = t.get("name").unwrap();
                    d.three = t.clone();
                },
                None => d.three = "".trim().to_string()
            }

            match tree.get("4").unwrap(){
                Some(v) => {
                    let t = tree.get("4").unwrap().unwrap();
                    let t = String::from_utf8(t.to_vec()).unwrap();
                    let t: HashMap<String, String> = serde_json::from_str(&t).unwrap();
                    let t = t.get("name").unwrap();
                    d.four = t.clone();
                },
                None => d.four = "".trim().to_string()
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
            
            match tree.get("1").unwrap(){
                Some(v) => {
                    let t = tree.get("1").unwrap().unwrap();
                    let t = String::from_utf8(t.to_vec()).unwrap();
                    let t: HashMap<String, String> = serde_json::from_str(&t).unwrap();
                    let t = t.get("name").unwrap();
                    d.one = t.clone();
                },
                None => d.one = "".trim().to_string()
            }

            match tree.get("2").unwrap(){
                Some(v) => {
                    let t = tree.get("2").unwrap().unwrap();
                    let t = String::from_utf8(t.to_vec()).unwrap();
                    let t: HashMap<String, String> = serde_json::from_str(&t).unwrap();
                    let t = t.get("name").unwrap();
                    d.two = t.clone();
                },
                None => d.two = "".trim().to_string()
            }

            match tree.get("3").unwrap(){
                Some(v) => {
                    let t = tree.get("3").unwrap().unwrap();
                    let t = String::from_utf8(t.to_vec()).unwrap();
                    let t: HashMap<String, String> = serde_json::from_str(&t).unwrap();
                    let t = t.get("name").unwrap();
                    d.three = t.clone();
                },
                None => d.three = "".trim().to_string()
            }

            match tree.get("4").unwrap(){
                Some(v) => {
                    let t = tree.get("4").unwrap().unwrap();
                    let t = String::from_utf8(t.to_vec()).unwrap();
                    let t: HashMap<String, String> = serde_json::from_str(&t).unwrap();
                    let t = t.get("name").unwrap();
                    d.four = t.clone();
                },
                None => d.four = "".trim().to_string()
            }
            data.push(Vendor::new(t[3..].strip_prefix('[').map(|s| s.to_string()).unwrap().strip_suffix(']').map(|s| s.to_string()).unwrap().trim().to_string(), String::from_utf8(tree.get("comment").unwrap().unwrap().to_vec()).unwrap().trim().to_string(), d));
        }
    }
    let table = Table::new(data).with(Style::modern()).to_string();
    println!("{}", table);
}   


//Todo: 找指定的零件都在哪个盒子里的几号
pub fn select_part() {
    
}