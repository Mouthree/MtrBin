use std::collections::{HashMap, LinkedList};
use super::about_other;
use ansi_term::Colour;
use ansi_term::Style;
use serde::{Serialize, Deserialize};

use tabled::builder::Builder;
//name是box名,part是零件名,num是存在哪个零件里
pub fn add_part_in_box(name: &str, part: &str, num: &str) {
    //对三个变量依次判断是否为 [xxx] 格式
    let name = about_other::do_away_with(name);
    if let None = name{
        return;
    }
    let part = about_other::do_away_with(part);
    if let None = part{
        return;
    }
    let num = about_other::do_away_with(num);
    if let None = num{
        return;
    }
    //当输入的num不是整数的时候退出
    match num.as_ref().unwrap().parse::<i32>() {
        Ok(_) => (),
        Err(_) => {
            println!("The value of num should be an integer");
            return;
        }
    }

    let db = sled::open("MtrBin.db").unwrap();
    let name_save = format!("box[{}]", name.as_ref().unwrap());
    let tree = db.open_tree(name_save).unwrap();

    //Todo: 对box进行一个有没有的判断,如果没有则让用户选择是否创建
    if tree.is_empty() {
        println!("no box");
    }else {
        if tree.contains_key(num.as_ref().unwrap().as_str()).unwrap() {
            println!("Num is not empty");
            let t = tree.get(num.as_ref().unwrap()).unwrap().unwrap();
            let t = String::from_utf8(t.to_vec()).unwrap();
            let t: HashMap<String, String> = serde_json::from_str(&t).unwrap();
            let t = t.get("name").unwrap();
            let mut builder = Builder::new();
            builder.push_record([num.unwrap(), t.clone()]);
            println!("{}", builder.build()
            .with(tabled::settings::Style::ascii_rounded())
            .to_string());
            //Todo: 输入replace替换,输入q退出
        }else {
            let mut a: HashMap<String, String> = HashMap::new();
            a.insert(String::from("name"), part.clone().unwrap());
            let serialized = serde_json::to_string(&a).unwrap();
            tree.insert(num.as_ref().unwrap(), serialized.as_str());
            println!("Pare: {} be deposited into {}", Style::new().bold().paint(part.unwrap()), Style::new().bold().paint(num.unwrap()));
        }
    }

}

//创建的时候就带一些biaoqian
pub fn add_part_in_with_comment(name: &str, part: &str, num: &str, tag: &[&str]) {
    println!("name:{}", name);
    println!("part:{}", part);
    println!("tag:{:?}", tag);
}


//给指定的box中的指定元素添加标签
pub fn add_tag_for() {
    
}

