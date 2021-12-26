use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines : Vec<&str> = file_contents.lines().collect();

    // solve_part1(lines.clone());
    solve_part2(lines.clone());
}

fn solve_part1(lines : Vec<&str>) {
    let mut crabs: Vec<usize> = lines[0]
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();
    crabs.sort();
    let len = crabs.len();
    let x = if len % 2 == 0 {(crabs[len/2] + crabs[len/2-1])/2} else {crabs[len/2+1]};
    let mut result = 0;
    for c in crabs {
        result += (c as i32 - x as i32).abs();
    }
    println!("{}", result);
}

fn solve_part2(lines : Vec<&str>) {
    let mut result = 0;
    let mut crabs: Vec<usize> = lines[0]
    .split(",")
    .map(|i| i.parse().unwrap())
    .collect();
    let x = crabs.iter().sum::<usize>() as f32 / crabs.len() as f32;
    let x = x.round();
    for item in crabs {
        result += sum_from_one((item as i32 - x as i32).abs() as usize);
    }
    println!("{}", result);
}

fn sum_from_one( n: usize) -> usize {
    (1 .. n+1).fold(0, |a, b| a + b)
}
