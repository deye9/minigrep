use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = read_file(filename);

    println!("With text:\n{}", contents);
}

fn read_file(filename: &std::string::String) -> std::string::String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}