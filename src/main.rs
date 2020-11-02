use std::io;
use std::path::Path;
use std::env;
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use std::io::{BufWriter, Write, BufReader};
use bincode::{serialize_into, deserialize_from};


#[derive(Serialize, Deserialize)]
struct Bookmark {   
    url: String,
    description: String,
}


fn main() {
    // check whether file exists and create new if it doesn't
    if Path::new("bookmarks.bin").exists() == false {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("bookmarks.bin")
            .unwrap();
        let storage: Vec<Bookmark> = Vec::new();
        let mut writer = BufWriter::new(file);
        serialize_into(&mut writer, &storage).unwrap();
    }

    // take arguments and proceed with parsing
    let input: Vec<String> = env::args().collect();
    
    match input.len() {
        1 => println!("missing arguments"),
        
        2 => match &input[1][..] {
            "list" => println!("{}", list()),
            "help" => println!("{}", help()),
            "add" => println!("{}", add()),
            _ => println!("unknown command '{}'", input[1]),
        }
        
        3 => match &input[1][..] {
            "find" => println!("{}", find(&input[2])),
            "del" => println!("{}", del(&input[2])),
            _ => println!("unknown command '{} {}'", input[1], input[2]),
        }
        
        _ => println!("too many arguments"),
    }
}


fn help() -> String {
    "\n  help, list, add, del <index>, find <substring>\n".to_string()
}


fn list() -> String {
    let file = OpenOptions::new()
        .read(true)
        .open("bookmarks.bin")
        .unwrap();

    let mut reader = BufReader::new(file);
    let storage: Vec<Bookmark> = deserialize_from(&mut reader).unwrap();
    
    if storage.is_empty() {
        "bookmarks empty".to_string()
    } else {
        println!();
        for (i, bookmark) in storage.iter().enumerate() {
            println!("  {}. {}", i+1, bookmark.description);
            println!("  {}\n", bookmark.url);
        }
        "ok".to_string()
    }
}


fn add() -> String {
    // handle input
    let mut url = String::new();
    print!("url: ");    
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut url).unwrap();
    
    let mut description = String::new();
    print!("description: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut description).unwrap();

    let bookmark = Bookmark{
        url: url.trim().to_string(),
        description: description.trim().to_string(),
    };

    // deserialize
    let file = OpenOptions::new()
        .read(true)
        .open("bookmarks.bin")
        .unwrap();

    let mut reader = BufReader::new(&file);
    let mut storage: Vec<Bookmark> = deserialize_from(&mut reader).unwrap();
    storage.push(bookmark);

    // serialize
    let file = OpenOptions::new()
        .write(true)
        .open("bookmarks.bin")
        .unwrap();

    let mut writer = BufWriter::new(&file);
    serialize_into(&mut writer, &storage).unwrap();

    "ok".to_string()
}


fn del(index: &String) -> String {
    let i: usize = index.parse().unwrap();
    
    let file = OpenOptions::new()
        .read(true)
        .open("bookmarks.bin")
        .unwrap();

    let mut reader = BufReader::new(file);
    let mut storage: Vec<Bookmark> = deserialize_from(&mut reader).unwrap();
        
    if storage.is_empty() {
        "bookmarks empty".to_string()

    } else if i > storage.len() || i < 1 {
        "index out of range".to_string()

    } else {
        storage.remove(i-1);
        let file = OpenOptions::new()
            .write(true)
            .open("bookmarks.bin")
            .unwrap();

        let mut writer = BufWriter::new(file);
        serialize_into(&mut writer, &storage).unwrap();
        "ok".to_string()
    }
}


///// FIND BOOKMARK /////
fn find(_substring: &String) -> String {
    "find".to_string()    
}


