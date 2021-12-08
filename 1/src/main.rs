use std::fs;

fn main() {
    println!("reading file");

    let file = fs::read_to_string("input.txt")
        .expect("Something went wrong");
    
    println!("File contents: {}", file);
}
