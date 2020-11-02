// kbm :: kolu's bookmarks terminal tool

use std::io;
use std::path::Path;
use std::fs::File;
use std::env;
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use std::io::{BufWriter, Write, BufReader, BufRead};
use bincode::{serialize_into, deserialize_from};


///// BOOKMARK STRUCT /////
#[derive(Serialize, Deserialize, Debug)]
struct Bookmark {   
    url: String,
    description: String,
}


///// MAIN /////
fn main() {

    // check whether file exists and create new if it doesn't
    if Path::new("bookmarks.bin").exists() == false {
        File::create("bookmarks.bin").unwrap();
    }

    // take arguments and proceed with parsing
    let input: Vec<String> = env::args().collect();
    
    match input.len() {
        // no arguments passed
        1 => println!("missing arguments"),
        
        // one argument passed
        2 => match &input[1][..] {
            "list" => list(),
            "help" => help(),
            "add" => add(),
            _ => println!("unknown command '{}'", input[1]),
        }
        
        // two arguments passed
        3 => match &input[1][..] {
            "find" => find(&input[2]),
            "del" => del(&input[2]),
            _ => println!("unknown command '{} {}'", input[1], input[2]),
        }
        
        // more than two arguments passed
        _ => println!("too many arguments"),
    }
}


///// LIST BOOKMARKS /////
fn list() {
    let file = OpenOptions::new()
        .read(true)
        .open("bookmarks.bin")
        .unwrap();

    let mut reader = BufReader::new(&file);
    let storage: Vec<Bookmark> = deserialize_from(&mut reader).unwrap();
    println!("{:?}", storage);
}


///// HELP //////
fn help() {
    println!("help");
}


///// ADD BOOKMARK /////
fn add() {
    // ask for 'url' input
    let mut url = String::new();
    print!("url: ");
    
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut url)
        .expect("stdin error");
    
    // ask for 'description' input
    let mut description = String::new();
    print!("description: ");

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut description)
        .expect("stdin error");

    // save both inputs into Bookmark struct
    let bookmark = Bookmark{
        url: url.trim().to_string(),
        description: description.trim().to_string(),
    };

    // open file for reading
    let file = OpenOptions::new()
        .read(true)
        .open("bookmarks.bin")
        .unwrap();

    // check whether file is empty
    // if so - create new storage vector - else deserialize existing
    let mut reader = BufReader::new(file);
    let buffer = reader.fill_buf().unwrap();
    let mut storage: Vec<Bookmark> = if buffer.is_empty() {
        Vec::new()
    } else {
        deserialize_from(&mut reader).unwrap()
    };
    storage.push(bookmark);

    // open file for writing
    let file = OpenOptions::new()
        .write(true)
        .open("bookmarks.bin")
        .unwrap();

    // serialize new bookmark and write to file
    let mut writer = BufWriter::new(file);
    serialize_into(&mut writer, &storage).unwrap();
}


///// FIND BOOKMARK /////
fn find(input: &String) {
    println!("{}", input);
}


///// DELETE BOOKMARK /////
fn del(input: &String) {
    println!("{}", input);
}

