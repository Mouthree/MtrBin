use std::fs::read;
use std::process::exit;
use std::{io, process};
use ansi_term::Style;
use sled;
use std::result::Result;
use rustyline::error::ReadlineError; 
use rustyline::DefaultEditor;

pub fn cd_tree() {

}

pub fn print_tree() {

}

pub fn export_Excel() {
    
}

pub fn do_away_with(v: &str) -> Option<String>{
    if !(v.starts_with("[") && v.ends_with("]")) {
        println!("The variable format should be [xxx]");
        return None;
    }
    return v.strip_prefix('[').map(|s| s.to_string()).unwrap().strip_suffix(']').map(|s| s.to_string())
}