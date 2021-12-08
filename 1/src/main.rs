use std::fs;

fn main() {
    println!("reading file");

    let mut result = 0;

    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");
    let mut lines = file_contents.lines();
    let line : String = lines.next().unwrap().to_string();
    let mut previous : u32 = line.trim().parse().expect("Line not a number");
    loop {
        let next_line = lines.next();
        if next_line == Option::None {
            break;
        }
        let line : String = next_line.unwrap().to_string();
        let next : u32 = match line.trim().parse() {Ok(num) => num, Err(_) => break,};

        if next > previous {
            result += 1;
        }
        previous = next;
    }

    println!("result: {}", result);
}
