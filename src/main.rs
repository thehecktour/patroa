use std::fs;

fn main() {
    let pathway = "README.md";

    match fs::read_to_string(pathway){
        Ok(content) => println!("Readme.md's content: {}", content),
        Err(err) => println!("File's problem: {}", err),
    }
}
