use std::io;
use std::fs;
use std::path::Path;
use std::env;
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use std::io::{BufWriter, Write, BufReader};
use bincode::{serialize_into, deserialize_from};

const PATH: &str = "/home/kolu/code/rust/bin/bookmarks.bin";

#[derive(Serialize, Deserialize, Debug)]
struct Bookmark {   
    url: String,
    description: String,
}


fn main() {
    check_for_file(PATH);    
    let input: Vec<String> = env::args().collect();
    
    let output = match input.len() {
        1 => help(),
        
        2 => match &input[1][..] {
            "list" => list(),
            "add" =>  add(),
            "clear!" => clear(),
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
    "\n  help, list, add, del <index>, find <substring>, clear!\n".to_string()
}


fn list() -> String {
    let bookmark_list: Vec<Bookmark> = deserialize_list(PATH); 
    print_bookmarks(bookmark_list, "empty")
}


fn add() -> String {
    let bookmark = Bookmark{
        url: bookmark_input("url"),
        description: bookmark_input("description"),
    };

    let mut bookmark_list = deserialize_list(PATH);
    bookmark_list.push(bookmark);
    serialize_list(bookmark_list, PATH);

    "".to_string()
}


fn del(index: &String) -> String {
    let check = index.chars().all(char::is_numeric);
    
    if check == false {
        "index must be a number".to_string()
    } else {
        let index: usize = index.parse().expect("error: index parsing");
        let mut bookmark_list = deserialize_list(PATH);

        if index > bookmark_list.len() || index < 1 {
            "index out of range".to_string()
        } else {
            bookmark_list.remove(index-1);
            serialize_list(bookmark_list, PATH);
            "".to_string()
        }
    }
}


fn find(substring: &String) -> String {
    let bookmark_list: Vec<Bookmark> = deserialize_list(PATH);
   
    let mut search_result: Vec<Bookmark> = Vec::new();
    for bookmark in bookmark_list {
        if bookmark.description.contains(substring) {
            search_result.push(bookmark);
        }
    }
    print_bookmarks(search_result, "nothing found")
}


fn clear() -> String {
    fs::remove_file(PATH).expect("error: remove file");
    "".to_string()
}


// UTILITY FUNCTIONS //
fn bookmark_input(prompt: &str) -> String {
    let mut var = String::new();
    print!("{}: ", prompt);
    io::stdout().flush().expect("error: stdout flush");
    io::stdin().read_line(&mut var).expect("error: stdin read line");
    var.trim().to_string()
}


fn deserialize_list(filename: &str) -> Vec<Bookmark> {
    let file = OpenOptions::new()
        .read(true)
        .open(filename)
        .expect("error: read file");
    
    let mut reader = BufReader::new(file);
    let bookmark_list: Vec<Bookmark> = deserialize_from(&mut reader)
        .expect("error: deserialize from");
    
    bookmark_list
}


fn serialize_list(bookmark_list: Vec<Bookmark>, filename: &str) {
    let file = OpenOptions::new()
        .write(true)
        .open(filename)
        .expect("error: write file");

    let mut writer = BufWriter::new(file);
    serialize_into(&mut writer, &bookmark_list)
        .expect("error: serialize into");
}


fn check_for_file(filename: &str) {
    if !Path::new(filename).exists() {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)
            .expect("error: create file");

        let bookmark_list: Vec<Bookmark> = Vec::new();
        let mut writer = BufWriter::new(file);
        serialize_into(&mut writer, &bookmark_list)
            .expect("error: serialize into");
    }
}


fn print_bookmarks(container: Vec<Bookmark>, error: &str) -> String {
    if container.is_empty() {
        error.to_string()
    } else {
        println!();
        for (index, bookmark) in container.iter().enumerate() {
            println!("  {}. {}\n  {}\n ", index+1, bookmark.description, bookmark.url);
         }
         "".to_string()
    }
}

