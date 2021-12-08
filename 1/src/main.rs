use std::fs;

const UNDEFINED_NUMBER: i32 = -1;

fn main() {
    let mut result = 0;

    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let list : Vec<i32> = file_contents.lines().map(|x| x.trim().parse().expect("panik")).collect();
    let mut previous : i32 = -1;

    let mut i = 0;
    let num_of_lines = list.len();

    while i < num_of_lines - 2 {
        let next = list[i] + list[i+1] + list[i+2];

        if previous != UNDEFINED_NUMBER && next > previous {
            result += 1;
        }

        previous = next;
        i += 1;
    }

    println!("result: {}", result);
}
