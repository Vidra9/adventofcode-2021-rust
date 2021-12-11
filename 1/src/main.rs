use std::fs;

const UNDEFINED_NUMBER: i32 = -1;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines : Vec<&str> = file_contents.lines().collect();

    solve_part1(lines.clone());
    solve_part2(lines.clone());
}

fn solve_part1(lines : Vec<&str>) {
    let mut result = 0;
    let mut previous : i32 = -1;

    for next_line in lines {
        let line : String = next_line.to_string();
        let next : i32 = match line.trim().parse() {Ok(num) => num, Err(_) => break,};

        if previous > -1 && next > previous {
            result += 1;
        }

        previous = next;
    }

    println!("part 1: {}", result);
}

fn solve_part2(lines : Vec<&str>) {
    let mut result = 0;

    let list : Vec<i32> = lines.iter().map(|x| x.trim().parse().expect("panik")).collect();
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

    println!("part 2: {}", result);
}
