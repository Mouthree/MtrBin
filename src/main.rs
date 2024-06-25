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

mod about_create;
mod about_add;
mod about_change;
mod about_delete;
mod about_other;
mod about_select;

mod sudo;

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
        let words = shellwords::split(line_in.as_str());
        let words: Vec<&str> = words.as_ref().unwrap().iter().map(|s| s.as_str()).collect();
        match words.as_slice(){
            //创建box
            //Todo: 实现变量都要加[],在每个函数中进行一个检测
            ["create" | "c", "-box" | "-b", name] => about_create::create_box(name),
            //创建BOM
            ["create" | "c", "-bom" | "-m", name] => about_create::create_BOM(name),
            //列出所有box
            //Todo: 在后面显示comment
            ["ls" | "l", "-box" | "-b"] => about_select::show_box(),
            //列出所有BOM
            //Todo: 在后面显示comment
            ["ls" | "l", "-bom" | "-m"] => about_select::show_BOM(),
            //列出详细内容
            ["ls" | "l", "-boxd" | "-bd"] => about_select::show_boxd(),
            //列出详细内容
            ["ls" | "l", "-bomd" | "-md"] => about_select::show_BOMd(),
            //向指定box中添加零件
            ["add" | "a", "-box" | "-b", name, "-n" | "-num", num, part] => about_add::add_part_in(name, part, num),
            //向指定box中添加零件,同时可以添加标签
            ["add" | "a", "-box" | "-b", name, "-n" | "-num", num, part, "-t" | "-tag", tag @ ..] => about_add::add_part_in_with_comment(name, part, num, tag),
            //查询指定box
            //查询指定BOM
            //以下仅为测试功能
            //删除所有tree
            ["mouthree", "nb"] => sudo::clear_tree(),
            //查看所有tree
            ["mouthree", "look"] => sudo::show_tree(),
            //当什么都没输入时不作操作
            [""] => println!(""),
            //未定义语句
            _ => println!("no command"),
        }
    }
}