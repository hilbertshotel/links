use std::io;
use std::path::Path;
use std::fs::File;
use std::env;
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use std::io::{BufWriter, Write, BufReader, BufRead};
use bincode::{serialize_into, deserialize_from};


#[derive(Serialize, Deserialize)]
struct Bookmark {   
    url: String,
    description: String,
}


fn main() {
    // check whether file exists and create new if it doesn't
    if Path::new("bookmarks.bin").exists() == false {
        File::create("bookmarks.bin").unwrap();
    }

    // take arguments and proceed with parsing
    let input: Vec<String> = env::args().collect();
    
    match input.len() {
        1 => println!("missing arguments"),
        
        2 => match &input[1][..] {
            "list" => list(),
            "help" => help(),
            "add" => add(),
            _ => println!("unknown command '{}'", input[1]),
        }
        
        3 => match &input[1][..] {
            "find" => find(&input[2]),
            "del" => del(&input[2]),
            _ => println!("unknown command '{} {}'", input[1], input[2]),
        }
        
        _ => println!("too many arguments"),
    }
}


fn help() {
    println!("\n  help, list, add, del <index>, find <substring>\n");
}


fn list() {
    let file = OpenOptions::new()
        .read(true)
        .open("bookmarks.bin")
        .unwrap();

    let mut reader = BufReader::new(&file);
    let buffer = reader.fill_buf().unwrap();

    if buffer.is_empty() {
        println!("bookmarks empty");
    } else {
        let storage: Vec<Bookmark> = deserialize_from(&mut reader).unwrap();

        if storage.is_empty() {
            println!("bookmarks empty");
        } else {
            println!();
            for (i, bookmark) in storage.iter().enumerate() {
                println!("  {}. {}", i+1, bookmark.description);
                println!("  {}\n", bookmark.url);
            }
        }
    }
}


fn add() {
    // handle input
    let mut url = String::new();
    print!("url: ");
    
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut url)
        .expect("stdin error");
    
    let mut description = String::new();
    print!("description: ");

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut description)
        .expect("stdin error");

    let bookmark = Bookmark{
        url: url.trim().to_string(),
        description: description.trim().to_string(),
    };

    // deserialize
    let file = OpenOptions::new()
        .read(true)
        .open("bookmarks.bin")
        .unwrap();

    let mut reader = BufReader::new(file);
    let buffer = reader.fill_buf().unwrap();
    
    let mut storage: Vec<Bookmark> = if buffer.is_empty() {
        Vec::new()
    } else {
        deserialize_from(&mut reader).unwrap()
    };
    
    storage.push(bookmark);

    // serialize
    let file = OpenOptions::new()
        .write(true)
        .open("bookmarks.bin")
        .unwrap();

    let mut writer = BufWriter::new(file);
    serialize_into(&mut writer, &storage).unwrap();
}


fn del(index: &String) {
    let i: usize = index.parse().unwrap();
    
    let file = OpenOptions::new()
        .read(true)
        .open("bookmarks.bin")
        .unwrap();

    let mut reader = BufReader::new(file);
    let buffer = reader.fill_buf().unwrap();

    if buffer.is_empty() {
        println!("bookmarks empty");
    } else {
        let mut storage: Vec<Bookmark> = deserialize_from(&mut reader).unwrap();
        
        if storage.is_empty() {
            println!("bookmarks empty");
        } else if i > storage.len() || i < 1 {
            println!("index out of range");
        } else {
            storage.remove(i-1);
            let file = OpenOptions::new()
                .write(true)
                .open("bookmarks.bin")
                .unwrap();

            let mut writer = BufWriter::new(file);
            serialize_into(&mut writer, &storage).unwrap();
        }
    }
}


///// FIND BOOKMARK /////
fn find(substring: &String) {
    println!("{}", substring);
    
}


