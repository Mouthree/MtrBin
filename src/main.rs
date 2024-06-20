#![allow(unused)]
#![allow(non_snake_case)]
use std::fs::read;
use std::process::exit;
use std::{io, process};
use ansi_term::Style;
use sled;
use std::result::Result;
use rustyline::error::ReadlineError; 
use rustyline::DefaultEditor;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    // 注意下面的注释是三个斜杠!!!
    /// use which method 
    #[arg(short, long)]
    method: Option<String>,
    
    /// Optional name to call
    name: Option<String>,
}
fn main() -> rustyline::Result<()> {


    let mut rl = DefaultEditor::new()?;

    //这个是给命令行文字加粗的
    let bold = Style::new().bold();
    println!("███╗   ███╗████████╗██████╗ ██████╗ ██╗███╗   ██╗
████╗ ████║╚══██╔══╝██╔══██╗██╔══██╗██║████╗  ██║
██╔████╔██║   ██║   ██████╔╝██████╔╝██║██╔██╗ ██║
██║╚██╔╝██║   ██║   ██╔══██╗██╔══██╗██║██║╚██╗██║
██║ ╚═╝ ██║   ██║   ██║  ██║██████╔╝██║██║ ╚████║
╚═╝     ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═════╝ ╚═╝╚═╝  ╚═══╝");
    loop {
        let readline = rl.readline(">> ");
        if let "quit" | "q" | "exit" | "close" | "over" = readline.as_ref().unwrap().as_str(){
            exit(0);
        }
        if let "MOUTHREE" = readline.as_ref().unwrap().as_str(){
            println!("███╗   ███╗ ██████╗ ██╗   ██╗████████╗██╗  ██╗██████╗ ███████╗███████╗
████╗ ████║██╔═══██╗██║   ██║╚══██╔══╝██║  ██║██╔══██╗██╔════╝██╔════╝
██╔████╔██║██║   ██║██║   ██║   ██║   ███████║██████╔╝█████╗  █████╗  
██║╚██╔╝██║██║   ██║██║   ██║   ██║   ██╔══██║██╔══██╗██╔══╝  ██╔══╝  
██║ ╚═╝ ██║╚██████╔╝╚██████╔╝   ██║   ██║  ██║██║  ██║███████╗███████╗
╚═╝     ╚═╝ ╚═════╝  ╚═════╝    ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝
                                                                     ");
        }
        let mut line_in: String = String::new();
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                line_in = line;
            },
            Err(_) => {
                println!("Err");
            }
        }

    }
}

fn create_box(name: &str, comment: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bold = Style::new().bold();
    let db = sled::open("MtrBin.db")?;
    let tree = db.open_tree(name)?;
    if tree.is_empty() {
        tree.insert("comment", comment);
        println!("successfully create box: {}", bold.paint(name));
    }else {
        let mut t: String = String::new();
        println!("exist box: {}", bold.paint(name));
        if let Some(value) = tree.get("commend").unwrap(){
            t = String::from_utf8(value.to_vec()).unwrap();
        }
        println!("comment is: {}", bold.paint(t));
        println!("the first five values are");
    }
    Ok(())
}

fn create_BOM() {

}

fn show_box() -> Result<(), Box<dyn std::error::Error>> {
    let db = sled::open("MtrBin.db")?;
    let tree_names = db.tree_names();
    for i in tree_names {
        println!("{:?}", i);
    }
    Ok(())
}