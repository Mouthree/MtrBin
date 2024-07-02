use std::fs::read;
use std::process::exit;
use std::{io, process};
use ansi_term::Style;
use sled;
use std::result::Result;
use rustyline::error::ReadlineError; 
use rustyline::DefaultEditor;
use chrono::{Datelike, Local, Timelike};
use super::about_other;
pub fn create_box(name: &str){
    let name = about_other::do_away_with(name);
    if let None = name{
        return;
    }
    let name = name.unwrap();
    let bold = Style::new().bold();
    let db = sled::open("MtrBin.db").unwrap();
    let name_save = format!("box[{}]", name);
    let tree = db.open_tree(name_save).unwrap();
    if tree.is_empty() {
        println!("input comment: ");
        let mut comment: String = String::new();
        io::stdin().read_line(&mut comment).expect("err");
        if comment.trim().is_empty() {
            comment = String::from("Not set comment");
        }
        tree.insert("comment", comment.as_str());
        println!("successfully create box: {}", bold.paint(name));
    }else {
        let mut t: String = String::new();
        println!("exist box: {}", bold.paint(name));
        if let Some(value) = tree.get("comment").unwrap(){
            println!("comment is: {}", bold.paint(String::from_utf8(value.to_vec()).unwrap()));
        }
        //Todo: 添加-bd指令同款显示效果,但是就显示一条
        println!("The first three values are");
        
        for i in 1..=3{
            if let Some(value) = tree.get(format!("{}", i)).unwrap(){
                let t = String::from_utf8(value.to_vec()).unwrap();
                println!("{}. [{}]", i, t);
            }
        }
    }
}

pub fn create_BOM(name: &str) {
    let name = about_other::do_away_with(name);
    if let None = name{
        return;
    }
    let name = name.unwrap();
    let bold = Style::new().bold();
    let db = sled::open("MtrBin.db").unwrap();
    let name_save = format!("BOM[{}]", name);
    let tree = db.open_tree(name_save).unwrap();
    if tree.is_empty() {
        println!("input comment: ");
        let mut comment: String = String::new();
        io::stdin().read_line(&mut comment).expect("err");
        if comment.trim().is_empty() {
            comment = String::from("Not set comment");
        }
        tree.insert("comment", comment.as_str());
        let now = Local::now();
        let create_time = format!("{}-{}-{}-{}-{}", now.year(), now.month(), now.day(), now.hour(), now.minute());
        tree.insert("time", create_time.as_str());
        println!("successfully create BOM: {}, in {}", bold.paint(name), bold.paint(create_time));
    }else {
        println!("exist BOM: {}", bold.paint(name));
        if let Some(value) = tree.get("comment").unwrap(){
            println!("comment is: {}", bold.paint(String::from_utf8(value.to_vec()).unwrap()));
        }
        if let Some(value) = tree.get("time").unwrap() {
            println!("build time is: {}", bold.paint(String::from_utf8(value.to_vec()).unwrap()));
        }
        println!("The first three values are");
        for i in 1..=3{
            if let Some(value) = tree.get(format!("{}", i)).unwrap(){
                let t = String::from_utf8(value.to_vec()).unwrap();
                println!("{}. [{}]", i, t);
            }
        }
    }
}