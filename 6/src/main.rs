use std::fs;

const SIMULATION_DURATION : u8 = 80;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines : Vec<&str> = file_contents.lines().collect();

    solve_part1(lines.clone());
    solve_part2(lines.clone());
}

fn solve_part1(lines : Vec<&str>) {
}

fn solve_part2(lines : Vec<&str>) {
}