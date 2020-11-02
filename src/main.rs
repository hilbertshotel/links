use std::io;
use std::path::Path;
use std::env;
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use std::io::{BufWriter, Write, BufReader};
use bincode::{serialize_into, deserialize_from};


#[derive(Serialize, Deserialize, Debug)]
struct Bookmark {   
    url: String,
    description: String,
}


fn main() {
    check_for_file("bookmarks.bin");    
    let input: Vec<String> = env::args().collect();
    
    let output = match input.len() {
        1 => "missing arguments".to_string(),
        
        2 => match &input[1][..] {
            "list" => list(),
            "help" => help(),
            "add" =>  add(),
            _ => format!("unknown command '{}'", input[1]),
        }
        
        3 => match &input[1][..] {
            "find" => find(&input[2]),
            "del" => del(&input[2]),
            _ => format!("unknown command '{} {}'", input[1], input[2]),
        }
        
        _ => "too many arguments".to_string(),
    };
    
    if !output.is_empty() {
        println!("{}", output);
    }
}


// COMMANDS //
fn help() -> String {
    "\n  help, list, add, del <index>, find <substring>\n".to_string()
}


fn list() -> String {
    let bookmark_list: Vec<Bookmark> = deserialize_list("bookmarks.bin"); 

    if bookmark_list.is_empty() {
        "empty".to_string()
    } else {
        println!();
        for (index, bookmark) in bookmark_list.iter().enumerate() {
            println!("  {}. {}\n  {}\n", index+1, bookmark.description, bookmark.url);
            //println!("  {}\n", bookmark.url);
        }
        "".to_string()
    }
}


fn add() -> String {
    let bookmark = Bookmark{
        url: bookmark_input("url"),
        description: bookmark_input("description"),
    };

    let mut bookmark_list = deserialize_list("bookmarks.bin");
    bookmark_list.push(bookmark);
    serialize_list( bookmark_list, "bookmarks.bin");

    "".to_string()
}


fn del(index: &String) -> String {
    let index: usize = index.parse().unwrap();
    let mut bookmark_list = deserialize_list("bookmarks.bin");

    if index > bookmark_list.len() || index < 1 {
        "index out of range".to_string()
    } else {
        bookmark_list.remove(index-1);
        serialize_list(bookmark_list, "bookmarks.bin");
        "".to_string()
    }
}


fn find(substring: &String) -> String {
    let bookmark_list: Vec<Bookmark> = deserialize_list("bookmarks.bin");
   
    let mut search_result: Vec<Bookmark> = Vec::new();
    for bookmark in bookmark_list {
        if bookmark.description.contains(substring) {
            search_result.push(bookmark);
        }
    }

    if search_result.is_empty() {
        "nothing found".to_string()
    } else {
        println!();
        for (index, bookmark) in search_result.iter().enumerate() {
            println!("  {}. {}\n  {}\n", index+1, bookmark.description, bookmark.url);    
        }
        "".to_string()
    }
}


// UTILITY FUNCTIONS //
fn bookmark_input(prompt: &str) -> String {
    let mut var = String::new();
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut var).unwrap();
    var.trim().to_string()
}


fn deserialize_list(filename: &str) -> Vec<Bookmark> {
    let file = OpenOptions::new()
        .read(true)
        .open(filename)
        .unwrap();
    let mut reader = BufReader::new(file);
    let bookmark_list: Vec<Bookmark> = deserialize_from(&mut reader).unwrap();
    bookmark_list
}


fn serialize_list(bookmark_list: Vec<Bookmark>, filename: &str) {
    let file = OpenOptions::new()
        .write(true)
        .open(filename)
        .unwrap();
    let mut writer = BufWriter::new(file);
    serialize_into(&mut writer, &bookmark_list).unwrap();
}

fn check_for_file(filename: &str) {
    if !Path::new(filename).exists() {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();
        let bookmark_list: Vec<Bookmark> = Vec::new();
        let mut writer = BufWriter::new(file);
        serialize_into(&mut writer, &bookmark_list).unwrap();
    }
}

