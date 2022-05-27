#![allow(unused)]
use std::fs::File;
use std::io::{self, Read};

fn main() {

    fn read_username_from_file() -> Result<String, io::Error> {
        /*let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }*/

        // usign '?' operator after a Result
        // returns the Error or assigns the Ok value to variable
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;
        
        Ok(s)
        
        // even shorter version:
        // File::read_to_string("hello.txt") 
        // returns the Result<String, io::Error>
    }
    // will return te Ok result and panic if Err
    // let r = read_username_from_file().unwrap();
    
    // same, but will show defined msg if Err
    let r = read_username_from_file().expect("o ao ler arquivo hello.txt");
    println!("{}", r);
}
