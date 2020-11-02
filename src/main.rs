// kbm :: kolu's bookmarks terminal tool
use std::env;
use bincode::{serialize_into, deserialize_from};
use serde::{Serialize, Deserialize};


///// BOOKMARK STRUCT /////
#[derive(Serialize, Deserialize, Debug)]
struct Bookmark {   
    url: String,
    description: String,
}


///// MAIN /////
fn main() {
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
    println!("list");
}


///// HELP //////
fn help() {
    println!("help");
}


///// ADD BOOKMARK /////
fn add() {
    use std::io;
    use std::io::{BufWriter, Write, BufReader};
    use std::fs::OpenOptions;

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


    let mut storage_vector: Vec<Bookmark> = Vec::new();
    storage_vector.push(bookmark);
    // read file and check if empty
    // if empty create new vector, append bookmark and serialize
    // else load content - deserialize and append bookmark
    // serialize and write

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("bookmarks.bin");


    let mut writer = BufWriter::new(file.unwrap());
    serialize_into(&mut writer, &storage_vector).unwrap();


    let file = OpenOptions::new()
        .read(true)
        .open("bookmarks.bin");
    
    let mut reader = BufReader::new(file.unwrap());
    let decoded: Vec<Bookmark> = deserialize_from(&mut reader).unwrap();

    println!("{:?}", decoded);

}


///// FIND BOOKMARK /////
fn find(input: &String) {
    println!("{}", input);
}


///// DELETE BOOKMARK /////
fn del(input: &String) {
    println!("{}", input);
}

