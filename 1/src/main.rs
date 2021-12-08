use std::fs;

fn main() {
    let mut result = 0;

    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines = file_contents.lines();
    let mut previous : i32 = -1;

    for next_line in lines {
        let line : String = next_line.to_string();
        let next : i32 = match line.trim().parse() {Ok(num) => num, Err(_) => break,};

        if previous > -1 && next > previous {
            result += 1;
        }

        previous = next;
    }

    println!("result: {}", result);
}
