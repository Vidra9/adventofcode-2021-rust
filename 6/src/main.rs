use std::fs;

const SIMULATION_DURATION : u64 = 18;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines : Vec<&str> = file_contents.lines().collect();

    solve_part1(lines.clone());
    solve_part2(lines.clone());
}

fn solve_part1(lines : Vec<&str>) {
    let fishes: Vec<usize> = lines[0]
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();
    let max_day:usize = 9;
    let mut counter = vec![0;max_day];
    for fish in fishes {
        counter[fish]+=1;
    }
    let days:usize = 256;
    for d in 0..days {
        counter[(7+d)%max_day] += counter[d%max_day];
    }
    println!("{:?}", counter.iter().sum::<usize>());
}

fn solve_part2(lines : Vec<&str>) {
    let mut result = 0;
    
    println!("part 2: {}", result);
}