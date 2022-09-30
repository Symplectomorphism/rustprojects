use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
// use std::io;
// use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
