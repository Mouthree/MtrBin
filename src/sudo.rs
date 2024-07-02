

pub fn clear_tree(){
    let db = sled::open("MtrBin.db").unwrap();
    let tree_names = db.tree_names();
    for i in tree_names {
        let t = String::from_utf8(i.to_vec()).unwrap();
        if t != String::from("box[mouthree]") {
            db.drop_tree(t);
        }
    }
    println!("All files have been deleted")
}

pub fn show_tree(){
    let db = sled::open("MtrBin.db").unwrap();
    let tree_names = db.tree_names();
    for i in tree_names {
        let t = String::from_utf8(i.to_vec()).unwrap();
        println!("{t}");
    }
}