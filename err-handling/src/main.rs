use std::fs::File;
use std::io::{self, Read};

fn main() {
    let maybe_username = read_username_from_file();

    if let Ok(username)  = maybe_username  {
        println!("eto yon {}", username)
    } else {
        println!("tae tae na mo")
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("helloa.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    println!("dumaan ba or nireturn agad?");
    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}